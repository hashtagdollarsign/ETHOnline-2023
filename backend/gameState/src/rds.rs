
use std::env;
use crate::events::EventLog;
use tokio_postgres::{NoTls, Error,Config};

pub async fn insert_into_rds(event: &EventLog) -> Result<(), Error> {

    let db_name = env::var("DB_NAME").unwrap();
    let username = env::var("USER_NAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    let db_endpoint = env::var("DB_ENDPOINT").unwrap();



    let connection_url = format!("postgresql://{username}:{password}@{db_endpoint}");
    let config = format!("user={username} password={password} host={db_endpoint} port=5432 dbname={db_name} sslmode=require");



    let (client, connection) = tokio_postgres::connect(&config, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}",e)
        }
    });


    let sql_stmnt =
        "INSERT INTO raw_events (event, timestamp) values ($1,$2);";

    let result = client.execute(
        sql_stmnt,
        &[&event.change.to_string(), &event.time.to_string()]
    ).await?;



    Ok(())
}
