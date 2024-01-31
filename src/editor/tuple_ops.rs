use std::{cmp, ops::{Add, Sub}};

#[derive(PartialEq, Debug)]
pub struct T(pub (usize, usize));

pub fn t(a: usize, b: usize) -> T { T((a, b)) }

impl Add for T {
    type Output = T;

    fn add(self, other: T) -> T {
        T((
            self.0.0 + other.0.0,
            self.0.1 + other.0.1
        ))
    }
}

impl Sub for T {
    type Output = T;

    fn sub(self, other: T) -> T {
        T((
            self.0.0 - cmp::min(other.0.0, self.0.0),
            self.0.1 - cmp::min(other.0.1, self.0.1)
        ))
    }
}


#[cfg(test)]
mod tests {
    use super::t;

    #[test]
    fn test_add() {
        assert_eq!(t(1, 2) + t(3, 4), t(4, 6));
    }

    #[test]
    fn test_sub() {
        assert_eq!(t(1, 2) - t(3, 4), t(0, 0));
        assert_eq!(t(3, 4) - t(1, 2), t(2, 2));
    }
}
