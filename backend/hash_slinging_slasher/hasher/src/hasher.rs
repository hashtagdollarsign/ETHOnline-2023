
use flate2::Compression;
use crate::retriever::EventLogData;
use std::error::Error;
use std::io::Write;
use flate2::write::ZlibEncoder;


pub fn encrypt_data(event_data: EventLogData) -> Result<Vec<u8>, Box<dyn Error>> {
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



#[cfg(test)]
mod tests {
    use super::*;

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
}