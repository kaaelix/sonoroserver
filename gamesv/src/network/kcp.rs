use std::collections::VecDeque;

pub const IKCP_OVERHEAD: usize = 24;
const IKCP_RTO_NDL: i32 = 30;
const IKCP_RTO_MIN: i32 = 100;
const IKCP_RTO_DEF: i32 = 200;
const IKCP_RTO_MAX: i32 = 60000;
const IKCP_CMD_PUSH: u8 = 81;
const IKCP_CMD_ACK: u8 = 82;
const IKCP_CMD_WASK: u8 = 83;
const IKCP_CMD_WINS: u8 = 84;
const IKCP_ASK_SEND: u32 = 1;
const IKCP_ASK_TELL: u32 = 2;
const IKCP_WND_SND: u32 = 64;
const IKCP_WND_RCV: u32 = 512;
const IKCP_MTU_DEF: usize = 1400;
const IKCP_INTERVAL: u32 = 100;
const IKCP_DEADLINK: u32 = 20;
const IKCP_THRESH_INIT: u32 = 2;
const IKCP_THRESH_MIN: u32 = 2;
const IKCP_PROBE_INIT: u32 = 7000;
const IKCP_PROBE_LIMIT: u32 = 120000;
const IKCP_FASTACK_LIMIT: u32 = 5;
const IKCP_FASTACK_CONSERVE: bool = true;

#[inline]
fn timediff(later: u32, earlier: u32) -> i32 {
    later.wrapping_sub(earlier) as i32
}

#[derive(Debug, Clone)]
struct Segment {
    conv: u32,
    cmd: u8,
    frg: u8,
    wnd: u16,
    ts: u32,
    sn: u32,
    una: u32,
    resendts: u32,
    rto: u32,
    fastack: u32,
    xmit: u32,
    data: Vec<u8>,
}

impl Segment {
    fn encode_header(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.conv.to_le_bytes());
        out.push(self.cmd);
        out.push(self.frg);
        out.extend_from_slice(&self.wnd.to_le_bytes());
        out.extend_from_slice(&self.ts.to_le_bytes());
        out.extend_from_slice(&self.sn.to_le_bytes());
        out.extend_from_slice(&self.una.to_le_bytes());
        out.extend_from_slice(&(self.data.len() as u32).to_le_bytes());
    }
}

#[derive(Debug, Clone, Copy)]
struct AckEntry {
    sn: u32,
    ts: u32,
}

#[derive(Debug)]
pub struct Kcp {
    pub conv: u32,
    pub mtu: usize,
    pub mss: usize,
    pub state: i32,
    pub snd_una: u32,
    pub snd_nxt: u32,
    pub rcv_nxt: u32,
    pub ssthresh: u32,
    pub rx_rttval: i32,
    pub rx_srtt: i32,
    pub rx_rto: i32,
    pub rx_minrto: i32,
    pub snd_wnd: u32,
    pub rcv_wnd: u32,
    pub rmt_wnd: u32,
    pub cwnd: u32,
    pub probe: u32,
    pub current: u32,
    pub interval: u32,
    pub ts_flush: u32,
    pub xmit: u32,
    pub nodelay: u32,
    pub updated: bool,
    pub ts_probe: u32,
    pub probe_wait: u32,
    pub dead_link: u32,
    pub incr: u32,
    snd_queue: VecDeque<Segment>,
    rcv_queue: VecDeque<Segment>,
    snd_buf: VecDeque<Segment>,
    rcv_buf: VecDeque<Segment>,
    acklist: Vec<AckEntry>,
    tx_buf: Vec<u8>,
    fastresend: u32,
    fastlimit: u32,
    nocwnd: bool,
    pub stream: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KcpError {
    EmptyQueue,
    IncompletePacket,
    TooManyFragments,
    InvalidInput,
    InvalidMtu,
}

impl Kcp {
    pub fn new(conv: u32) -> Self {
        let mtu = IKCP_MTU_DEF;
        Self {
            conv,
            mtu,
            mss: mtu - IKCP_OVERHEAD,
            state: 0,
            snd_una: 0,
            snd_nxt: 0,
            rcv_nxt: 0,
            ssthresh: IKCP_THRESH_INIT,
            rx_rttval: 0,
            rx_srtt: 0,
            rx_rto: IKCP_RTO_DEF,
            rx_minrto: IKCP_RTO_MIN,
            snd_wnd: IKCP_WND_SND,
            rcv_wnd: IKCP_WND_RCV,
            rmt_wnd: IKCP_WND_RCV,
            cwnd: 0,
            probe: 0,
            current: 0,
            interval: IKCP_INTERVAL,
            ts_flush: IKCP_INTERVAL,
            xmit: 0,
            nodelay: 0,
            updated: false,
            ts_probe: 0,
            probe_wait: 0,
            dead_link: IKCP_DEADLINK,
            incr: 0,
            snd_queue: VecDeque::new(),
            rcv_queue: VecDeque::new(),
            snd_buf: VecDeque::new(),
            rcv_buf: VecDeque::new(),
            acklist: Vec::new(),
            tx_buf: Vec::with_capacity(mtu),
            fastresend: 0,
            fastlimit: IKCP_FASTACK_LIMIT,
            nocwnd: false,
            stream: false,
        }
    }

