use self::Inner::*;

use std::convert::AsRef;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Region(Inner);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Inner {
    BR,
    EUNE,
    EUW,
    JP,
    KR,
    LAN,
    LAS,
    NA,
    OCE,
    TR,
    RU,
    PBE,
}

impl Region {
    /// Brazil
    pub const BR: Region = Region(BR);
    /// Europe North East
    pub const EUNE: Region = Region(EUNE);
    /// Europe West
    pub const EUW: Region = Region(EUW);
    /// Japan
    pub const JP: Region = Region(JP);
    /// Korea
    pub const KR: Region = Region(KR);
    /// Latin America North
    pub const LAN: Region = Region(LAN);
    /// Latin America South
    pub const LAS: Region = Region(LAS);
    /// North America
    pub const NA: Region = Region(NA);
    /// Oceania
    pub const OCE: Region = Region(OCE);
    /// Turkey
    pub const TR: Region = Region(TR);
    /// Russia
    pub const RU: Region = Region(RU);
    /// Public Beta Environment
    pub const PBE: Region = Region(PBE);

    /// String representation of Region
    #[inline]
    pub fn as_str(&self) -> &str {
        match self.0 {
            BR => "BR",
            EUNE => "EUNE",
            EUW => "EUW",
            JP => "JP",
            KR => "KR",
            LAN => "LAN",
            LAS => "LAS",
            NA => "NA",
            OCE => "OCE",
            TR => "TR",
            RU => "RU",
            PBE => "PBE",
        }
    }

    /// Platform string to make requests
    #[inline]
    pub fn as_platform_str(&self) -> &str {
        match self.0 {
            BR => "BR1",
            EUNE => "EUN1",
            EUW => "EUW1",
            JP => "JP1",
            KR => "KR",
            LAN => "LA1",
            LAS => "LA2",
            NA => "NA1",
            OCE => "OC1",
            TR => "TR1",
            RU => "RU",
            PBE => "PBE1",
        }
    }
}

impl AsRef<str> for Region {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Default for Region {
    #[inline]
    fn default() -> Region {
        Region::NA
    }
}

#[cfg(test)]
mod tests {
    use super::Region;

    #[test]
    fn region_gets_correct_platform_string() {
        assert_eq!(Region::NA.as_platform_str(), "NA1")
    }
}
