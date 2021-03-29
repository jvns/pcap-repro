use pcap::{Capture, Error, Packet};
use pcap::stream::PacketCodec;

struct PrintCodec {}

impl PacketCodec for PrintCodec {
    type Type = ();

    fn decode(&mut self, _packet: Packet) -> Result<(), Error> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let cap = Capture::from_device("any")
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap()
        .setnonblock()
        .unwrap();
    cap.stream(PrintCodec {}).unwrap();
}