    pub fn set_nodelay(&mut self, nodelay: u32, interval: u32, resend: u32, nc: bool) {
        self.nodelay = nodelay;
        self.rx_minrto = if nodelay > 0 {
            IKCP_RTO_NDL
        } else {
            IKCP_RTO_MIN
        };
        self.interval = interval.clamp(5, 5000);
        self.fastresend = resend;
        self.nocwnd = nc;
    }

    pub fn set_wnd_size(&mut self, sndwnd: u32, rcvwnd: u32) {
        if sndwnd > 0 {
            self.snd_wnd = sndwnd;
        }
        if rcvwnd > 0 {
            self.rcv_wnd = rcvwnd.max(IKCP_WND_RCV);
        }
    }

    #[allow(dead_code)]
    pub fn set_mtu(&mut self, mtu: usize) -> Result<(), KcpError> {
        if mtu < 50 || mtu < IKCP_OVERHEAD {
            return Err(KcpError::InvalidMtu);
        }
        self.mtu = mtu;
        self.mss = mtu - IKCP_OVERHEAD;
        Ok(())
    }

    pub fn peek_size(&self) -> Result<usize, KcpError> {
        let front = self.rcv_queue.front().ok_or(KcpError::EmptyQueue)?;
        if front.frg == 0 {
            return Ok(front.data.len());
        }
        if self.rcv_queue.len() < usize::from(front.frg) + 1 {
            return Err(KcpError::IncompletePacket);
        }
        let mut length = 0usize;
        for seg in &self.rcv_queue {
            length += seg.data.len();
            if seg.frg == 0 {
                break;
            }
        }
        Ok(length)
    }

    pub fn recv_vec(&mut self) -> Result<Vec<u8>, KcpError> {
        let peeksize = self.peek_size()?;
        let mut out = Vec::with_capacity(peeksize);
        let recover = self.rcv_queue.len() >= self.rcv_wnd as usize;

        while let Some(seg) = self.rcv_queue.pop_front() {
            let frg = seg.frg;
            out.extend_from_slice(&seg.data);
            if frg == 0 {
                break;
            }
        }

        while let Some(front) = self.rcv_buf.front() {
            if front.sn == self.rcv_nxt && self.rcv_queue.len() < self.rcv_wnd as usize {
                let seg = self.rcv_buf.pop_front().unwrap();
                self.rcv_queue.push_back(seg);
                self.rcv_nxt = self.rcv_nxt.wrapping_add(1);
            } else {
                break;
            }
        }

        if self.rcv_queue.len() < self.rcv_wnd as usize && recover {
            self.probe |= IKCP_ASK_TELL;
        }
        Ok(out)
    }

