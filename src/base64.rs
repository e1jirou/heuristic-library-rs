pub struct Base64;

impl Base64 {
    const BASE: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
        'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
        'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
    ];

    fn inv_base(c: u8) -> u8 {
        if c >= 'a' as u8 && c <= 'z' as u8 {
            26 + (c - 'a' as u8)
        } else if c >= 'A' as u8 && c <= 'Z' as u8 {
            c - 'A' as u8
        } else if c >= '0' as u8 && c <= '9' as u8 {
            52 + (c - '0' as u8)
        } else if c == '+' as u8 {
            62
        } else if c == '/' as u8 {
            63
        } else {
            unreachable!("invalid base64 char: {}", c);
        }
    }

    pub fn encode<T: num_traits::PrimInt>(data: &[T], bits: usize) -> String {
        debug_assert!(bits >= 6);
        let mut ret = vec![0; (data.len() * bits + 5) / 6 + 1];
        ret[0] = bits as u8;
        for (i, &x) in data.iter().enumerate() {
            for j in 0..bits {
                if ((x >> j) & T::one()) == T::one() {
                    let p = bits * i + j;
                    ret[p / 6 + 1] |= 1 << (p % 6);
                }
            }
        }
        for c in ret.iter_mut() {
            *c = Self::BASE[*c as usize] as u8;
        }
        String::from_utf8(ret).unwrap()
    }

    pub fn decode<T: num_traits::PrimInt + std::ops::BitOrAssign>(s: &String) -> Vec<T> {
        debug_assert!(!s.is_empty());
        let s: Vec<u8> = s.as_bytes().iter().map(|c| Self::inv_base(*c)).collect();
        let bits = s[0] as usize;
        let mut ret = vec![T::zero(); 6 * (s.len() - 1) / bits];
        for i in 1..(s.len()) {
            for j in 0..6 {
                if ((s[i] >> j) & 1) == 1 {
                    let p = (i - 1) * 6 + j;
                    ret[p / bits] |= T::one() << (p % bits);
                }
            }
        }
        ret
    }
}
