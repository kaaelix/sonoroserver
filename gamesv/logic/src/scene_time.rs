const MAX_FINITE_DILATION: f64 = 1_000_000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeInfo {
    pub flow_timestamp_at_anchor: i64,
    pub monotonic_anchor_timestamp: i64,
    pub dilation: f64,
    fractional_remainder: f64,
}

impl Default for TimeInfo {
    fn default() -> Self {
        Self {
            flow_timestamp_at_anchor: 0,
            monotonic_anchor_timestamp: 0,
            dilation: 1.0,
            fractional_remainder: 0.0,
        }
    }
}

impl TimeInfo {
    pub fn reset(&mut self, flow_timestamp: i64, monotonic_timestamp: i64) {
        self.flow_timestamp_at_anchor = flow_timestamp;
        self.monotonic_anchor_timestamp = monotonic_timestamp;
        self.dilation = 1.0;
        self.fractional_remainder = 0.0;
    }

    fn compute_exact(&self, monotonic_timestamp: i64) -> (i64, f64) {
        let elapsed_i128 =
            ((monotonic_timestamp as i128) - (self.monotonic_anchor_timestamp as i128)).max(0);
        if elapsed_i128 == 0 || self.dilation == 0.0 {
            return (self.flow_timestamp_at_anchor, self.fractional_remainder);
        }

        let delta_f = (elapsed_i128 as f64) * self.dilation + self.fractional_remainder;
        if !delta_f.is_finite() {
            let positive = (elapsed_i128 > 0) == (self.dilation > 0.0);
            return (if positive { i64::MAX } else { i64::MIN }, 0.0);
        }

        let whole_f = delta_f.trunc();
        let rem = delta_f - whole_f;

        if whole_f >= i128::MAX as f64 {
            return (i64::MAX, 0.0);
        }
        if whole_f <= i128::MIN as f64 {
            return (i64::MIN, 0.0);
        }

        if self.flow_timestamp_at_anchor.unsigned_abs() >= (1u64 << 53) {
            let sum_f = (self.flow_timestamp_at_anchor as f64) + delta_f;
            if !sum_f.is_finite() || sum_f >= i64::MAX as f64 {
                return (i64::MAX, 0.0);
            }
            if sum_f <= i64::MIN as f64 {
                return (i64::MIN, 0.0);
            }
            return (sum_f.trunc() as i64, 0.0);
        }

        let sum_i128 = (self.flow_timestamp_at_anchor as i128).saturating_add(whole_f as i128);
        if sum_i128 >= i64::MAX as i128 {
            (i64::MAX, 0.0)
        } else if sum_i128 <= i64::MIN as i128 {
            (i64::MIN, 0.0)
        } else {
            (sum_i128 as i64, rem)
        }
    }

    pub fn set_dilation(&mut self, monotonic_timestamp: i64, dilation: f64) {
        if !dilation.is_finite() || dilation.abs() > MAX_FINITE_DILATION {
            return;
        }
        let (anchored, rem) = self.compute_exact(monotonic_timestamp);
        self.flow_timestamp_at_anchor = anchored;
        self.monotonic_anchor_timestamp = monotonic_timestamp;
        self.dilation = dilation;
        self.fractional_remainder = rem;
    }

    pub fn current_flow_timestamp(&self, monotonic_timestamp: i64) -> i64 {
        self.compute_exact(monotonic_timestamp).0
    }
}

#[cfg(test)]
mod tests {
    use super::TimeInfo;

    #[test]
    fn current_flow_timestamp_follows_dilation() {
        let mut time = TimeInfo::default();
        time.reset(1_000, 100);
        assert_eq!(1_000, time.current_flow_timestamp(100));
        assert_eq!(1_250, time.current_flow_timestamp(350));

        time.set_dilation(350, 2.0);
        assert_eq!(1_250, time.current_flow_timestamp(350));
        assert_eq!(1_650, time.current_flow_timestamp(550));
    }

