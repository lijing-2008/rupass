use bytes::{Bytes, BytesMut};
use prost::Message;

pub use pwd::*;

mod pwd;

impl PwdKey {
    pub fn new(url: String, user: String) -> Self {
        Self {
            url,
            user,
        }
    }
}


impl PwdInfo {
    pub fn new(password: String, notes: String) -> Self {
        Self {
            password,
            notes,
        }
    }
}


impl From<PwdKey> for Bytes {
    fn from(key: PwdKey) -> Self {
        let mut buf = BytesMut::new();
        key.encode(&mut buf).unwrap();
        buf.freeze()
    }
}

impl From<PwdInfo> for Bytes {
    fn from(info: PwdInfo) -> Self {
        let mut buf = BytesMut::new();
        info.encode(&mut buf).unwrap();
        buf.freeze()
    }
}

impl TryFrom<BytesMut> for PwdKey {
    type Error = prost::DecodeError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(value)
    }
}
impl TryFrom<BytesMut> for PwdInfo {
    type Error = prost::DecodeError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Message::decode(value)
    }
}
