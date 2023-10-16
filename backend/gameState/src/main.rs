mod events;
mod rds;

use lambda_http::{ service_fn, Error};
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Request {
    game_move: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {

    let state_change = match event.payload.game_move.as_str() {
        "Up" => events::ChangeEvent::Up,
        "Down" => events::ChangeEvent::Down,
        "Left" => events::ChangeEvent::Down,
        "Right" => events::ChangeEvent::Down,
        "A" => events::ChangeEvent::Down,
        "B" => events::ChangeEvent::Down,
        "X" => events::ChangeEvent::Down,
        "Y" => events::ChangeEvent::Down,
        _ => events::ChangeEvent::UnRegisteredMove,
    };

    let record  = events::create_event(state_change)
        .expect("Attach a timestamp to event");


    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {} executed.", record),
    };
    Ok(resp)
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
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