    #[test]
    fn dilation_changes_materialize_elapsed_flow_at_previous_rate() {
        let mut time = TimeInfo::default();
        time.reset(0, 0);
        time.set_dilation(100, 3.0);
        assert_eq!(100, time.current_flow_timestamp(100));
        time.set_dilation(200, 0.5);
        assert_eq!(400, time.current_flow_timestamp(200));
        assert_eq!(450, time.current_flow_timestamp(300));
    }

    #[test]
    fn lazy_reads_do_not_mutate_or_accumulate_rounding_drift() {
        let mut time = TimeInfo::default();
        time.reset(0, 0);
        time.set_dilation(0, 0.3);
        let a = time.current_flow_timestamp(1);
        let b = time.current_flow_timestamp(10);
        assert_eq!(0, a);
        assert_eq!(3, b);
    }

    #[test]
    fn fractional_flow_survives_repeated_reanchoring() {
        let mut time = TimeInfo::default();
        time.reset(1_000, 0);
        for ts in (100..=1_000).step_by(100) {
            time.set_dilation(ts - 100, 0.5);
        }
        assert_eq!(1_500, time.current_flow_timestamp(1_000));
    }

    #[test]
    fn non_finite_dilation_is_ignored() {
        let mut time = TimeInfo::default();
        time.reset(1_000, 0);
        time.set_dilation(100, 2.0);
        let expected = time;

        time.set_dilation(200, f64::INFINITY);
        assert_eq!(expected.flow_timestamp_at_anchor, time.flow_timestamp_at_anchor);
        assert_eq!(expected.monotonic_anchor_timestamp, time.monotonic_anchor_timestamp);
        assert_eq!(expected.dilation, time.dilation);

        time.set_dilation(200, f64::NAN);
        assert_eq!(expected.flow_timestamp_at_anchor, time.flow_timestamp_at_anchor);
        assert_eq!(expected.monotonic_anchor_timestamp, time.monotonic_anchor_timestamp);
        assert_eq!(expected.dilation, time.dilation);
    }

    #[test]
    fn huge_finite_dilation_is_ignored() {
        let mut time = TimeInfo::default();
        time.reset(1_000, 0);
        time.set_dilation(100, 2.0);
        let expected = time;

        time.set_dilation(200, f64::MAX);
        assert_eq!(expected.flow_timestamp_at_anchor, time.flow_timestamp_at_anchor);
        assert_eq!(expected.monotonic_anchor_timestamp, time.monotonic_anchor_timestamp);
        assert_eq!(expected.dilation, time.dilation);

        time.set_dilation(200, -f64::MAX);
        assert_eq!(expected.flow_timestamp_at_anchor, time.flow_timestamp_at_anchor);
        assert_eq!(expected.monotonic_anchor_timestamp, time.monotonic_anchor_timestamp);
        assert_eq!(expected.dilation, time.dilation);
    }

    #[test]
    fn flow_timestamps_saturate_in_both_directions() {
        let mut time = TimeInfo::default();
        time.reset(i64::MAX, 0);
        time.set_dilation(0, 2.0);
        assert_eq!(i64::MAX, time.current_flow_timestamp(1));

        time.reset(i64::MIN, 0);
        time.set_dilation(0, -2.0);
        assert_eq!(i64::MIN, time.current_flow_timestamp(1));
    }

    #[test]
    fn extreme_monotonic_timestamps_do_not_overflow_elapsed_time() {
        let mut time = TimeInfo::default();
        time.reset(0, i64::MIN);
        assert_eq!(i64::MAX, time.current_flow_timestamp(i64::MAX));
    }

    #[test]
    fn reanchoring_after_saturation_remains_safe() {
        let mut time = TimeInfo::default();
        time.reset(i64::MAX, 0);
        time.set_dilation(0, 2.0);
        time.set_dilation(1, -1.0);
        assert_eq!(i64::MAX, time.current_flow_timestamp(2));
    }
}