    pub fn send(&mut self, mut buf: &[u8]) -> Result<usize, KcpError> {
        if buf.is_empty() {
            return Ok(0);
        }
        let mut sent = 0usize;

        if self.stream
            && let Some(last) = self.snd_queue.back_mut()
            && last.data.len() < self.mss
        {
            let cap = self.mss - last.data.len();
            let extend = buf.len().min(cap);
            last.data.extend_from_slice(&buf[..extend]);
            last.frg = 0;
            buf = &buf[extend..];
            sent += extend;
            if buf.is_empty() {
                return Ok(sent);
            }
        }

        let count = buf.len().div_ceil(self.mss);
        if count >= IKCP_WND_RCV as usize {
            if self.stream && sent > 0 {
                return Ok(sent);
            }
            return Err(KcpError::TooManyFragments);
        }

        for i in 0..count {
            let size = buf.len().min(self.mss);
            let frg = if self.stream { 0 } else { (count - i - 1) as u8 };
            self.snd_queue.push_back(Segment {
                conv: self.conv,
                cmd: 0,
                frg,
                wnd: 0,
                ts: 0,
                sn: 0,
                una: 0,
                resendts: 0,
                rto: 0,
                fastack: 0,
                xmit: 0,
                data: buf[..size].to_vec(),
            });
            buf = &buf[size..];
            sent += size;
        }
        Ok(sent)
    }

    fn update_ack(&mut self, rtt: i32) {
        if self.rx_srtt == 0 {
            self.rx_srtt = rtt;
            self.rx_rttval = rtt / 2;
        } else {
            let delta = (rtt - self.rx_srtt).abs();
            self.rx_rttval = (3 * self.rx_rttval + delta) / 4;
            self.rx_srtt = ((7 * self.rx_srtt + rtt) / 8).max(1);
        }
        let rto = self.rx_srtt + (self.interval.max(4 * (self.rx_rttval.max(0) as u32)) as i32);
        self.rx_rto = rto.clamp(self.rx_minrto, IKCP_RTO_MAX);
    }

    fn shrink_buf(&mut self) {
        self.snd_una = self
            .snd_buf
            .front()
            .map_or(self.snd_nxt, |seg| seg.sn);
    }

    fn parse_ack(&mut self, sn: u32) {
        if timediff(sn, self.snd_una) < 0 || timediff(sn, self.snd_nxt) >= 0 {
            return;
        }
        if let Some(pos) = self.snd_buf.iter().position(|seg| seg.sn == sn) {
            self.snd_buf.remove(pos);
        }
    }

    fn parse_una(&mut self, una: u32) {
        while let Some(front) = self.snd_buf.front() {
            if timediff(una, front.sn) > 0 {
                self.snd_buf.pop_front();
            } else {
                break;
            }
        }
    }

    fn parse_fastack(&mut self, sn: u32, ts: u32) {
        if timediff(sn, self.snd_una) < 0 || timediff(sn, self.snd_nxt) >= 0 {
            return;
        }
        for seg in &mut self.snd_buf {
            if timediff(sn, seg.sn) < 0 {
                break;
            } else if sn != seg.sn && (!IKCP_FASTACK_CONSERVE || timediff(ts, seg.ts) >= 0) {
                seg.fastack += 1;
            }
        }
    }

    fn parse_data(&mut self, newseg: Segment) {
        let sn = newseg.sn;
        if timediff(sn, self.rcv_nxt.wrapping_add(self.rcv_wnd)) >= 0
            || timediff(sn, self.rcv_nxt) < 0
        {
            return;
        }

        let mut insert_idx = self.rcv_buf.len();
        let mut repeat = false;
        while insert_idx > 0 {
            let existing = &self.rcv_buf[insert_idx - 1];
            if existing.sn == sn {
                repeat = true;
                break;
            }
            if timediff(sn, existing.sn) > 0 {
                break;
            }
            insert_idx -= 1;
        }

        if !repeat {
            self.rcv_buf.insert(insert_idx, newseg);
        }

        while let Some(front) = self.rcv_buf.front() {
            if front.sn == self.rcv_nxt && self.rcv_queue.len() < self.rcv_wnd as usize {
                let seg = self.rcv_buf.pop_front().unwrap();
                self.rcv_queue.push_back(seg);
                self.rcv_nxt = self.rcv_nxt.wrapping_add(1);
            } else {
                break;
            }
        }
    }

