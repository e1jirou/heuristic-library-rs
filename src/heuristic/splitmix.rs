#[inline(always)]
pub fn splitmix32(mut x: u32) -> u32 {
    x = x.wrapping_add(0x9e3779b9);
    x = (x ^ (x >> 16)).wrapping_mul(0x85ebca6b);
    x = (x ^ (x >> 13)).wrapping_mul(0xc2b2ae35);
    x ^ (x >> 16)
}

#[inline(always)]
pub fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
