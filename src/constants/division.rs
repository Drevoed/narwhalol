use std::convert::AsRef;
use std::fmt::{self, Debug};
use Inner::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Division(Inner);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Inner {
    I,
    II,
    III,
    IV,
}

impl Division {
    pub const I: Division = Division(I);

    pub const II: Division = Division(II);

    pub const III: Division = Division(III);

    pub const IV: Division = Division(IV);

    #[inline]
    pub fn as_str(&self) -> &str {
        match self.0 {
            I => "I",
            II => "II",
            III => "III",
            IV => "IV",
        }
    }
}

impl AsRef<str> for Division {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> PartialEq<&'a Division> for Division {
    #[inline]
    fn eq(&self, other: &&'a Division) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<Division> for &'a Division {
    #[inline]
    fn eq(&self, other: &Division) -> bool {
        *self == other
    }
}

impl PartialEq<str> for Division {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl<'a> PartialEq<&'a str> for Division {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<Division> for str {
    #[inline]
    fn eq(&self, other: &Division) -> bool {
        self == other.as_ref()
    }
}

impl fmt::Display for Division {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_ref())
    }
}

mod tests {
    use super::*;

    #[test]
    fn returns_correct_str() {
        let division = Division::III;
        assert_eq!(division.as_str(), "III")
    }
}
