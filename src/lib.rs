use std::ops::RangeInclusive;

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
///
/// assert_eq!(Ranges::from(sequence.into_iter()).collect::<Vec<_>>(), target);
/// ```
#[derive(Debug)]
pub struct Ranges<T>
where
    T: Iterator<Item = i64>,
{
    numbers: T,
    start: Option<i64>,
}

#[derive(Clone, Eq, PartialEq)]
enum Order {
    Descending,
    Ascending,
}

impl<T> Ranges<T>
where
    T: Iterator<Item = i64>,
{
    pub fn new(numbers: T) -> Self {
        Self {
            numbers,
            start: None,
        }
    }
}

impl<T> Iterator for Ranges<T>
where
    T: Iterator<Item = i64>,
{
    type Item = RangeInclusive<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut order: Option<Order> = None;
        let mut end: Option<i64> = None;

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
                    Some(start) => match end {
                        None => match order.clone() {
                            None => {
                                if next == start + 1 {
                                    order = Some(Order::Ascending);
                                    end = Some(next);
                                } else if next == start - 1 {
                                    order = Some(Order::Descending);
                                    end = Some(next);
                                } else {
                                    self.start = Some(next);
                                    return Some(start..=start);
                                }
                            }
                            Some(order) => {
                                if (order == Order::Ascending && next == start + 1)
                                    || (order == Order::Descending && next == start - 1)
                                {
                                    end = Some(next);
                                } else {
                                    self.start = Some(next);
                                    return Some(start..=start);
                                }
                            }
                        },
                        Some(last) => match order.clone() {
                            None => {
                                if next == last + 1 {
                                    order = Some(Order::Ascending);
                                    end = Some(next);
                                } else if next == last - 1 {
                                    order = Some(Order::Descending);
                                    end = Some(next);
                                } else {
                                    self.start = Some(next);
                                    return Some(start..=last);
                                }
                            }
                            Some(order) => {
                                if (order == Order::Ascending && next == last + 1)
                                    || (order == Order::Descending && next == last - 1)
                                {
                                    end = Some(next);
                                } else {
                                    self.start = Some(next);
                                    return Some(start..=last);
                                }
                            }
                        },
                    },
                },
            }
        }
    }
}

impl<T> From<T> for Ranges<T>
where
    T: Iterator<Item = i64>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
