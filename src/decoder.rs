use byteorder::{BigEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::error::Error;

use crate::header::{Header, CSRC, SSRC};
use crate::version::Version;

pub fn decode<TBuffer>(buffer: TBuffer) -> Result<Header, Box<Error>>
where
    TBuffer: AsRef<[u8]>,
{
    let mut buffer = buffer.as_ref();
    let byte = buffer.read_u8()?;
    let version = Version::try_from(byte & 0x3)?;
    let has_padding = byte & 0x4 > 0;
    let has_extension = byte & 0x8 > 0;
    let csrc_count = byte >> 4;
    let byte = buffer.read_u8()?;
    let has_marker = byte & 0x1 > 0;
    let payload_type = byte >> 1;
    let sequence_number = buffer.read_u16::<BigEndian>()?;
    let timestamp = buffer.read_u32::<BigEndian>()?;
    let ssrc = SSRC(buffer.read_u32::<BigEndian>()?);
    let mut csrcs = Vec::with_capacity(csrc_count as usize);

    for _ in 0..csrc_count {
        csrcs.push(CSRC(buffer.read_u32::<BigEndian>()?));
    }

    Ok(Header {
        csrcs,
        has_extension,
        has_marker,
        has_padding,
        payload_type,
        sequence_number,
        ssrc,
        timestamp,
        version,
    })
}
