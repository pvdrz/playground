#[cfg(feature = "feat")]
pub type Data = Vec<u8>;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    #[cfg(feature = "feat")]
    pub data: Data,
}
