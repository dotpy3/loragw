//! Bindings to retrieve and push packets from the concentrator.

#[repr(C)]
pub struct LoRaGWTxPacket {
    Frequency: u32,
    TxMode: u8,
    Timestamp: u32,
    RFChain: u8,
    RFPower: i8,
    Modulation: u8,
    Bandwidth: u8,
    DataRate: u32,
    CodeRate: u8,
    InvertPolarization: bool,
    FrequencyDeviation: u8,
    Preamble: u16,
    NoCRC: bool,
    NoHeader: bool,
    Size: u16,
    Payload: [u8; 256]
}

#[link(name = "libloragw")]
extern {
    fn lgw_send(packet: LoRaGWTxPacket) -> i16;
    fn lgw_receive(max_packets: u8, packet: &LoRaGWRxPacket) -> i16;
}

pub fn AddToBuffer(packet: LoRaGWTxPacket) -> bool {
    return true;
}

#[repr(C)]
pub struct LoRaGWRxPacket {
    Frequency: u32,
    IFChain: u8,
    Status: u8,
    Timestamp: u32,
    RFChain: u8,
    Modulation: u8,
    Bandwidth: u8,
    DataRate: u32,
    CodeRate: u8,
    RSSI: f32,
    SNRAverage: f32,
    SNRMin: f32,
    SNRMax: f32,
    CRC: u16,
    Size: u16,
    Payload: [u8; 256]
}

pub fn Poll() -> Option<LoRaGWRxPacket> {
    return None;
}
