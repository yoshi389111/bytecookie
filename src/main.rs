use clap::Parser;
use serde::Deserialize;
use std::io::Read;

const BYTE_COOKIES_GZ: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bytecookies.json.gz"));

#[derive(Debug, Deserialize)]
struct ByteCookie {
    snippet: String,
    message: String,
}

fn get_embedded_cookies() -> Vec<ByteCookie> {
    let mut json = String::new();
    let mut decoder = flate2::read::GzDecoder::new(BYTE_COOKIES_GZ);
    decoder
        .read_to_string(&mut json)
        .expect("Failed to decode gzip");
    serde_json::from_str(&json).expect("JSON parsing failed")
}

fn decide_random_index(count: usize) -> usize {
    let mut rng = rand::rng();
    rand::Rng::random_range(&mut rng, 0..count)
}

fn decide_todays_index(count: usize, date: &str, user: &str) -> usize {
    let mut context = md5::Context::new();
    context.consume(user.as_bytes());
    context.consume(b":");
    context.consume(date.as_bytes());
    let digest = context.finalize();
    let num = u32::from_be_bytes(digest.0[0..4].try_into().unwrap());
    (num as usize) % count
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Print your message today
    #[arg(short, long, default_value_t = false)]
    today: bool,
}

fn main() {
    let args = Args::parse();

    let byte_cookies = get_embedded_cookies();

    let index = if args.today {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let username = whoami::username();
        decide_todays_index(byte_cookies.len(), &today, &username)
    } else {
        decide_random_index(byte_cookies.len())
    };

    let cookie = &byte_cookies[index];
    println!("{}", cookie.snippet);
    println!("{}", cookie.message);
}
