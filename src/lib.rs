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
        let mut last: Option<i64> = None;

        loop {
            match self.numbers.next() {
                None => {
                    if let Some(start) = self.start {
                        if let Some(last) = last {
                            return Some(start..=last);
                        }
                    }
                }
                Some(current) => match self.start {
                    None => {
                        self.start = Some(current);
                    }
                    Some(_) => match last {
                        None => {
                            last = Some(current);
                        }
                        Some(l) => {
                            if current == l + 1 {
                                last = Some(current);
                            } else {
                                let range = Some(self.start?..=last?);
                                self.start = Some(current);
                                return range;
                            }
                        }
                    },
                },
            }
        }
    }
}
