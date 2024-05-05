mod services;

use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    http::{HeaderName, HeaderValue, Method, Request},
    response::Response,
    routing::get,
    Router,
};
use clap::{value_parser, Arg, ArgAction, Command};
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer};
use tower_http::{
    classify::ServerErrorsFailureClass, cors::CorsLayer, propagate_header::PropagateHeaderLayer, request_id::{MakeRequestUuid, SetRequestIdLayer}, trace::TraceLayer
};
use tracing::{info_span, Level, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::services::{handler_404, index, redirect};



fn cmd() -> Command {
    Command::new("amacerels-musings")
        .about("A musings store")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .help("Server address")
                .env("AMACKEREL_SERVER_ADDRESS")
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Server port")
                .env("AMACKEREL_SERVER_PORT")
                .default_value("3000")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("verbosity")
                .short('v')
                .help("Set the log level")
                .required(false)
                .env("AMACKEREL_SERVER_VERBOSITY")
                .action(ArgAction::Count),
        )
}

fn handle_startup_commands() -> (String, String) {
    let matches = cmd().get_matches();

    // unwraps are fine as Clap has validated the inputs already
    let address = matches.get_one::<String>("address").unwrap().to_owned();
    let port = matches.get_one::<u32>("port").unwrap().to_string();
    let log_level = match matches.get_one::<u8>("verbosity").unwrap() {
        0 => Level::INFO,
        1 => Level::DEBUG,
        _ => Level::TRACE,
    };

    setup_tracing(log_level);
    tracing::info!("Server setup complete.");
    tracing::info!("\tServer address: {}", address);
    tracing::info!("\tServer port: {}", port);
    tracing::info!("\tServer log level: {}", log_level.to_string());
    
    (address, port)
}

fn setup_tracing(log_level: Level) {
    // set both package and tower tracing to log level
    let tracing_env_var = format!(
        "{}={},tower_http=debug,axum::rejection=trace",
        env!("CARGO_PKG_NAME").replace("-", "_"),
        log_level.as_str(),
    );

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_env_var.into()),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
}

fn main() {
    let (address, port) = handle_startup_commands();

    run_app(address, port);
}

#[tokio::main]
async fn run_app(address: String, port: String) {

    // rate limiting config. Set to 5 requests max with one request replenishing every 20 seconds.
    // Uses a SmartIpKeyExtractor which tries to identify client IP through a number of options 
    let governor_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(20)
            .burst_size(5)
            .key_extractor(SmartIpKeyExtractor)
            .use_headers()
            .finish()
            .unwrap()
    );


    // TODO: security headers
    // TODO: global 404 handler with Span
    // TODO: error handling and on error request.
    let app = Router::new()
        .route("/", get(index))
        .route("/redirect", get(redirect))
        // .route("/blog-post/:id", get(get_blog_post))
        .layer(
        // add middlewear, this is executed from top to bottom
        ServiceBuilder::new()
            // set `x-request-id` header on all requests and propogate to response
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
            .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
            // set tracing details
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<_>| {
                        let request_id = request
                            .headers()
                            .get("x-request-id")
                            .map(|v| v.to_str().unwrap_or_default())
                            .unwrap_or_default();

                        info_span!(
                            "http_request",
                            request_id = request_id,
                            status_code = tracing::field::Empty,
                            latency_ms = tracing::field::Empty,
                            method = ?request.method(),
                            uri = ?request.uri(),
                            version = ?request.version(),
                            response_headers = tracing::field::Empty,
                            request_headers = tracing::field::Empty,
                        )
                    })
                    .on_request(|request: &Request<_>, span: &Span| {
                        tracing::debug!("Entering span...");
                    })
                    .on_response(|response: &Response, latency: Duration, span: &Span| {
                        let status_code = response.status();
                        let latency_ms = latency.as_millis();
                        span.record("status_code", status_code.as_str());
                        span.record("latency_ms", latency_ms);

                        tracing::debug!("...span ended")
                    })
                    .on_failure(|error: ServerErrorsFailureClass, latency: Duration, span: &Span| {
                        let latency_ms = latency.as_millis();
                        span.record("latency_ms", latency_ms);

                        match error {
                            ServerErrorsFailureClass::StatusCode(code) => {
                                span.record("status_code", code.to_string());
                                tracing::error!("An error has occured with status code: {}", code);
                            },
                            ServerErrorsFailureClass::Error(err) => {
                                tracing::error!("Error occured in span. Error: {:#?}", err);
                            }
                        }
                    },
                ),
            )
            // implement rate limiting based on IP addr (not ideal but better than nothing) 
            .layer(GovernorLayer { config: governor_config })
            // implement CORS handling
            .layer(
                // pay attention that for some request types like posting content-type: application/json
                // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
                // or see this issue https://github.com/tokio-rs/axum/issues/849
                // TODO: SET ACTUAL HOST BASED ON CLI ARGS
                CorsLayer::new()
                    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::GET]),
            ),
    )
    // generic 404 fallback
    .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

