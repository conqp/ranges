use std::ops::{Add, RangeInclusive, Sub};

/// Generate ranges from integer sequences
///
/// # Examples
///
/// ```
/// use std::ops::RangeInclusive;
/// use ranges::RangesIterator;
///
/// let sequence: Vec<i64> = vec![1, 2, 3, 6, 7, 9, 9, 9, 11, 20, 21, 22, 24, 23, 22];
/// let target: Vec<RangeInclusive<i64>> = vec![1..=3, 6..=7, 9..=9, 9..=9, 9..=9, 11..=11, 20..=22, 24..=22];
/// let ranges: Vec<RangeInclusive<i64>> = RangesIterator::from(sequence.into_iter()).collect();
///
/// assert_eq!(ranges, target);
/// ```
pub trait Ranges<T>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + PartialEq + From<u8> + Copy,
    Self: Iterator<Item = T> + Sized,
{
    fn ranges(self) -> RangesIterator<T, Self>;
}

impl<T, I> Ranges<T> for I
where
    T: Add<T, Output = T> + Sub<T, Output = T> + PartialEq + From<u8> + Copy,
    I: Iterator<Item = T>,
{
    fn ranges(self) -> RangesIterator<T, Self> {
        RangesIterator::from(self)
    }
}

#[derive(Debug)]
pub struct RangesIterator<T, I>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + PartialEq + From<u8> + Copy,
    I: Iterator<Item = T>,
{
    numbers: I,
    start: Option<T>,
    one: T,
}

#[derive(Eq, PartialEq)]
enum Order {
    Ascending,
    Descending,
}

impl<T, I> RangesIterator<T, I>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + PartialEq + From<u8> + Copy,
    I: Iterator<Item = T>,
{
    pub fn new(numbers: I) -> Self {
        Self {
            numbers,
            start: None,
            one: 1.into(),
        }
    }
}

impl<T, I> Iterator for RangesIterator<T, I>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + PartialEq + From<u8> + Copy,
    I: Iterator<Item = T>,
{
    type Item = RangeInclusive<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut order: Option<Order> = None;
        let mut end: Option<T> = None;

        loop {
            match self.numbers.next() {
                None => {
                    return match self.start {
                        None => None,
                        Some(start) => {
                            self.start = None;

                            match end {
                                None => Some(start..=start),
                                Some(end) => Some(start..=end),
                            }
                        }
                    }
                }
                Some(next) => match self.start {
                    None => {
                        self.start = Some(next);
                    }
                    Some(start) => {
                        let last = end.unwrap_or(start);

                        match &order {
                            None => {
                                if next == last + self.one {
                                    end = Some(next);
                                    order = Some(Order::Ascending);
                                } else if next == last - self.one {
                                    end = Some(next);
                                    order = Some(Order::Descending);
                                } else {
                                    self.start = Some(next);
                                    return Some(start..=last);
                                }
                            }
                            Some(order) => {
                                if (order == &Order::Ascending && next == last + self.one)
                                    || (order == &Order::Descending && next == last - self.one)
                                {
                                    end = Some(next)
                                } else {
                                    self.start = Some(next);
                                    return Some(start..=last);
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}

impl<T, I> From<I> for RangesIterator<T, I>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + PartialEq + From<u8> + Copy,
    I: Iterator<Item = T>,
{
    fn from(value: I) -> Self {
        Self::new(value)
    }
}
