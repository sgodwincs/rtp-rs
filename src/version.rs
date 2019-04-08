use std::convert::{Infallible, TryFrom};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Version {
    RTP0,
    RTP1,
    RTP2,
}

impl Version {
    pub fn is_rtp0(self) -> bool {
        use self::Version::*;

        match self {
            RTP0 => true,
            _ => false,
        }
    }

    pub fn is_rtp1(self) -> bool {
        use self::Version::*;

        match self {
            RTP1 => true,
            _ => false,
        }
    }

    pub fn is_rtp2(self) -> bool {
        use self::Version::*;

        match self {
            RTP2 => true,
            _ => false,
        }
    }
}

impl TryFrom<u8> for Version {
    type Error = VersionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use self::Version::*;

        match value {
            0 => Ok(RTP0),
            1 => Ok(RTP1),
            2 => Ok(RTP2),
            3 => Err(VersionError::Unknown),
            _ => Err(VersionError::Invalid),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum VersionError {
    Invalid,
    Unknown,
}

impl Display for VersionError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        use self::VersionError::*;

        match self {
            Invalid => write!(formatter, "invalid version"),
            Unknown => write!(formatter, "unknown version"),
        }
    }
}

impl Error for VersionError {}

impl From<Infallible> for VersionError {
    fn from(_: Infallible) -> Self {
        VersionError::Invalid
    }
}
