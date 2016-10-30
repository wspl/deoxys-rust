use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;

pub mod ping;

//pub const MAX_DGRAM_SIZE: usize = 1500;
pub const VERSION: i16 = 2;

#[derive(Clone, Copy, Debug)]
pub struct MessageBase {
    pub connect_id: i32,
    pub client_id: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum MessageClasses {
    PingMessage = 0,
    PingMessage2 = 1,
    DataMessage = 2,
    AckListMessage = 3,
    CloseStreamMessage = 4,
    CloseConnMessage = 5,
    Unknown = -1
}

struct DgramBuilder {
    dgram: Vec<u8>
}

impl DgramBuilder {
    fn new() -> DgramBuilder {
        DgramBuilder {
            dgram: vec![]
        }
    }

    fn write_i16(&mut self, data: i16, pos: usize) -> &mut DgramBuilder {
        let mut fragm = vec![];
        fragm.write_i16::<BigEndian>(data);

        let mut relative_pos = 0;
        for byte in fragm {
            self.dgram[pos + relative_pos] = byte;
            relative_pos += 1;
        }
        self
    }

    fn write_i32(&mut self, data: i32, pos: usize) -> &mut DgramBuilder {
        let mut fragm = vec![];
        fragm.write_i32::<BigEndian>(data);

        let mut relative_pos = 0;
        for byte in fragm {
            self.dgram[pos + relative_pos] = byte;
            relative_pos += 1;
        }
        self
    }

    fn push_i16(&mut self, data: i16) -> &mut DgramBuilder {
        let mut fragm = vec![];
        fragm.write_i16::<BigEndian>(data);

        self.dgram.append(&mut fragm);
        self
    }

    fn push_i32(&mut self, data: i32) -> &mut DgramBuilder {
        let mut fragm = vec![];
        fragm.write_i32::<BigEndian>(data);

        self.dgram.append(&mut fragm);
        self
    }

    fn init_header(&mut self, base: MessageBase, class: MessageClasses) -> &mut DgramBuilder {
        self.push_i16(VERSION)
            .push_i16(class as i16)
            .push_i32(base.connect_id)
            .push_i32(base.client_id)
    }

    fn finalize(&self) -> Vec<u8> {
        self.dgram.clone()
    }
}

pub struct DgramReader {
    dgram: Vec<u8>
}

impl DgramReader {
    fn new(dgram: Vec<u8>) -> DgramReader {
        DgramReader {
            dgram: dgram
        }
    }

    fn i32(&self, pos: usize) -> i32 {
        let mut fragm = vec![0u8; 4];
        for i in 0..4 {
            fragm[i] = self.dgram[pos + i]
        }
        let mut rdr = Cursor::new(fragm);
        rdr.read_i32::<BigEndian>().unwrap()
    }

    fn i16(&self, pos: usize) -> i16 {
        let mut fragm = vec![0u8; 2];
        for i in 0..2 {
            fragm[i] = self.dgram[pos + i]
        }
        let mut rdr = Cursor::new(fragm);
        rdr.read_i16::<BigEndian>().unwrap()
    }

    fn base(&self) -> MessageBase {
        MessageBase {
            connect_id: self.i32(4),
            client_id: self.i32(8),
        }
    }

    fn version(&self) -> i16 {
        self.i16(0)
    }

    fn class(&self) -> MessageClasses {
        match self.i16(2) {
            0 => MessageClasses::PingMessage,
            1 => MessageClasses::PingMessage2,
            2 => MessageClasses::DataMessage,
            3 => MessageClasses::AckListMessage,
            4 => MessageClasses::CloseStreamMessage,
            5 => MessageClasses::CloseConnMessage,
            _ => MessageClasses::Unknown
        }
    }

    fn read_class(dgram: Vec<u8>) -> MessageClasses {
        let reader = DgramReader {
            dgram: dgram
        };
        reader.class()
    }
}