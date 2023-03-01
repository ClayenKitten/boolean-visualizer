use bitvec::prelude::*;

pub struct BoolIterator {
    width: u8,
    value: u32,
}

impl BoolIterator {
    pub const fn new(width: u8) -> Self {
        Self {
            width,
            value: 0,
        }
    }
}

impl Iterator for BoolIterator {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.width == 0 ||
            self.value >= 2u32.pow(self.width as u32) {
            return None;
        }
        let item = self.value.view_bits::<Lsb0>()
            .iter()
            .by_vals()
            .take(self.width as usize)
            .collect();
        self.value += 1;
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::BoolIterator;

    #[test]
    fn bool_iterator_0() {
        let mut iter = BoolIterator::new(0);
        assert_eq!(None, iter.next());
    }

    #[test]
    fn bool_iterator_1() {
        let mut iter = BoolIterator::new(1);
        assert_eq!(Some(vec![false]), iter.next());
        assert_eq!(Some(vec![true]), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn bool_iterator_2() {
        let mut iter = BoolIterator::new(2);
        assert_eq!(Some(vec![false, false]), iter.next());
        assert_eq!(Some(vec![true, false]), iter.next());
        assert_eq!(Some(vec![false, true]), iter.next());
        assert_eq!(Some(vec![true, true]), iter.next());
        assert_eq!(None, iter.next());
    }
}
