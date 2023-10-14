use std::time::Duration;
use flate2::read::ZlibDecoder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Read;
use std::io::Write;
use std::fmt::Formatter;
use std::error::Error;

#[derive(PartialEq, Debug, Deserialize, Serialize, Hash)]
pub enum ChangeEvent {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    X,
    Y,
    UnRegisteredMove,
}


#[derive(PartialEq, Debug, Deserialize, Serialize, Hash)]
pub struct EventLog{
    change: ChangeEvent,
    time: Duration
}

#[derive(Deserialize, Debug, PartialEq, Serialize, Hash)]
pub struct EventLogData{
    data: Vec<EventLog>
}
impl Display for EventLog {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"change: {:?}\ntime:{:?}", self.change, self.time )
    }
}

pub fn decrypt_data(compressed_data: Vec<u8>) -> Result<EventLogData, Box<dyn Error>> {
    let mut decompressed_data = Vec::new();
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    decoder.read_to_end(&mut decompressed_data).expect("Decompression failed");

    // Step 4: Deserialize the data from binary format
    let deserialized_data: EventLogData = bincode::deserialize(&decompressed_data).expect("Deserialization failed");

    // 'deserialized_data' now contains the original data
    Ok(deserialized_data)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decryption() {

        let expected_data = EventLogData {
            data: vec![
                EventLog {
                    change: ChangeEvent::Up,
                    time: Duration::new(1697293500, 271320600)
                },
                EventLog {
                    change: ChangeEvent::A,
                    time: Duration::new(1697293812, 271610600)
                },
            ],

        };

        let json_data = decrypt_data(vec![120, 156, 99, 98, 64, 128, 61, 75, 180, 82, 65, 180, 4, 155, 142, 0, 11, 144, 254, 178, 20, 194, 127, 81, 100, 32, 0, 0, 88, 77, 6, 18]);

        assert_eq!(json_data.unwrap(), expected_data);
    }

}