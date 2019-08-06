use self::Inner::*;

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
    pub const CzechRepublic: LanguageCode = LanguageCode(CzechRepublic);
    /// Greece language code
    pub const Greece: LanguageCode = LanguageCode(Greece);
}
