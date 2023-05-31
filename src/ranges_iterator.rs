use crate::order::Order;
use crate::Range;
use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct RangesIterator<T>
where
    T: Iterator,
{
    numbers: T,
    start: Option<T::Item>,
}

impl<T> RangesIterator<T>
where
    T: Iterator,
{
    pub fn new(numbers: T) -> Self {
        Self {
            numbers,
            start: None,
        }
    }
}

impl<T> Iterator for RangesIterator<T>
where
    T: Iterator,
    T::Item: Add<T::Item, Output = T::Item>
        + Copy
        + Display
        + From<u8>
        + Sub<T::Item, Output = T::Item>
        + PartialEq,
{
    type Item = Range<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut order: Option<Order> = None;
        let mut end: Option<T::Item> = None;

        for next in &mut self.numbers {
            match self.start {
                None => {
                    self.start = Some(next);
                }
                Some(start) => {
                    let last = end.unwrap_or(start);

                    match &order {
                        None => match Order::new(last, next) {
                            Some(new_order) => {
                                order = Some(new_order);
                                end = Some(next);
                            }
                            None => {
                                self.start = Some(next);
                                return Some(Range::new(start, last));
                            }
                        },
                        Some(order) => {
                            if let Some(new_order) = &Order::new(last, next) {
                                if new_order == order {
                                    end = Some(next);
                                    continue;
                                }
                            }
                            self.start = Some(next);
                            return Some(Range::new(start, last));
                        }
                    }
                }
            }
        }

        if let Some(start) = self.start {
            self.start = None;

            match end {
                None => Some(Range::new(start, start)),
                Some(end) => Some(Range::new(start, end)),
            }
        } else {
            None
        }
    }
}

impl<T> From<T> for RangesIterator<T>
where
    T: Iterator,
    T::Item: From<u8>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
