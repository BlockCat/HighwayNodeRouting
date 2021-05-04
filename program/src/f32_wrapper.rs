#[derive(Debug, PartialEq, PartialOrd)]
pub struct F32Wrapper(pub f32);

impl Ord for F32Wrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Eq for F32Wrapper {}