use std::{io::{Read, Seek}, sync::Arc};

use uuid::Uuid;

#[derive(Clone)]
pub struct AudioClip {
    uuid: Uuid,
    data: Arc<[u8]>,
}

impl AudioClip {
    pub fn new<R: Read + Seek + Send + Sync + 'static>(mut data: R) -> std::io::Result<Self> {
        let mut buf = vec![];
        data.read_to_end(&mut buf)?;

        let data = buf.into();
        let uuid = Uuid::new_v4();
        return Ok(Self { uuid, data });
    }

    pub(crate) fn data(&self) -> Arc<[u8]> {
        self.data.clone()
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}