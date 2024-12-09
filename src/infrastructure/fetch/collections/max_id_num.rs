use std::{fmt::Display, num::NonZero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaxIdNum(NonZero<usize>);

impl MaxIdNum {
    pub fn new(max_id_num: NonZero<usize>) -> Self {
        Self(max_id_num)
    }

    pub fn max_limit() -> NonZero<usize> {
        NonZero::try_from(50).unwrap()
    }

    pub fn max_id_num(&self) -> usize {
        self.0.into()
    }
}

impl Default for MaxIdNum {
    fn default() -> Self {
        Self::new(Self::max_limit())
    }
}

impl Display for MaxIdNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<MaxIdNum> for usize {
    fn from(value: MaxIdNum) -> Self {
        value.0.into()
    }
}
