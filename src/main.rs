extern crate byteorder;

mod core;
use core::messages::ping::PingMessageBuilder;
use core::messages::{MessageBase, MessageClasses, DgramReader};

fn main() {
    let msg = PingMessageBuilder::new()
        .base(MessageBase {
            connect_id: 32133,
            client_id: 12345,
        })
        .ping_id(13)
        .download_rate(33333)
        .upload_rate(66666)
        .dgram();

    let msg2 = PingMessageBuilder::parse(msg);

    println!("{}", msg2.download_rate.to_string());
}