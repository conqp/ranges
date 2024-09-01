use num_traits::One;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Order {
    Ascending,
    Descending,
}

impl Order {
    pub fn new<T>(start: T, next: T) -> Option<Self>
    where
        T: Add<T, Output = T> + Copy + One + PartialEq + Sub<T, Output = T>,
    {
        if next == start + T::one() {
            Some(Self::Ascending)
        } else if next == start - T::one() {
            Some(Self::Descending)
        } else {
            None
        }
    }
}
