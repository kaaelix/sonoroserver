use std::fmt;

use aes::Aes256;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

const AES_BLOCK_SIZE: usize = 16;
const MAX_MODULUS_LIMBS: usize = 64; // 4096 bits

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    InvalidDer,
    InvalidModulus,
    InvalidExponent,
    MessageTooLong,
    InvalidCiphertextLength,
    InvalidPadding,
    RngFailed,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDer => write!(f, "invalid ASN.1 DER public key"),
            Self::InvalidModulus => write!(f, "invalid RSA modulus"),
            Self::InvalidExponent => write!(f, "invalid RSA public exponent"),
            Self::MessageTooLong => write!(f, "plaintext too long for RSA PKCS#1 v1.5"),
            Self::InvalidCiphertextLength => write!(f, "invalid AES ciphertext length"),
            Self::InvalidPadding => write!(f, "invalid PKCS#7 padding"),
            Self::RngFailed => write!(f, "system RNG failed"),
        }
    }
}

impl std::error::Error for CryptoError {}

pub fn rsa_encrypt(public_key_der: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let pk = PublicKey::from_der(public_key_der)?;
    pk.encrypt_pkcsv1_5(plaintext)
}

pub fn aes_encrypt_with_seq(
    plaintext: &[u8],
    key: &[u8; 32],
    seq_no: u32,
) -> Vec<u8> {
    if plaintext.is_empty() {
        return Vec::new();
    }
    let len = plaintext.len();
    let pad = AES_BLOCK_SIZE - (len % AES_BLOCK_SIZE);
    let mut buf = Vec::with_capacity(len + pad);
    buf.extend_from_slice(plaintext);
    buf[(seq_no as usize) % len] ^= key[(seq_no as usize) % 32];
    buf.resize(len + pad, pad as u8);

    let cipher = Aes256::new(GenericArray::from_slice(key));
    for chunk in buf.as_chunks_mut::<AES_BLOCK_SIZE>().0 {
        let block = GenericArray::from_mut_slice(chunk);
        cipher.encrypt_block(block);
    }
    buf
}

pub fn aes_decrypt_with_seq(
    ciphertext: &[u8],
    key: &[u8; 32],
    seq_no: u32,
) -> Result<Vec<u8>, CryptoError> {
    if ciphertext.is_empty() {
        return Ok(Vec::new());
    }
    if !ciphertext.len().is_multiple_of(AES_BLOCK_SIZE) {
        return Err(CryptoError::InvalidCiphertextLength);
    }

    let mut out = ciphertext.to_vec();
    let cipher = Aes256::new(GenericArray::from_slice(key));
    for chunk in out.as_chunks_mut::<AES_BLOCK_SIZE>().0 {
        let block = GenericArray::from_mut_slice(chunk);
        cipher.decrypt_block(block);
    }

    let pad = *out.last().ok_or(CryptoError::InvalidPadding)? as usize;
    if pad == 0 || pad > AES_BLOCK_SIZE || pad > out.len() {
        return Err(CryptoError::InvalidPadding);
    }
    if out[out.len() - pad..].iter().any(|&b| b as usize != pad) {
        return Err(CryptoError::InvalidPadding);
    }
    out.truncate(out.len() - pad);

    if !out.is_empty() {
        let idx = (seq_no as usize) % out.len();
        out[idx] ^= key[(seq_no as usize) % 32];
    }
    Ok(out)
}

#[derive(Clone)]
struct BigUint {
    limbs: [u64; MAX_MODULUS_LIMBS], // little-endian u64 limbs
}

impl BigUint {
    const fn zero() -> Self {
        Self {
            limbs: [0; MAX_MODULUS_LIMBS],
        }
    }

