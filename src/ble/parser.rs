const BTHOME_HEADER_MASK: u8 = 0b1110_0000;
const BTHOME_HEADER_VALUE: u8 = 0b0100_0000;

const PACKET_COUNTER_ID: u8 = 0x00;
const BATTERY_ID: u8 = 0x01;
const TEMPERATURE_ID: u8 = 0x02;
const HUMIDITY_ID: u8 = 0x03;

#[derive(Debug, Clone, Default)]
pub struct BthomeSample {
    pub packet_counter: Option<u8>,
    pub battery: Option<u8>,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
}

pub fn parse_bthome_service_data(data: &[u8]) -> Option<BthomeSample> {
    if data.is_empty() {
        return None;
    }

    let header = data[0];
    if header & BTHOME_HEADER_MASK != BTHOME_HEADER_VALUE {
        return None;
    }

    let mut i = 1usize;
    let mut s = BthomeSample::default();

    if i + 1 < data.len() && data[i] == PACKET_COUNTER_ID {
        s.packet_counter = Some(data[i + 1]);
        i += 2;
    }

    while i < data.len() {
        let id = data[i];
        i += 1;

        match id {
            BATTERY_ID => {
                if i < data.len() {
                    s.battery = Some(data[i]);
                    i += 1;
                }
            }
            TEMPERATURE_ID => {
                if i + 2 <= data.len() {
                    let raw = i16::from_le_bytes([data[i], data[i + 1]]);
                    s.temperature = Some(raw as f32 / 100.0);
                    i += 2;
                }
            }
            HUMIDITY_ID => {
                if i + 2 <= data.len() {
                    let raw = u16::from_le_bytes([data[i], data[i + 1]]);
                    s.humidity = Some(raw as f32 / 100.0);
                    i += 2;
                }
            }
            _ => return None,
        }
    }

    Some(s)
}
