use byteorder::{BigEndian, ReadBytesExt};
use std::convert::{Infallible, TryFrom};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::Read;

use crate::version::{Version, VersionError};

#[derive(Debug)]
pub struct Header {
    csrcs: Vec<CSRC>,
    extension: Option<Extension>,
    has_marker: bool,
    has_padding: bool,
    payload_type: u8,
    sequence_number: u16,
    ssrc: SSRC,
    timestamp: u32,
    version: Version,
}

#[derive(Clone, Copy, Debug)]
pub struct SSRC(u32);

#[derive(Clone, Copy, Debug)]
pub struct CSRC(u32);

#[derive(Debug)]
pub struct Extension {
    body: Vec<u8>,
    parameters: u16,
}

pub fn decode<TBuffer>(buffer: TBuffer) -> Result<Header, DecodeError>
where
    TBuffer: AsRef<[u8]>,
{
    let mut buffer = buffer.as_ref();

    let byte = buffer.read_u8().map_err(|_| DecodeError::UnexpectedEOF)?;
    let version = Version::try_from(byte & 0x3)?;

    if !version.is_rtp2() {
        return Err(DecodeError::UnsupportedVersion);
    }

    let has_padding = byte & 0x4 > 0;
    let has_extension = byte & 0x8 > 0;
    let csrc_count = byte >> 4;
    let byte = buffer.read_u8().map_err(|_| DecodeError::UnexpectedEOF)?;
    let has_marker = byte & 0x1 > 0;
    let payload_type = byte >> 1;
    let sequence_number = buffer
        .read_u16::<BigEndian>()
        .map_err(|_| DecodeError::UnexpectedEOF)?;
    let timestamp = buffer
        .read_u32::<BigEndian>()
        .map_err(|_| DecodeError::UnexpectedEOF)?;
    let ssrc = SSRC(
        buffer
            .read_u32::<BigEndian>()
            .map_err(|_| DecodeError::UnexpectedEOF)?,
    );
    let mut csrcs = Vec::with_capacity(csrc_count as usize);

    for _ in 0..csrc_count {
        let csrc = buffer
            .read_u32::<BigEndian>()
            .map_err(|_| DecodeError::UnexpectedEOF)?;
        csrcs.push(CSRC(csrc));
    }

    let extension = if has_extension {
        let parameters = buffer
            .read_u16::<BigEndian>()
            .map_err(|_| DecodeError::UnexpectedEOF)?;

        // The length defined in the header is the number of 32-bit words in the extension, so
        // multiply by 4 to get the number of bytes.
        let length = buffer
            .read_u16::<BigEndian>()
            .map_err(|_| DecodeError::UnexpectedEOF)?;
        let length = 4 * (length as usize);
        let mut body = Vec::with_capacity(length);
        buffer
            .read_exact(&mut body)
            .map_err(|_| DecodeError::UnexpectedEOF)?;
        Some(Extension { body, parameters })
    } else {
        None
    };

    Ok(Header {
        csrcs,
        extension,
        has_marker,
        has_padding,
        payload_type,
        sequence_number,
        ssrc,
        timestamp,
        version,
    })
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DecodeError {
    Version(VersionError),
    UnexpectedEOF,
    UnsupportedVersion,
}

impl Display for DecodeError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        use self::DecodeError::*;

        match self {
            Version(error) => error.fmt(formatter),
            UnexpectedEOF => write!(formatter, "unexpected EOF"),
            UnsupportedVersion => write!(formatter, "unsupported version"),
        }
    }
}

impl Error for DecodeError {}

impl From<Infallible> for DecodeError {
    fn from(_: Infallible) -> Self {
        DecodeError::UnsupportedVersion
    }
}

impl From<VersionError> for DecodeError {
    fn from(value: VersionError) -> Self {
        DecodeError::Version(value)
    }
}
