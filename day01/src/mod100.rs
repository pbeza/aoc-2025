use std::ops::{Add, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Mod100(u8);

impl Mod100 {
    #[inline]
    pub fn new(v: u16) -> Self {
        Mod100((v % 100) as u8)
    }

    #[inline]
    fn add_mod_100(a: u8, b: u8) -> u8 {
        // Branchless addition
        let s = a as u16 + b as u16; // 0..198
        let m = (((s >= 100) as u16).wrapping_neg()) as u8; // 0x00 or 0xFF
        (s as u8).wrapping_sub(m & 100)
    }

    #[inline]
    fn sub_mod_100(a: u8, b: u8) -> u8 {
        // Branchless subtraction
        let d = a as i16 - b as i16; // -99..99
        let m = (((d < 0) as i16).wrapping_neg()) as u8; // 0x00 or 0xFF
        (d + ((m as i16) & 100)) as u8
    }
}

impl Add for Mod100 {
    type Output = Mod100;

    #[inline]
    fn add(self, rhs: Mod100) -> Mod100 {
        Mod100(Self::add_mod_100(self.0, rhs.0))
    }
}

impl Sub for Mod100 {
    type Output = Mod100;

    #[inline]
    fn sub(self, rhs: Mod100) -> Mod100 {
        Mod100(Self::sub_mod_100(self.0, rhs.0))
    }
}
