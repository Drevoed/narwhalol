use self::Inner::*;
use std::convert::AsRef;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RankedQueue(Inner);

#[derive(Clone, PartialEq, Eq, Hash)]
enum Inner {
    Solo,
    Flex,
    TwistedTreeline,
}

impl RankedQueue {
    /// Solo ranked queue
    pub const SOLO: RankedQueue = RankedQueue(Solo);
    /// Flex ranked queue
    pub const FLEX: RankedQueue = RankedQueue(Flex);
    //TODO mark as deprecated when update that removes TT will be dropped
    /// Twisted Treeline ranked queue (soon will be deprecated)
    pub const TWISTED_TREELINE: RankedQueue = RankedQueue(TwistedTreeline);

    #[inline]
    pub fn as_str(&self) -> &str {
        match self.0 {
            Solo => "RANKED_SOLO_5x5",
            Flex => "RANKED_FLEX_SR",
            TwistedTreeline => "RANKED_FLEX_TT",
        }
    }
}

impl AsRef<str> for RankedQueue {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> PartialEq<&'a RankedQueue> for RankedQueue {
    #[inline]
    fn eq(&self, other: &&'a RankedQueue) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<RankedQueue> for &'a RankedQueue {
    #[inline]
    fn eq(&self, other: &RankedQueue) -> bool {
        *self == other
    }
}

impl PartialEq<str> for RankedQueue {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl<'a> PartialEq<&'a str> for RankedQueue {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<RankedQueue> for str {
    #[inline]
    fn eq(&self, other: &RankedQueue) -> bool {
        self == other.as_ref()
    }
}

impl fmt::Debug for RankedQueue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_ref())
    }
}

impl fmt::Display for RankedQueue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_ref())
    }
}
