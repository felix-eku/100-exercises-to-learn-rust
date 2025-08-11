use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SaturatingU16(u16);

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16(value)
    }
}
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16(value.into())
    }
}
impl<T: Into<SaturatingU16> + Copy> From<&T> for SaturatingU16 {
    fn from(value: &T) -> Self {
        (*value).into()
    }
}

impl Add for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: Self) -> Self::Output {
        SaturatingU16::from(self.0.saturating_add(rhs.0))
    }
}
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &Self) -> Self::Output {
        SaturatingU16::from(self.0.saturating_add(rhs.0))
    }
}
impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16::from(self.0.saturating_add(rhs))
    }
}
impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16::from(self.0.saturating_add(*rhs))
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}
