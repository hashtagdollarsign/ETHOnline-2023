use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use serde::ser::StdError;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(PartialEq, Debug)]
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

impl ChangeEvent {
    fn to_string(&self) -> String {
        match self {
            ChangeEvent::Up => "Up".to_string(),
            ChangeEvent::Down => "Down".to_string(),
            ChangeEvent::Left => "Left".to_string(),
            ChangeEvent::Right => "Right".to_string(),
            ChangeEvent::A => "A".to_string(),
            ChangeEvent::B => "B".to_string(),
            ChangeEvent::X => "X".to_string(),
            ChangeEvent::Y => "Y".to_string(),
            ChangeEvent::UnRegisteredMove => "UnRegistered".to_string(),
        }
    }
}



#[derive(PartialEq, Debug)]
pub struct EventLog{
    pub change: ChangeEvent,
    pub time: i64
}

impl Display for EventLog {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"change: {:?}\ntime:{:?}", self.change, self.time )
    }
}

fn current_unix_timestamp() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs() as u32
}

pub fn create_event(event: ChangeEvent) -> Result<EventLog,Box<dyn Error>> {
    let event_log = EventLog {
        change: event,
        time: current_unix_timestamp() as i64,
    };
    Ok(event_log)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_create_event() {
        let event: ChangeEvent = ChangeEvent::Up;

        let result = create_event(event).unwrap();

        assert_eq!(result.change, ChangeEvent::Up);
        assert!(result.time >= std::time::Instant::now() - Duration::from_micros(5)
                ,"Function took longer than expected to run");
    }

}