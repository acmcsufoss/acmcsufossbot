use acmcsufossbot::http;
use acmcsufossbot::env::Config;

fn main() {
    let config = Config::get_env();
    http::serve(config);
}
