use crate::range::Range;
use num_traits::One;
use order::Order;
use std::fmt::Display;
use std::ops::{Add, Sub};

mod order;

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
    pub const fn new(numbers: T) -> Self {
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
        + One
        + PartialEq
        + Sub<T::Item, Output = T::Item>,
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
                        None => {
                            if let Some(new_order) = Order::new(last, next) {
                                order = Some(new_order);
                                end = Some(next);
                            } else {
                                self.start = Some(next);
                                return Some(Range::new(start, last));
                            }
                        }
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
            end.map_or_else(
                || Some(Range::new(start, start)),
                |end| Some(Range::new(start, end)),
            )
        } else {
            None
        }
    }
}

impl<T> From<T> for RangesIterator<T>
where
    T: Iterator,
    T::Item: One,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
