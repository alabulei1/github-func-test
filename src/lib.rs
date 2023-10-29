use std::collections::HashMap;

use http_req::request;
use serde::Deserialize;
use serde_json::Value;
use webhook_flows::{create_endpoint, request_handler, send_response};

use chrono::prelude::*;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
    panic!("123");
}

#[request_handler(OPTIONS)]
async fn options(
    _headers: Vec<(String, String)>,
    subpath: String,
    qry: HashMap<String, Value>,
    _body: Vec<u8>,
) {
    send_response(
        200,
        vec![
            (
                String::from("Allow"),
                String::from("OPTIONS, GET, HEAD, POST"),
            ),
            (
                String::from("Access-Control-Allow-Origin"),
                String::from("http://127.0.0.1"),
            ),
            (
                String::from("Access-Control-Allow-Methods"),
                String::from("POST, GET, OPTIONS"),
            ),
        ],
        vec![],
    )
}

#[request_handler(GET, POST)]
async fn handler(
    _headers: Vec<(String, String)>,
    subpath: String,
    qry: HashMap<String, Value>,
    _body: Vec<u8>,
) {
    flowsnet_platform_sdk::logger::init();
    let local: DateTime<Local> = Local::now();

    log::debug!("----- before request: {}", local);
    let city = qry.get("city").unwrap_or(&Value::Null).as_str();
    let resp = match city {
        Some(c) => get_weather(c).map(|w| {
            format!(
                "Today: {},
Low temperature: {} °C,
High temperature: {} °C,
Wind Speed: {} km/h",
                w.weather
                    .first()
                    .unwrap_or(&Weather {
                        main: "Unknown".to_string()
                    })
                    .main,
                w.main.temp_min as i32,
                w.main.temp_max as i32,
                w.wind.speed as i32
            )
        }),
        None => Err(String::from("No city in query")),
    };
    log::debug!("----- after request: {}", local);

    match resp {
        Ok(r) => send_response(
            200,
            vec![(
                String::from("content-type"),
                String::from("text/html; charset=UTF-8"),
            )],
            r.as_bytes().to_vec(),
        ),
        Err(e) => send_response(
            400,
            vec![(
                String::from("content-type"),
                String::from("text/html; charset=UTF-8"),
            )],
            e.as_bytes().to_vec(),
        ),
    }
}

#[derive(Deserialize)]
struct ApiResult {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
}

#[derive(Deserialize)]
struct Weather {
    main: String,
}

#[derive(Deserialize)]
struct Main {
    temp_max: f64,
    temp_min: f64,
}

#[derive(Deserialize)]
struct Wind {
    speed: f64,
}

fn get_weather(city: &str) -> Result<ApiResult, String> {
    let mut writer = Vec::new();
    let api_key = "09a55b004ce2f065b903015e3284de35";
    let query_str = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&units=metric&appid={api_key}"
    );

    request::get(query_str, &mut writer)
        .map_err(|e| e.to_string())
        .and_then(|_| {
            serde_json::from_slice::<ApiResult>(&writer).map_err(|_| {
                "Please check if you've typed the name of your city correctly".to_string()
            })
        })
}
