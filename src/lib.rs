use std::ops::RangeInclusive;

pub struct Ranges<T>
where
    T: Iterator<Item = i64>,
{
    numbers: T,
    start: Option<i64>,
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
                        None => {
                            if next == start + 1 {
                                end = Some(next);
                            } else {
                                self.start = Some(next);
                                return Some(start..=start);
                            }
                        }
                        Some(last) => {
                            if last + 1 == next {
                                end = Some(next);
                            } else {
                                self.start = Some(next);
                                return Some(start..=last);
                            }
                        }
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
