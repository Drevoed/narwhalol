use self::Inner::*;

use std::convert::AsRef;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LanguageCode(Inner);

#[derive(Clone, PartialEq, Eq, Hash)]
enum Inner {
    CzechRepublic,
    Greece,
    Poland,
    Romania,
    Hungary,
    UnitedKingdom,
    Germany,
    Spain,
    Italy,
    France,
    Japan,
    Korea,
    Mexico,
    Argentina,
    Brazil,
    UnitedStates,
    Australia,
    Russia,
    Turkey,
    Malaysia,
    RepublicOfThePhilipinnes,
    Singapore,
    Thailand,
    Vietnam,
    Indonesia,
    MalaysiaChinese,
    China,
    Taiwan,
}

impl LanguageCode {
    /// Czech Republic language code
    pub const CZECH_REPUBLIC: LanguageCode = LanguageCode(CzechRepublic);
    /// Greece language code
    pub const GREECE: LanguageCode = LanguageCode(Greece);
    /// Greece language code
    pub const POLAND: LanguageCode = LanguageCode(Poland);
    /// Greece language code
    pub const ROMANIA: LanguageCode = LanguageCode(Romania);
    /// Greece language code
    pub const HUNGARY: LanguageCode = LanguageCode(Hungary);
    /// Greece language code
    pub const UNITED_KINGDOM: LanguageCode = LanguageCode(UnitedKingdom);
    /// Greece language code
    pub const GERMANY: LanguageCode = LanguageCode(Germany);
    /// Greece language code
    pub const SPAIN: LanguageCode = LanguageCode(Spain);
    /// Greece language code
    pub const ITALY: LanguageCode = LanguageCode(Italy);
    /// Greece language code
    pub const FRANCE: LanguageCode = LanguageCode(France);
    /// Greece language code
    pub const JAPAN: LanguageCode = LanguageCode(Japan);
    /// Greece language code
    pub const KOREA: LanguageCode = LanguageCode(Korea);
    /// Greece language code
    pub const MEXICO: LanguageCode = LanguageCode(Mexico);
    /// Greece language code
    pub const ARGENTINA: LanguageCode = LanguageCode(Argentina);
    /// Greece language code
    pub const BRAZIL: LanguageCode = LanguageCode(Brazil);
    /// Greece language code
    pub const UNITED_STATES: LanguageCode = LanguageCode(UnitedStates);
    /// Greece language code
    pub const AUSTRALIA: LanguageCode = LanguageCode(Australia);
    /// Greece language code
    pub const RUSSIA: LanguageCode = LanguageCode(Russia);
    /// Greece language code
    pub const TURKEY: LanguageCode = LanguageCode(Turkey);
    /// Greece language code
    pub const MALAYSIA: LanguageCode = LanguageCode(Malaysia);
    /// Greece language code
    pub const PHILIPINNES: LanguageCode = LanguageCode(RepublicOfThePhilipinnes);
    /// Greece language code
    pub const SINGAPORE: LanguageCode = LanguageCode(Singapore);
    /// Greece language code
    pub const THAILAND: LanguageCode = LanguageCode(Thailand);
    /// Greece language code
    pub const VIETNAM: LanguageCode = LanguageCode(Vietnam);
    /// Greece language code
    pub const INDONESIA: LanguageCode = LanguageCode(Indonesia);
    /// Greece language code
    pub const MALAYSIA_CHINESE: LanguageCode = LanguageCode(MalaysiaChinese);
    /// Greece language code
    pub const CHINA: LanguageCode = LanguageCode(China);
    /// Greece language code
    pub const TAIWAN: LanguageCode = LanguageCode(Taiwan);

    #[inline]
    pub fn as_str(&self) -> &str {
        match self.0 {
            CzechRepublic => "cs_CZ",
            Greece => "el_GR",
            Poland => "pl_PL",
            Romania => "ro_RO",
            Hungary => "hu_HU",
            UnitedKingdom => "en_GB",
            Germany => "de_DE",
            Spain => "es_ES",
            Italy => "it_IT",
            France => "fr_FR",
            Japan => "ja_JP",
            Korea => "ko_KR",
            Mexico => "es_MX",
            Argentina => "es_AR",
            Brazil => "pt_BR",
            UnitedStates => "en_US",
            Australia => "en_AU",
            Russia => "ru_RU",
            Turkey => "tr_TR",
            Malaysia => "ms_MY",
            RepublicOfThePhilipinnes => "en_PH",
            Singapore => "en_SG",
            Thailand => "th_TH",
            Vietnam => "vn_VN",
            Indonesia => "id_ID",
            MalaysiaChinese => "zh_MY",
            China => "zh_CN",
            Taiwan => "zh_TW",
        }
    }
}

impl AsRef<str> for LanguageCode {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> PartialEq<&'a LanguageCode> for LanguageCode {
    #[inline]
    fn eq(&self, other: &&'a LanguageCode) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<LanguageCode> for &'a LanguageCode {
    #[inline]
    fn eq(&self, other: &LanguageCode) -> bool {
        *self == other
    }
}

impl PartialEq<str> for LanguageCode {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl<'a> PartialEq<&'a str> for LanguageCode {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<LanguageCode> for str {
    #[inline]
    fn eq(&self, other: &LanguageCode) -> bool {
        self == other.as_ref()
    }
}

impl fmt::Debug for LanguageCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_ref())
    }
}

impl fmt::Display for LanguageCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::LanguageCode;

    #[test]
    fn lang_code_returns_correct_lang_string() {
        assert_eq!(LanguageCode::TURKEY, "tr_TR")
    }
}
