mod de_hasher;

use serde::{Deserialize, Serialize};
use lambda_http::{ service_fn, Error};
use lambda_runtime::LambdaEvent;
use crate::de_hasher::EventLogData;
use base64::{Engine as _, engine::{self, general_purpose}};

#[derive(Deserialize, Serialize)]
struct Request {
    hashed_data: String,
}

#[derive(Deserialize, Serialize)]
struct RequestBinary {
    hashed_data: Vec<u8>,
}

impl Request {
    fn to_binary(&self) -> Result<Vec<u8>, base64::DecodeError> {
        let decoded_data = general_purpose::STANDARD
            .decode(&self.hashed_data).expect("failed to convert string to binary.");
        Ok(decoded_data)
    }
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: EventLogData,
}

async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {

    let converted_data = event.payload.to_binary().unwrap();
    let de_hashed_data = de_hasher::decrypt_data(
            converted_data
    )
        .expect("Failed to decrypt data");
    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: de_hashed_data,
    };
    Ok(resp)
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


