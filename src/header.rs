use crate::version::Version;

#[derive(Debug)]
pub struct Header {
    pub(crate) csrcs: Vec<CSRC>,
    pub(crate) extension: Option<Extension>,
    pub(crate) has_marker: bool,
    pub(crate) has_padding: bool,
    pub(crate) payload_type: u8,
    pub(crate) sequence_number: u16,
    pub(crate) ssrc: SSRC,
    pub(crate) timestamp: u32,
    pub(crate) version: Version,
}

#[derive(Debug)]
pub struct SSRC(pub(crate) u32);

#[derive(Debug)]
pub struct CSRC(pub(crate) u32);

#[derive(Debug)]
pub struct Extension {
    pub(crate) body: Vec<u8>,
    pub(crate) parameters: u16,
}
