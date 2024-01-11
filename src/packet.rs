use bilge::prelude::*;
use defmt::Format;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

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

// A response packet
#[bitsize(8)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct ResponsePacket {
    pub packet_type: PacketType,
    pub resv: u4,
}

impl Format for ResponsePacket {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ResponsePacket {{ packet_type: {:?}, resv: {:#x} }}",
            self.packet_type(),
            self.resv().value(),
        )
    }
}

// DW3000 40-bit timestamp
#[derive(Debug, Format, Copy, Clone, PartialEq, FromZeroes, FromBytes, AsBytes)]
#[repr(packed)]
pub struct DeviceTimestamp {
    pub bytes: [u8; 5],
}

impl DeviceTimestamp {
    pub fn new(timestamp: u40) -> Self {
        Self {
            bytes: timestamp.to_le_bytes(),
        }
    }

    pub fn value(&self) -> u40 {
        u40::from_le_bytes(self.bytes)
    }
}

// Packet Header
#[bitsize(8)]
#[derive(FromBits, DebugBits, PartialEq)]
pub struct PacketHeader {
    pub packet_type: PacketType,
    pub resv: u4,
}

// Final Packet
#[derive(Debug, Format, Clone, Copy, PartialEq, FromZeroes, FromBytes, AsBytes)]
#[repr(packed)]
pub struct FinalPacket {
    pub header_byte: u8,
    pub rx_timestamps: [DeviceTimestamp; 3],
    pub tx_timestamp: DeviceTimestamp,
}

/// The Final Packet
impl FinalPacket {
    pub fn new(packet_type: PacketType, resv: u4, rx_timestamps: [u40; 3], tx_timestamp: u40) -> Self {
        Self {
            header_byte: PacketHeader::new(packet_type, resv).value,
            rx_timestamps: [
                DeviceTimestamp::new(rx_timestamps[0]),
                DeviceTimestamp::new(rx_timestamps[1]),
                DeviceTimestamp::new(rx_timestamps[2]),
            ],
            tx_timestamp: DeviceTimestamp::new(tx_timestamp),
        }
    }

    pub fn header(&self) -> PacketHeader {
        PacketHeader::from(self.header_byte)
    }
}

/// Packet Type
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

    use zerocopy::{AsBytes, transmute};

    #[test]
    fn test_poll_packet() {
        let poll_packet =
            PollPacket::new(PacketType::Poll, u4::new(0), u40::new(0x12356789).into());

        let poll_packet_bytes = poll_packet.value.to_le_bytes();

        assert_eq!(poll_packet_bytes, [0x00, 0x89, 0x67, 0x35, 0x12, 0x00]);
    }

    #[test]
    fn test_response_packet() {
        let response_packet = ResponsePacket::new(PacketType::Response, u4::new(0));

        let response_packet_bytes = response_packet.value.to_le_bytes();

        assert_eq!(response_packet_bytes, [0x1]);
    }

    #[test]
    fn test_final_packet() {
        let final_packet = FinalPacket::new(
            PacketType::Final,
            u4::new(0),
            [
                u40::new(0x12356789).into(),
                u40::new(0x12356789).into(),
                u40::new(0x12356789).into(),
            ],
            u40::new(0xDEADBEEF).into(),
        );

        let final_packet_bytes = final_packet.as_bytes();
        let mut ts_bytes: [u8; 5] = [0; 5];
        ts_bytes.copy_from_slice(&(0x12356789u64.to_le_bytes()[..5]));

        assert_eq!(
            final_packet_bytes,
            [
                0x02, 0x89, 0x67, 0x35, 0x12, 0x00, 0x89, 0x67, 0x35, 0x12, 0x00, 0x89, 0x67, 0x35,
                0x12, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x00
            ]
        );

        assert_eq!(final_packet_bytes[1..6], ts_bytes);

        let mut some_bytes: [u8; 21] = [0; 21];

        // copy from final_packet_bytes
        some_bytes.copy_from_slice(&final_packet_bytes[..]);

        let transmuted: FinalPacket = transmute!(some_bytes);

        assert_eq!(transmuted, final_packet);
    }

    #[test]
    fn test_device_timestamp() {
        let dt = DeviceTimestamp::new(u40::new(0x12356789).into());

        assert_eq!(dt.bytes, [0x89, 0x67, 0x35, 0x12, 0x00]);
        assert_eq!(core::mem::size_of::<DeviceTimestamp>(), 5);
    }
}
