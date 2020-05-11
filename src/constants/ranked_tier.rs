use std::convert::AsRef;
use std::fmt;
use Inner::*;

#[derive(Clone, PartialEq, Eq)]
pub struct RankedTier(Inner);

#[derive(Clone, PartialEq, Eq)]
enum Inner {
    Iron,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    Grandmaster,
    Challenger,
}

impl RankedTier {
    pub const IRON: RankedTier = RankedTier(Iron);

    pub const BRONZE: RankedTier = RankedTier(Bronze);

    pub const SILVER: RankedTier = RankedTier(Silver);

    pub const GOLD: RankedTier = RankedTier(Gold);

    pub const PLATINUM: RankedTier = RankedTier(Platinum);

    pub const DIAMOND: RankedTier = RankedTier(Diamond);

    pub const MASTER: RankedTier = RankedTier(Master);

    pub const GRANDMASTER: RankedTier = RankedTier(Grandmaster);

    pub const CHALLENGER: RankedTier = RankedTier(Challenger);

    #[inline]
    pub fn as_str(&self) -> &str {
        match self.0 {
            Iron => "IRON",
            Bronze => "BRONZE",
            Silver => "SILVER",
            Gold => "GOLD",
            Platinum => "PLATINUM",
            Diamond => "DIAMOND",
            Master => "MASTER",
            Grandmaster => "GRANDMASTER",
            Challenger => "CHALLENGER",
        }
    }
}

impl AsRef<str> for RankedTier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> PartialEq<&'a RankedTier> for RankedTier {
    #[inline]
    fn eq(&self, other: &&'a RankedTier) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<RankedTier> for &'a RankedTier {
    #[inline]
    fn eq(&self, other: &RankedTier) -> bool {
        *self == other
    }
}

impl PartialEq<str> for RankedTier {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl<'a> PartialEq<&'a str> for RankedTier {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<RankedTier> for str {
    #[inline]
    fn eq(&self, other: &RankedTier) -> bool {
        self == other.as_ref()
    }
}

impl fmt::Debug for RankedTier {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_ref())
    }
}

impl fmt::Display for RankedTier {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_ref())
    }
}
