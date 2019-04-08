use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Version {
    RTP0,
    RTP1,
    RTP2,
}

impl TryFrom<u8> for Version {
    type Error = VersionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use self::Version::*;

        match value {
            0 => Ok(RTP0),
            1 => Ok(RTP1),
            2 => Ok(RTP2),
            _ => Err(VersionError),
        }
    }
}

#[derive(Debug)]
pub struct VersionError;

impl Display for VersionError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "version error")
    }
}

impl Error for VersionError {}
