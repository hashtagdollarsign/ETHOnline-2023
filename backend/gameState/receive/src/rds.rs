
use std::env;
use chrono::{DateTime, Utc};
use tracing::info;
use crate::events::{ChangeEvent, EventLog};
use tokio_postgres::{NoTls, Error,Config};
use tokio_postgres::types::{ToSql, Type, IsNull};

pub async fn insert_into_rds(event: &EventLog) -> Result<(), Error> {

    let db_name = env::var("DB_NAME").unwrap();
    let username = env::var("USER_NAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    let db_endpoint = env::var("DB_ENDPOINT").unwrap();

    info!("All env accounted for");


    let connection_url = format!("postgresql://{username}:{password}@{db_endpoint}:5432/{db_name}");
    let config = format!("user={username} password={password} host={db_endpoint} port=5432 dbname={db_name}");


    info!("Connecting with: {}", &connection_url);

    let (client, connection) = tokio_postgres::connect(&connection_url, NoTls).await
        .expect("failed here");
    info!("Maybe connected");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}",e)
        }
    });

    info!("After Spawn");


    let sql_stmnt = "INSERT INTO postgres.raw_events.raw_game_events (move, time_in_seconds) values ($1, $2);";

    let change = format!("{:?}",event.change);
    let time = event.time;

    info!("Inserting {} and {} into db", &change,&time);

    let result = client.execute(
        sql_stmnt,
        &[&change, &time]
    ).await;

    info!("{:?}", &result);


    Ok(())
}

