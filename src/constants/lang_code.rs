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

    pub const POLAND: LanguageCode = LanguageCode(Poland);

    pub const ROMANIA: LanguageCode = LanguageCode(Romania);

    pub const HUNGARY: LanguageCode = LanguageCode(Hungary);

    pub const UNITED_KINGDOM: LanguageCode = LanguageCode(UnitedKingdom);

    pub const GERMANY: LanguageCode = LanguageCode(Germany);

    pub const SPAIN: LanguageCode = LanguageCode(Spain);

    pub const ITALY: LanguageCode = LanguageCode(Italy);

    pub const FRANCE: LanguageCode = LanguageCode(France);

    pub const JAPAN: LanguageCode = LanguageCode(Japan);

    pub const KOREA: LanguageCode = LanguageCode(Korea);

    pub const MEXICO: LanguageCode = LanguageCode(Mexico);

    pub const ARGENTINA: LanguageCode = LanguageCode(Argentina);

    pub const BRAZIL: LanguageCode = LanguageCode(Brazil);

    pub const UNITED_STATES: LanguageCode = LanguageCode(UnitedStates);

    pub const AUSTRALIA: LanguageCode = LanguageCode(Australia);

    pub const RUSSIA: LanguageCode = LanguageCode(Russia);

    pub const TURKEY: LanguageCode = LanguageCode(Turkey);

    pub const MALAYSIA: LanguageCode = LanguageCode(Malaysia);

    pub const PHILIPINNES: LanguageCode = LanguageCode(RepublicOfThePhilipinnes);

    pub const SINGAPORE: LanguageCode = LanguageCode(Singapore);

    pub const THAILAND: LanguageCode = LanguageCode(Thailand);

    pub const VIETNAM: LanguageCode = LanguageCode(Vietnam);

    pub const INDONESIA: LanguageCode = LanguageCode(Indonesia);

    pub const MALAYSIA_CHINESE: LanguageCode = LanguageCode(MalaysiaChinese);

    pub const CHINA: LanguageCode = LanguageCode(China);

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

#[test]
fn lang_code_returns_correct_lang_string() {
    assert_eq!(LanguageCode::TURKEY, "tr_TR")
}