    pub fn input(&mut self, mut data: &[u8]) -> Result<(), KcpError> {
        if data.len() < IKCP_OVERHEAD {
            return Err(KcpError::InvalidInput);
        }
        let prev_una = self.snd_una;
        let mut maxack = 0u32;
        let mut latest_ts = 0u32;
        let mut flag = false;

        while data.len() >= IKCP_OVERHEAD {
            let conv = u32::from_le_bytes(data[0..4].try_into().unwrap());
            if conv != self.conv {
                return Err(KcpError::InvalidInput);
            }
            let cmd = data[4];
            let frg = data[5];
            let wnd = u16::from_le_bytes(data[6..8].try_into().unwrap());
            let ts = u32::from_le_bytes(data[8..12].try_into().unwrap());
            let sn = u32::from_le_bytes(data[12..16].try_into().unwrap());
            let una = u32::from_le_bytes(data[16..20].try_into().unwrap());
            let len = u32::from_le_bytes(data[20..24].try_into().unwrap()) as usize;
            data = &data[IKCP_OVERHEAD..];

            if data.len() < len {
                return Err(KcpError::InvalidInput);
            }

            if !matches!(
                cmd,
                IKCP_CMD_PUSH | IKCP_CMD_ACK | IKCP_CMD_WASK | IKCP_CMD_WINS
            ) {
                return Err(KcpError::InvalidInput);
            }

            self.rmt_wnd = u32::from(wnd);
            self.parse_una(una);
            self.shrink_buf();

            match cmd {
                IKCP_CMD_ACK => {
                    let rtt = timediff(self.current, ts);
                    if rtt >= 0 {
                        self.update_ack(rtt);
                    }
                    self.parse_ack(sn);
                    self.shrink_buf();
                    if !flag {
                        flag = true;
                        maxack = sn;
                        latest_ts = ts;
                    } else if timediff(sn, maxack) > 0
                        && (!IKCP_FASTACK_CONSERVE || timediff(ts, latest_ts) > 0)
                    {
                        maxack = sn;
                        latest_ts = ts;
                    }
                }
                IKCP_CMD_PUSH => {
                    if timediff(sn, self.rcv_nxt.wrapping_add(self.rcv_wnd)) < 0 {
                        self.acklist.push(AckEntry { sn, ts });
                        if timediff(sn, self.rcv_nxt) >= 0 {
                            let seg = Segment {
                                conv,
                                cmd,
                                frg,
                                wnd,
                                ts,
                                sn,
                                una,
                                resendts: 0,
                                rto: 0,
                                fastack: 0,
                                xmit: 0,
                                data: data[..len].to_vec(),
                            };
                            self.parse_data(seg);
                        }
                    }
                }
                IKCP_CMD_WASK => {
                    self.probe |= IKCP_ASK_TELL;
                }
                IKCP_CMD_WINS => {}
                _ => unreachable!(),
            }

            data = &data[len..];
        }

        if flag {
            self.parse_fastack(maxack, latest_ts);
        }

        if timediff(self.snd_una, prev_una) > 0 && self.cwnd < self.rmt_wnd {
            let mss = self.mss as u32;
            if self.cwnd < self.ssthresh {
                self.cwnd += 1;
                self.incr += mss;
            } else {
                if self.incr < mss {
                    self.incr = mss;
                }
                self.incr += (mss * mss) / self.incr + (mss / 16);
                if (self.cwnd + 1) * mss <= self.incr {
                    self.cwnd = (self.incr + mss - 1) / mss.max(1);
                }
            }
            if self.cwnd > self.rmt_wnd {
                self.cwnd = self.rmt_wnd;
                self.incr = self.rmt_wnd * mss;
            }
        }

        Ok(())
    }

    pub fn wnd_unused(&self) -> u16 {
        let len = self.rcv_queue.len() as u32;
        if len < self.rcv_wnd {
            (self.rcv_wnd - len).min(u32::from(u16::MAX)) as u16
        } else {
            0
        }
    }

