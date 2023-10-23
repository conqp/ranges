use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq)]
pub enum Order {
    Ascending,
    Descending,
}

impl Order {
    pub fn new<T>(start: T, next: T) -> Option<Self>
    where
        T: Add<T, Output = T> + Copy + From<u8> + PartialEq + Sub<T, Output = T>,
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
