
use std::fmt::Formatter;
use serde::{Deserialize,Serialize};
use std::fmt::Display;
use std::env;
use tokio_postgres::{NoTls, Error};


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

impl ChangeEvent {
    pub fn from_str(input: &str) -> Result<ChangeEvent, String> {
        match input {
            "Up" => Ok(ChangeEvent::Up),
            "Down" => Ok(ChangeEvent::Down),
            "Left" => Ok(ChangeEvent::Left),
            "Right" => Ok(ChangeEvent::Right),
            "A" => Ok(ChangeEvent::A),
            "B" => Ok(ChangeEvent::B),
            "X" => Ok(ChangeEvent::X),
            "Y" => Ok(ChangeEvent::Y),
            "UnRegisteredMove" => Ok(ChangeEvent::UnRegisteredMove),
            _ => Err("Invalid ChangeEvent string".to_string()),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EventLog{
    change: ChangeEvent,
    time: i64
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
pub async fn postgres_retrieve()
    -> Result<EventLogData, Error>
{

    let db_name = env::var("DB_NAME").unwrap();
    let username = env::var("USER_NAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    let db_endpoint = env::var("DB_ENDPOINT").unwrap();

    let connection_url = format!("postgresql://{username}:{password}@{db_endpoint}:5432/{db_name}");

    let (client, connection) = tokio_postgres::connect(&connection_url, NoTls).await
        .expect("failed here");


    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}",e)
        }
    });

    let sql_stmnt = format!("SELECT move, time_in_seconds FROM POSTGRES.RAW_EVENTS.RAW_GAME_EVENTS;");

    let rows = client.query (&sql_stmnt, &[])
        .await?;


    let mut to_be_hashed: Vec<EventLog> = vec![];

    for row in rows {

        let game_move: &str = row.get(0);
        let timeing:i64 = row.get(1) ;


        let event_log = EventLog {
            change: ChangeEvent::from_str(game_move).unwrap(),
            time: timeing,
        };

        to_be_hashed.push(event_log);
    }

    let data = EventLogData {
        data: to_be_hashed,
    };

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