    pub fn flush(&mut self, output: &mut dyn FnMut(&[u8])) {
        if !self.updated {
            return;
        }
        let current = self.current;
        let wnd = self.wnd_unused();
        let mut buf = std::mem::take(&mut self.tx_buf);
        buf.clear();
        if buf.capacity() < self.mtu {
            buf.reserve(self.mtu - buf.capacity());
        }

        let mut ack_seg = Segment {
            conv: self.conv,
            cmd: IKCP_CMD_ACK,
            frg: 0,
            wnd,
            ts: 0,
            sn: 0,
            una: self.rcv_nxt,
            resendts: 0,
            rto: 0,
            fastack: 0,
            xmit: 0,
            data: Vec::new(),
        };

        for ack in self.acklist.drain(..) {
            if buf.len() + IKCP_OVERHEAD > self.mtu {
                output(&buf);
                buf.clear();
            }
            ack_seg.sn = ack.sn;
            ack_seg.ts = ack.ts;
            ack_seg.encode_header(&mut buf);
        }

        if self.rmt_wnd == 0 {
            if self.probe_wait == 0 {
                self.probe_wait = IKCP_PROBE_INIT;
                self.ts_probe = current.wrapping_add(self.probe_wait);
            } else if timediff(current, self.ts_probe) >= 0 {
                if self.probe_wait < IKCP_PROBE_INIT {
                    self.probe_wait = IKCP_PROBE_INIT;
                }
                self.probe_wait += self.probe_wait / 2;
                if self.probe_wait > IKCP_PROBE_LIMIT {
                    self.probe_wait = IKCP_PROBE_LIMIT;
                }
                self.ts_probe = current.wrapping_add(self.probe_wait);
                self.probe |= IKCP_ASK_SEND;
            }
        } else {
            self.ts_probe = 0;
            self.probe_wait = 0;
        }

        if (self.probe & IKCP_ASK_SEND) != 0 {
            ack_seg.cmd = IKCP_CMD_WASK;
            ack_seg.sn = 0;
            ack_seg.ts = 0;
            if buf.len() + IKCP_OVERHEAD > self.mtu {
                output(&buf);
                buf.clear();
            }
            ack_seg.encode_header(&mut buf);
        }

        if (self.probe & IKCP_ASK_TELL) != 0 {
            ack_seg.cmd = IKCP_CMD_WINS;
            ack_seg.sn = 0;
            ack_seg.ts = 0;
            if buf.len() + IKCP_OVERHEAD > self.mtu {
                output(&buf);
                buf.clear();
            }
            ack_seg.encode_header(&mut buf);
        }

        self.probe = 0;

        let mut cwnd = self.snd_wnd.min(self.rmt_wnd);
        if !self.nocwnd {
            cwnd = self.cwnd.min(cwnd);
        }

        while timediff(self.snd_nxt, self.snd_una.wrapping_add(cwnd)) < 0 {
            let Some(mut newseg) = self.snd_queue.pop_front() else {
                break;
            };
            newseg.conv = self.conv;
            newseg.cmd = IKCP_CMD_PUSH;
            newseg.wnd = wnd;
            newseg.ts = current;
            newseg.sn = self.snd_nxt;
            self.snd_nxt = self.snd_nxt.wrapping_add(1);
            newseg.una = self.rcv_nxt;
            newseg.resendts = current;
            newseg.rto = self.rx_rto.max(0) as u32;
            newseg.fastack = 0;
            newseg.xmit = 0;
            self.snd_buf.push_back(newseg);
        }

        let resent = if self.fastresend > 0 {
            self.fastresend
        } else {
            u32::MAX
        };
        let rtomin = if self.nodelay == 0 {
            (self.rx_rto.max(0) as u32) >> 3
        } else {
            0
        };

        let mut change = false;
        let mut lost = false;

        for segment in &mut self.snd_buf {
            let mut needsend = false;
            if segment.xmit == 0 {
                needsend = true;
                segment.xmit += 1;
                segment.rto = self.rx_rto.max(0) as u32;
                segment.resendts = current.wrapping_add(segment.rto).wrapping_add(rtomin);
            } else if timediff(current, segment.resendts) >= 0 {
                needsend = true;
                segment.xmit += 1;
                self.xmit += 1;
                if self.nodelay == 0 {
                    segment.rto = segment
                        .rto
                        .saturating_add(segment.rto.max(self.rx_rto.max(0) as u32));
                } else {
                    let step = if self.nodelay < 2 {
                        segment.rto
                    } else {
                        self.rx_rto.max(0) as u32
                    };
                    segment.rto = segment.rto.saturating_add(step / 2);
                }
                segment.resendts = current.wrapping_add(segment.rto);
                lost = true;
            } else if segment.fastack >= resent && (segment.xmit <= self.fastlimit || self.fastlimit == 0) {
                needsend = true;
                segment.xmit += 1;
                segment.fastack = 0;
                segment.resendts = current.wrapping_add(segment.rto);
                change = true;
            }

            if needsend {
                segment.ts = current;
                segment.wnd = wnd;
                segment.una = self.rcv_nxt;

                let need = IKCP_OVERHEAD + segment.data.len();
                if buf.len() + need > self.mtu && !buf.is_empty() {
                    output(&buf);
                    buf.clear();
                }

                segment.encode_header(&mut buf);
                if !segment.data.is_empty() {
                    buf.extend_from_slice(&segment.data);
                }
                if segment.xmit >= self.dead_link {
                    self.state = -1;
                }
            }
        }

        if !buf.is_empty() {
            output(&buf);
        }
        buf.clear();
        self.tx_buf = buf;

        if change {
            let inflight = self.snd_nxt.wrapping_sub(self.snd_una);
            self.ssthresh = (inflight / 2).max(IKCP_THRESH_MIN);
            self.cwnd = self.ssthresh + resent;
            self.incr = self.cwnd * (self.mss as u32);
        }

        if lost {
            self.ssthresh = (cwnd / 2).max(IKCP_THRESH_MIN);
            self.cwnd = 1;
            self.incr = self.mss as u32;
        }

        if self.cwnd < 1 {
            self.cwnd = 1;
            self.incr = self.mss as u32;
        }
    }

