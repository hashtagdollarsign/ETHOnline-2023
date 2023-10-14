use std::fs::File;
use std::fmt::Formatter;
use std::time::Duration;
use serde::{Deserialize,Serialize};
use std::fmt::Display;
use std::error::Error;
use std::io::Read;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
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


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EventLog{
    change: ChangeEvent,
    time: Duration
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EventLogData{
    data: Vec<EventLog>
}
impl Display for EventLog {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"change: {:?}\ntime:{:?}", self.change, self.time )
    }
}

// This will have to be changed to grab from RDS
pub fn json_retrieve() -> Result<EventLogData, Box<dyn Error>> {

    let mut file = File::open("./mock_data.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let data: EventLogData = serde_json::from_str(&data).expect("JSON was not well-formatted");

    Ok(data)
}



#[cfg(test)]
mod tests {
    use super::*;
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
}