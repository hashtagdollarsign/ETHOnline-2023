mod de_hasher;


use serde::{Deserialize, Serialize};
use lambda_http::{ service_fn, Error};
use lambda_runtime::LambdaEvent;
use crate::de_hasher::EventLogData;


#[derive(Deserialize, Serialize)]
struct Request {
    hashed_data: String,
}
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: EventLogData,
}

async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let de_hashed_data = de_hasher::decrypt_data(
        event.payload.hashed_data
            .into()
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


