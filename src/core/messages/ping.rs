use super::{VERSION, MessageBase, MessageClasses, DgramBuilder, DgramReader};

pub struct PingMessageBuilder {
    pub base: MessageBase,
    pub ping_id: i32,
    pub download_rate: i32,
    pub upload_rate: i32,
}

impl PingMessageBuilder {
    pub fn new() -> PingMessageBuilder {
        PingMessageBuilder {
            base: MessageBase {
                connect_id: 0,
                client_id: 0,
            },
            ping_id: 0,
            download_rate: 0,
            upload_rate: 0,
        }
    }

    pub fn parse(dgram: Vec<u8>) -> PingMessageBuilder {
        let reader = DgramReader::new(dgram);
        PingMessageBuilder {
            base: reader.base(),
            ping_id: reader.i32(12),
            download_rate: reader.i32(16),
            upload_rate: reader.i32(20),
        }
    }

    pub fn base(&mut self, base: MessageBase) -> &mut PingMessageBuilder {
        self.base = base.clone();
        self
    }

    pub fn ping_id(&mut self, id: i32) -> &mut PingMessageBuilder {
        self.ping_id = id;
        self
    }

    pub fn download_rate(&mut self, rate: i32) -> &mut PingMessageBuilder {
        self.download_rate = rate;
        self
    }

    pub fn upload_rate(&mut self, rate: i32) -> &mut PingMessageBuilder {
        self.upload_rate = rate;
        self
    }

    pub fn dgram(&self) -> Vec<u8> {
        DgramBuilder::new()
            .init_header(self.base, MessageClasses::PingMessage)
            .push_i32(self.ping_id)
            .push_i32(self.download_rate)
            .push_i32(self.upload_rate)
            .finalize()
    }
}