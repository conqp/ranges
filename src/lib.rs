use std::ops::{Add, RangeInclusive, Sub};

/// Generate ranges from integer sequences
///
/// # Examples
///
/// ```
/// use std::ops::RangeInclusive;
/// use ranges::Ranges;
///
/// let sequence: Vec<i64> = vec![1, 2, 3, 6, 7, 9, 9, 9, 11, 20, 21, 22, 24, 23, 22];
/// let target: Vec<RangeInclusive<i64>> = vec![1..=3, 6..=7, 9..=9, 9..=9, 9..=9, 11..=11, 20..=22, 24..=22];
/// let ranges: Vec<RangeInclusive<i64>> = Ranges::from(sequence.into_iter()).collect();
///
/// assert_eq!(ranges, target);
/// ```
#[derive(Debug)]
pub struct Ranges<T, I>
where
    T: Add<T, Output = T>
        + Sub<T, Output = T>
        + PartialEq<<T as Add>::Output>
        + PartialEq<<T as Sub>::Output>
        + From<u8>
        + Copy,
    I: Iterator<Item = T>,
{
    numbers: I,
    start: Option<T>,
}

#[derive(Eq, PartialEq)]
enum Order {
    Ascending,
    Descending,
}

impl<T, I> Ranges<T, I>
where
    T: Add<T, Output = T>
        + Sub<T, Output = T>
        + PartialEq<<T as Add>::Output>
        + PartialEq<<T as Sub>::Output>
        + From<u8>
        + Copy,
    I: Iterator<Item = T>,
{
    pub fn new(numbers: I) -> Self {
        Self {
            numbers,
            start: None,
        }
    }
}

impl<T, I> Iterator for Ranges<T, I>
where
    T: Add<T, Output = T>
        + Sub<T, Output = T>
        + PartialEq<<T as Add>::Output>
        + PartialEq<<T as Sub>::Output>
        + From<u8>
        + Copy,
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
                    Some(start) => match &order {
                        None => {
                            if next == end.unwrap_or(start) + 1.into() {
                                end = Some(next);
                                order = Some(Order::Ascending);
                            } else if next == end.unwrap_or(start) - 1.into() {
                                end = Some(next);
                                order = Some(Order::Descending);
                            } else {
                                self.start = Some(next);
                                return Some(start..=end.unwrap_or(start));
                            }
                        }
                        Some(order) => {
                            if (order == &Order::Ascending
                                && next == end.unwrap_or(start) + 1.into())
                                || (order == &Order::Descending
                                    && next == end.unwrap_or(start) - 1.into())
                            {
                                end = Some(next)
                            } else {
                                self.start = Some(next);
                                return Some(start..=end.unwrap_or(start));
                            }
                        }
                    },
                },
            }
        }
    }
}

impl<T, I> From<I> for Ranges<T, I>
where
    T: Add<T, Output = T>
        + Sub<T, Output = T>
        + PartialEq<<T as Add>::Output>
        + PartialEq<<T as Sub>::Output>
        + From<u8>
        + Copy,
    I: Iterator<Item = T>,
{
    fn from(value: I) -> Self {
        Self::new(value)
    }
}
