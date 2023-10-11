use std::error::Error;
use std::time::Instant;

#[derive(PartialEq, Debug)]
enum ChangeEvent {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
}

#[derive(PartialEq, Debug)]
struct EventLog{
    change: ChangeEvent,
    time: Instant
}

fn create_event(event: ChangeEvent) -> Result<EventLog,Box<dyn Error>> {
    let event_log = EventLog {
        change: event,
        time: std::time::Instant::now(),
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