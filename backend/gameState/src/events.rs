use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use chrono::{DateTime, Utc};

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

impl Display for ChangeEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self )
    }
}


#[derive(PartialEq, Debug)]
pub struct EventLog{
    pub change: ChangeEvent,
    pub time: DateTime<Utc>
}

impl Display for EventLog {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"change: {:?}\ntime:{:?}", self.change, self.time )
    }
}

pub fn create_event(event: ChangeEvent) -> Result<EventLog,Box<dyn Error>> {
    let event_log = EventLog {
        change: event,
        time: Utc::now(),
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