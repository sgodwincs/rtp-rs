use crate::version::Version;

#[derive(Debug)]
pub struct Header {
    pub(crate) csrcs: Vec<u32>,
    pub(crate) has_extension: bool,
    pub(crate) has_marker: bool,
    pub(crate) has_padding: bool,
    pub(crate) payload_type: u8,
    pub(crate) sequence_number: u16,
    pub(crate) ssrc: u32,
    pub(crate) timestamp: u32,
    pub(crate) version: Version,
}
