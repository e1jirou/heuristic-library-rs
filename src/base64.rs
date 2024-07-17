pub struct Base64;

impl Base64 {
    const BASE: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
        'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
        'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
    ];

    fn inv_base(c: char) -> usize {
        if c >= 'a' && c <= 'z' {
            26 + (c as usize - 'a' as usize)
        } else if c >= 'A' && c <= 'Z' {
            c as usize - 'A' as usize
        } else if c >= '0' && c <= '9' {
            52 + (c as usize - '0' as usize)
        } else if c == '+' {
            62
        } else if c == '/' {
            63
        } else {
            unreachable!("invalid base64 char");
        }
    }

    pub fn encode<T: num_traits::PrimInt>(data: &[T]) -> String {
        todo!();
    }

    pub fn decode<T: num_traits::PrimInt>(s: &[u8]) -> &[T] {
        todo!();
    }
}
