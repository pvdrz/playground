#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    #[cfg(feature = "feat")]
    pub data: crate_c::Data,
}
