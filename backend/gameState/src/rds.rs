
use std::env;
use crate::events::EventLog;
use tokio_postgres::{NoTls, Error};

pub async fn insert_into_rds(event: &EventLog) -> Result<(), Error> {

    let db_name = env::var("DB_NAME").unwrap();
    let username = env::var("USER_NAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    let connection_string = env::var("DB_ENDPOINT").unwrap();

    let sql_stmnt =
        "INSERT INTO raw_events (event, timestamp) values ($1,$2);";

    let connection_url = format!("postgresql://{username}:{password}@{connection_string}");
    let (client, connection) = tokio_postgres::connect(&connection_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}",e)
        }
    });


    client.execute(
        sql_stmnt,
        &[&event.change.to_string(), &event.time.to_string()]
    );

    Ok(())
}
