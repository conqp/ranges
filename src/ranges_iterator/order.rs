use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Order {
    Ascending,
    Descending,
}

impl Order {
    pub fn new<T>(start: T, next: T) -> Option<Self>
    where
        T: PartialEq + Add<T, Output = T> + Copy + Sub<T, Output = T> + From<u8>,
    {
        if next == start + 1.into() {
            Some(Self::Ascending)
        } else if next == start - 1.into() {
            Some(Self::Descending)
        } else {
            None
        }
    }
}
