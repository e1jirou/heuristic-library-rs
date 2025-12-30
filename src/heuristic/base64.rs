pub struct Base64;

impl Base64 {
    const BASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    fn inv_base(c: u8) -> u8 {
        if c >= 'A' as u8 && c <= 'Z' as u8 {
            c - 'A' as u8
        } else if c >= 'a' as u8 && c <= 'z' as u8 {
            26 + (c - 'a' as u8)
        } else if c >= '0' as u8 && c <= '9' as u8 {
            52 + (c - '0' as u8)
        } else if c == '+' as u8 {
            62
        } else if c == '/' as u8 {
            63
        } else {
            panic!("invalid base64 u8: {}", c);
        }
    }

    pub fn encode_bytes<T>(data: &[T]) -> String {
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<T>(),
            )
        };
        let mut ret = vec![0; (bytes.len() * 8 + 5) / 6];
        for i in 0..ret.len() {
            let p = 6 * i;
            let mut out = bytes[p / 8] >> (p % 8);
            if p % 8 > 2 && p / 8 + 1 < bytes.len() {
                out |= bytes[p / 8 + 1] << (8 - (p % 8));
            }
            ret[i] = Self::BASE[(out & 0x3f) as usize];
        }
        String::from_utf8(ret).unwrap()
    }

    pub fn decode_bytes<T: Copy>(s: &[u8]) -> Vec<T> {
        let s: Vec<u8> = s.iter().map(|c| Self::inv_base(*c)).collect();
        let elem_size = std::mem::size_of::<T>();
        assert!(elem_size != 0);
        let byte_len = 6 * s.len() / 8;
        assert!(byte_len % elem_size == 0);
        let len = byte_len / elem_size;
        let mut out = Vec::with_capacity(len);
        unsafe {
            out.set_len(len);
            std::ptr::write_bytes(out.as_mut_ptr() as *mut u8, 0, byte_len);
            let bytes = std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, byte_len);
            for i in 0..s.len() {
                let p = 6 * i;
                bytes[p / 8] |= s[i] << (p % 8);
                if p % 8 > 2 && p / 8 + 1 < bytes.len() {
                    bytes[p / 8 + 1] |= s[i] >> (8 - (p % 8));
                }
            }
        }
        out
    }

    pub fn encode_ints<T: num_traits::PrimInt + std::ops::BitOrAssign>(
        data: &[T],
        bits: usize,
    ) -> String {
        debug_assert!(6 <= bits && bits < 64);
        let mut ret = vec![0; (data.len() * bits + 5) / 6 + 1];
        ret[0] = Self::BASE[bits as usize];
        let mask = T::from(0x3f).unwrap();
        for i in 1..ret.len() {
            let p = 6 * (i - 1);
            let mut out = data[p / bits] >> (p % bits);
            if p % bits > bits - 6 && p / bits + 1 < data.len() {
                out |= data[p / bits + 1] << (bits - (p % bits));
            }
            ret[i] = Self::BASE[(out & mask).to_usize().unwrap()];
        }
        String::from_utf8(ret).unwrap()
    }

    pub fn decode_ints<T: num_traits::PrimInt + std::ops::BitOrAssign>(s: &[u8]) -> Vec<T> {
        debug_assert!(!s.is_empty());
        let s: Vec<u8> = s.iter().map(|c| Self::inv_base(*c)).collect();
        let bits = s[0] as usize;
        let mut ret = vec![T::zero(); 6 * (s.len() - 1) / bits];
        let mask = (T::one() << bits) - T::one();
        for i in 1..s.len() {
            let p = 6 * (i - 1);
            ret[p / bits] |= (T::from(s[i]).unwrap() << (p % bits)) & mask;
            if p % bits > bits - 6 && p / bits + 1 < ret.len() {
                ret[p / bits + 1] |= T::from(s[i]).unwrap() >> (bits - (p % bits));
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_bytes_u8_various_lengths() {
        for len in 0..64 {
            let data: Vec<u8> = (0..len as u8).map(|x| x.wrapping_mul(37)).collect();
            let s = Base64::encode_bytes(&data);
            let back: Vec<u8> = Base64::decode_bytes(&s.as_bytes());
            assert_eq!(data, back);
        }
    }

    #[test]
    fn roundtrip_bytes_u32() {
        let data: Vec<u32> = (0..200).map(|i| (i * 1_000_003) as u32).collect();
        let s = Base64::encode_bytes(&data);
        let back: Vec<u32> = Base64::decode_bytes(&s.as_bytes());
        assert_eq!(data, back);
    }

    #[test]
    fn roundtrip_bytes_f32_bits() {
        let data = vec![0.0f32, -0.0, 1.5, -2.25, 1000.0, 1e-10];
        let s = Base64::encode_bytes(&data);
        let back: Vec<f32> = Base64::decode_bytes(&s.as_bytes());
        assert_eq!(
            data.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            back.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn roundtrip_ints_u16_bits_10() {
        let bits = 10usize;
        let mask = (1u16 << bits) - 1;
        let data: Vec<u16> = (0..300).map(|i| ((i * 37) as u16) & mask).collect();
        let s = Base64::encode_ints(&data, bits);
        let back: Vec<u16> = Base64::decode_ints(&s.as_bytes());
        assert_eq!(data, back);
    }
}
