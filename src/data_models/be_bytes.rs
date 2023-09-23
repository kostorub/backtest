//
pub trait ToFromBytes {
    fn size() -> usize;
    fn from_be_bytes(b: &[u8]) -> Self;
    fn to_be_bytes(&self) -> Vec<u8>;
}