    pub fn update(&mut self, current: u32, output: &mut dyn FnMut(&[u8])) {
        self.current = current;
        if !self.updated {
            self.updated = true;
            self.ts_flush = self.current;
        }
        let mut slap = timediff(self.current, self.ts_flush);
        if !(-10000..10000).contains(&slap) {
            self.ts_flush = self.current;
            slap = 0;
        }
        if slap >= 0 {
            self.ts_flush = self.ts_flush.wrapping_add(self.interval);
            if timediff(self.current, self.ts_flush) >= 0 {
                self.ts_flush = self.current.wrapping_add(self.interval);
            }
            self.flush(output);
        }
    }

    pub fn check(&self, current: u32) -> u32 {
        if !self.updated {
            return current;
        }
        let mut ts_flush = self.ts_flush;
        if timediff(current, ts_flush).abs() >= 10000 {
            ts_flush = current;
        }
        if timediff(current, ts_flush) >= 0 {
            return current;
        }
        let tm_flush = timediff(ts_flush, current);
        let mut tm_packet = i32::MAX;
        for seg in &self.snd_buf {
            let diff = timediff(seg.resendts, current);
            if diff <= 0 {
                return current;
            }
            if diff < tm_packet {
                tm_packet = diff;
            }
        }
        let minimal = tm_packet.min(tm_flush).min(self.interval as i32).max(0) as u32;
        current.wrapping_add(minimal)
    }

    pub fn next_update_delay(&self, current: u32) -> u32 {
        timediff(self.check(current), current).max(0) as u32
    }

    pub fn is_dead(&self) -> bool {
        self.state != 0
    }

    pub fn wait_snd(&self) -> usize {
        self.snd_buf.len() + self.snd_queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kcp_roundtrips_packet_over_simulated_wire() {
        let mut a = Kcp::new(1);
        let mut b = Kcp::new(1);
        a.set_nodelay(1, 10, 2, true);
        b.set_nodelay(1, 10, 2, true);

        let payload = b"hello kcp reliable transport test payload";
        assert_eq!(a.send(payload).unwrap(), payload.len());

        let mut wire_a = Vec::new();
        a.update(10, &mut |pkt| wire_a.push(pkt.to_vec()));
        assert!(!wire_a.is_empty());

        for pkt in wire_a {
            b.input(&pkt).unwrap();
        }
        assert_eq!(b.recv_vec().unwrap(), payload);
    }
}
