use bilge::prelude::*;
use defmt::Format;

// A poll packet
#[bitsize(48)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct PollPacket {
    pub packet_type: PacketType,
    pub resv: u4,
    pub tx_timestamp: u40,
}

impl Format for PollPacket {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PollPacket {{ packet_type: {:?}, resv: {:#x}, tx_timestamp: {} }}",
            self.packet_type(),
            self.resv().value(),
            self.tx_timestamp().value()
        )
    }
}

#[bitsize(4)]
#[derive(FromBits, Debug, PartialEq, Format)]
pub enum PacketType {
    Poll = 0,
    Response = 1,
    Final = 2,
    #[fallback]
    Reserved,
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poll_packet() {
        let poll_packet =
            PollPacket::new(PacketType::Poll, u4::new(0), u40::new(0x12356789).into());

        let poll_packet_bytes = poll_packet.value.to_le_bytes();

        assert_eq!(poll_packet_bytes, [0x00, 0x89, 0x67, 0x35, 0x12, 0x00]);
    }
}
