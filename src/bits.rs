/// Bit manipulations defined and implemented as a trait.
///
/// This is really for my own convenience as I'm no bit
/// twiddling ninja and reading bitwise operators makes my
/// head hurt.
pub(crate) trait Bits {
    fn bit_range(&self, start: u8, stop: u8) -> u8;

    fn bit_is_set(&self, n: u8) -> bool {
        !self.bit_not_set(n)
    }

    fn bit_not_set(&self, n: u8) -> bool {
        !self.bit_is_set(n)
    }
}

impl Bits for u8 {
    fn bit_not_set(&self, n: u8) -> bool {
        self & (1 << n) == 0
    }

    fn bit_range(&self, lsb: u8, msb: u8) -> u8 {
        (self >> lsb) & !(!0 << (msb - lsb + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_setting() {
        assert!((0b10000100 as u8).bit_is_set(7));
        assert!((0b01000000 as u8).bit_is_set(6));
        assert!((0b00100000 as u8).bit_is_set(5));
        assert!((0b00010000 as u8).bit_is_set(4));
        assert!((0b01001000 as u8).bit_is_set(3));
        assert!((0b00000100 as u8).bit_is_set(2));
        assert!((0b10110010 as u8).bit_is_set(1));
        assert!((0b00010001 as u8).bit_is_set(0));
    }

    #[test]
    fn bit_range() {
        assert!((0b10110010 as u8).bit_range(0, 3) == 0b00000010);
        assert!((0b10110111 as u8).bit_range(0, 3) == 0b00000111);
        assert!((0b00111000 as u8).bit_range(3, 5) == 0b00000111);
    }
}