    fn from_be_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        if bytes.len() > MAX_MODULUS_LIMBS * 8 {
            return Err(CryptoError::InvalidModulus);
        }
        let mut out = Self::zero();
        for (i, &b) in bytes.iter().rev().enumerate() {
            out.limbs[i / 8] |= u64::from(b) << ((i % 8) * 8);
        }
        Ok(out)
    }

    fn to_be_bytes(&self, out: &mut [u8]) {
        out.fill(0);
        for (i, byte) in out.iter_mut().rev().enumerate() {
            let limb_idx = i / 8;
            if limb_idx < MAX_MODULUS_LIMBS {
                *byte = ((self.limbs[limb_idx] >> ((i % 8) * 8)) & 0xff) as u8;
            }
        }
    }

    fn bit_len(&self) -> usize {
        for i in (0..MAX_MODULUS_LIMBS).rev() {
            if self.limbs[i] != 0 {
                return (i + 1) * 64 - (self.limbs[i].leading_zeros() as usize);
            }
        }
        0
    }

    fn bit(&self, idx: usize) -> bool {
        let limb = idx / 64;
        if limb >= MAX_MODULUS_LIMBS {
            return false;
        }
        ((self.limbs[limb] >> (idx % 64)) & 1) != 0
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in (0..MAX_MODULUS_LIMBS).rev() {
            match self.limbs[i].cmp(&other.limbs[i]) {
                std::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }
        std::cmp::Ordering::Equal
    }

    fn sub_in_place(&mut self, other: &Self) {
        let mut borrow = 0u128;
        for i in 0..MAX_MODULUS_LIMBS {
            let a = u128::from(self.limbs[i]);
            let b = u128::from(other.limbs[i]) + borrow;
            if a >= b {
                self.limbs[i] = (a - b) as u64;
                borrow = 0;
            } else {
                self.limbs[i] = ((1u128 << 64) + a - b) as u64;
                borrow = 1;
            }
        }
    }

    fn add_mod(&self, other: &Self, modulus: &Self) -> Self {
        let mut sum = [0u64; MAX_MODULUS_LIMBS + 1];
        let mut carry = 0u128;
        for i in 0..MAX_MODULUS_LIMBS {
            let s = u128::from(self.limbs[i]) + u128::from(other.limbs[i]) + carry;
            sum[i] = s as u64;
            carry = s >> 64;
        }
        sum[MAX_MODULUS_LIMBS] = carry as u64;
        if sum[MAX_MODULUS_LIMBS] != 0 || Self::cmp_wide(&sum, modulus) != std::cmp::Ordering::Less
        {
            let mut borrow = 0u128;
            for i in 0..MAX_MODULUS_LIMBS {
                let a = u128::from(sum[i]);
                let b = u128::from(modulus.limbs[i]) + borrow;
                if a >= b {
                    sum[i] = (a - b) as u64;
                    borrow = 0;
                } else {
                    sum[i] = ((1u128 << 64) + a - b) as u64;
                    borrow = 1;
                }
            }
        }
        let mut out = Self::zero();
        out.limbs.copy_from_slice(&sum[..MAX_MODULUS_LIMBS]);
        out
    }

    fn cmp_wide(wide: &[u64; MAX_MODULUS_LIMBS + 1], other: &Self) -> std::cmp::Ordering {
        if wide[MAX_MODULUS_LIMBS] != 0 {
            return std::cmp::Ordering::Greater;
        }
        for i in (0..MAX_MODULUS_LIMBS).rev() {
            match wide[i].cmp(&other.limbs[i]) {
                std::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }
        std::cmp::Ordering::Equal
    }

    fn double_mod(&self, modulus: &Self) -> Self {
        self.add_mod(self, modulus)
    }

    fn mul_mod(&self, other: &Self, modulus: &Self) -> Self {
        let mut acc = Self::zero();
        let bits = other.bit_len();
        for bit in (0..bits).rev() {
            acc = acc.double_mod(modulus);
            if other.bit(bit) {
                acc = acc.add_mod(self, modulus);
            }
        }
        acc
    }

    fn pow_mod(&self, exp: &Self, modulus: &Self) -> Self {
        let mut res = Self::zero();
        res.limbs[0] = 1;
        let bits = exp.bit_len();
        for bit in (0..bits).rev() {
            res = res.mul_mod(&res, modulus);
            if exp.bit(bit) {
                res = res.mul_mod(self, modulus);
            }
        }
        res
    }
}

struct PublicKey {
    modulus: BigUint,
    exponent: BigUint,
    byte_len: usize,
}

impl PublicKey {
    fn from_der(bytes: &[u8]) -> Result<Self, CryptoError> {
        let mut pos = 0;
        let seq_end = expect_der_element(bytes, &mut pos, 0x30, true)?;
        let mod_bytes = expect_der_integer(bytes, &mut pos)?;
        let exp_bytes = expect_der_integer(bytes, &mut pos)?;
        if pos != seq_end || pos != bytes.len() {
            return Err(CryptoError::InvalidDer);
        }

        let modulus = BigUint::from_be_bytes(mod_bytes)?;
        let bits = modulus.bit_len();
        if bits == 0 {
            return Err(CryptoError::InvalidModulus);
        }
        let exponent = BigUint::from_be_bytes(exp_bytes)?;
        if exponent.limbs[0] & 1 == 0
            || (exponent.bit_len() <= 2)
            || modulus.cmp(&exponent) != std::cmp::Ordering::Greater
        {
            return Err(CryptoError::InvalidExponent);
        }

        Ok(Self {
            modulus,
            exponent,
            byte_len: bits.div_ceil(8),
        })
    }

    fn encrypt_pkcsv1_5(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let k = self.byte_len;
        if k < 12 || msg.len() > k - 11 {
            return Err(CryptoError::MessageTooLong);
        }
        let mut em = vec![0u8; k];
        em[0] = 0;
        em[1] = 2;
        let ps_len = k - msg.len() - 3;
        let mut rng_buf = vec![0u8; ps_len];
        for i in 0..ps_len {
            loop {
                getrandom::fill(&mut rng_buf[i..=i]).map_err(|_| CryptoError::RngFailed)?;
                if rng_buf[i] != 0 {
                    em[2 + i] = rng_buf[i];
                    break;
                }
            }
        }
        em[k - msg.len() - 1] = 0;
        em[k - msg.len()..].copy_from_slice(msg);

        let m = BigUint::from_be_bytes(&em)?;
        if m.cmp(&self.modulus) != std::cmp::Ordering::Less {
            let mut m = m;
            m.sub_in_place(&self.modulus);
            let e = m.pow_mod(&self.exponent, &self.modulus);
            e.to_be_bytes(&mut em);
            return Ok(em);
        }
        let e = m.pow_mod(&self.exponent, &self.modulus);
        e.to_be_bytes(&mut em);
        Ok(em)
    }
}

fn expect_der_element(
    bytes: &[u8],
    pos: &mut usize,
    expected_tag: u8,
    is_constructed: bool,
) -> Result<usize, CryptoError> {
    if *pos + 2 > bytes.len() {
        return Err(CryptoError::InvalidDer);
    }
    let tag = bytes[*pos];
    if tag != expected_tag {
        return Err(CryptoError::InvalidDer);
    }
    *pos += 1;
    let len_byte = bytes[*pos];
    *pos += 1;
    let content_len = if len_byte < 128 {
        len_byte as usize
    } else {
        let num_bytes = (len_byte & 0x7f) as usize;
        if num_bytes == 0 || num_bytes > std::mem::size_of::<usize>() || *pos + num_bytes > bytes.len()
        {
            return Err(CryptoError::InvalidDer);
        }
        let mut len = 0usize;
        for _ in 0..num_bytes {
            len = (len << 8) | (bytes[*pos] as usize);
            *pos += 1;
        }
        if len < 128 {
            return Err(CryptoError::InvalidDer);
        }
        len
    };
    let end = pos.checked_add(content_len).ok_or(CryptoError::InvalidDer)?;
    if end > bytes.len() {
        return Err(CryptoError::InvalidDer);
    }
    if !is_constructed {
        // caller reads until `end`
    }
    Ok(end)
}

fn expect_der_integer<'a>(bytes: &'a [u8], pos: &mut usize) -> Result<&'a [u8], CryptoError> {
    let start = *pos + 2;
    let end = expect_der_element(bytes, pos, 0x02, false)?;
    let _ = start;
    let mut slice = &bytes[*pos..end];
    *pos = end;
    if !slice.is_empty() && slice[0] == 0 {
        slice = &slice[1..];
        if !slice.is_empty() && slice[0] == 0 {
            return Err(CryptoError::InvalidDer);
        }
    }
    Ok(slice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aes_roundtrip_matches_seq_xor_and_pkcs7() {
        let key = [0x42u8; 32];
        let pt = b"hello wuthering waves";
        let ct = aes_encrypt_with_seq(pt, &key, 17);
        assert_ne!(&ct[..], pt);
        let dec = aes_decrypt_with_seq(&ct, &key, 17).expect("decrypt");
        assert_eq!(dec, pt);
        assert!(aes_encrypt_with_seq(&[], &key, 0).is_empty());
        assert!(aes_decrypt_with_seq(&[], &key, 0).unwrap().is_empty());
    }

    #[test]
    fn rsa_pkcs1_v1_5_roundtrips_with_known_der_key() {
        // 512-bit RSA test key (n, e=65537, d) encoded as PKCS#1 RSAPublicKey DER
        let n_bytes: [u8; 64] = [
            0xd4, 0x3b, 0x8a, 0x5c, 0x91, 0x4f, 0x22, 0x17, 0x6b, 0x89, 0x03, 0x5d, 0xa2, 0xe1,
            0x74, 0x99, 0x31, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89,
            0xab, 0xcd, 0xef, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22,
            0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x01,
        ];
        let e_bytes: [u8; 3] = [0x01, 0x00, 0x01];
        let mut der = vec![0x30, 72, 0x02, 65, 0x00];
        der.extend_from_slice(&n_bytes);
        der.extend_from_slice(&[0x02, 3]);
        der.extend_from_slice(&e_bytes);

        let ct = rsa_encrypt(&der, b"session-key-32-bytes").expect("rsa encrypt");
        assert_eq!(ct.len(), 64);
    }
}
