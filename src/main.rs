use std::env::{self, set_var};

use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");

    // curl -v -H 'Content-Type: application/json' -H "api-key: $NEW_RELIC_LICENSE_KEY" -X POST https://otlp.nr-data.net/v1/traces
    let client = reqwest::Client::new();

    let response = client
        .post(env::var("NEW_RELIC_OPENTELEMETRY_ENDPOINT")?)
        .header("Content-Type", "application/json")
        .header("api-key", env::var("NEW_RELIC_LICENSE_KEY")?)
        .send()
        .await?;
    let bytes = response.bytes().await?.to_vec();
    let message = String::from_utf8(bytes)?;
    // let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
    set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://localhost:4317");
    set_var("OTEL_SERVICE_NAME", "newrelic-otlp-issue");
    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    run(service_fn(function_handler)).await
}
