use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;




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
fn main() {
    let data = retrieve_data().expect("Failed to get data");
    let c_data = encrypt_data(data).expect("Failed to compress data");
    println!("{:?}", &c_data);
    decrypt_data(c_data).unwrap();
}


fn decrypt_data(compressed_data: Vec<u8>) -> Result<EventLogData, Box<dyn Error>> {
    let mut decompressed_data = Vec::new();
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    decoder.read_to_end(&mut decompressed_data).expect("Decompression failed");

    // Step 4: Deserialize the data from binary format
    let deserialized_data: EventLogData = bincode::deserialize(&decompressed_data).expect("Deserialization failed");

    // 'deserialized_data' now contains the original data
    Ok(deserialized_data)
}

fn encrypt_data(event_data: EventLogData) -> Result<Vec<u8>, Box<dyn Error>> {
    let serialized_data = bincode::serialize(&event_data).expect("Serialization failed");

    // Create an empty vector to store the compressed data
    let mut compressed_data = Vec::new();

    // Step 2: Compress the binary data and store it in 'compressed_data'
    {
        let mut encoder = ZlibEncoder::new(&mut compressed_data, Compression::default());
        encoder.write_all(&serialized_data).expect("Compression failed");
    } // 'encoder' goes out of scope here, and 'compressed_data' contains the compressed data

    Ok(compressed_data)
}

fn retrieve_data()
-> Result<EventLogData, Box<dyn Error>>
{
    let event_vec = json_retrieve()
        .unwrap();
    Ok(event_vec)
}

fn json_retrieve() -> Result<EventLogData, Box<dyn Error>> {

    let mut file = File::open("./mock_data.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let data: EventLogData = serde_json::from_str(&data).expect("JSON was not well-formatted");

    Ok(data)
}



#[cfg(test)]
mod tests {

    use super::*;
    // #[test]
    // #[ignore]
    // fn test_retrieve_data() {
    //
    //     assert_eq!(retrieve_data().unwrap(), ());
    // }

    #[test]
    fn test_json_retrieve() {

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

        let json_data = json_retrieve();

        assert_eq!(json_data.unwrap(), expected_data);
    }

    #[test]
    fn test_encryption() {

        let event_data = EventLogData {
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

        let expected_data = vec![120, 156, 99, 98, 64, 128, 61, 75, 180, 82, 65, 180, 4, 155, 142, 0, 11, 144, 254, 178, 20, 194, 127, 81, 100, 32, 0, 0, 88, 77, 6, 18];

        let json_data = encrypt_data(event_data);

        assert_eq!(json_data.unwrap(), expected_data);
    }

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

