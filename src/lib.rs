use cloud_vision_flows::text_detection;
use flowsnet_platform_sdk::logger;
use flowsnet_platform_sdk::{debug, error, info, warn};
use lambda_flows::{request_received, send_response};

#[no_mangle]
pub fn run() {
    logger::init();
    request_received(|_qry, body| {
        debug!("debug \ndebug");
        info!("info \ninfo");
        warn!("warn \nwarn");
        error!("error \nerror");
        let text = text_detection(String::from_utf8(body).unwrap());
        send_response(
            200,
            vec![(
                String::from("content-type"),
                String::from("text/plain; charset=UTF-8"),
            )],
            text.unwrap().as_bytes().to_vec(),
        );
    });
}
