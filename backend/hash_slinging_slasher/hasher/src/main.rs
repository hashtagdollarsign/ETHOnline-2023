mod hasher;
mod retriever;

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Read;
use std::io::Write;
use lambda_http::{service_fn, Error};
use lambda_runtime::LambdaEvent;

use crate::retriever::EventLogData;


#[derive(Serialize, Deserialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[derive(Deserialize, Serialize)]
struct Request {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let data = retriever::postgres_retrieve().await.expect("Could not grab data");
    let hashed_data = hasher::encrypt_data(data).unwrap();

    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("{:?}", hashed_data)
    };
    Ok(resp)
}
