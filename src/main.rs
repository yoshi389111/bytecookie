use clap::{Parser, ValueEnum};
use std::io::Read;

/// Color mode for output
#[derive(Clone, Debug, ValueEnum)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

/// Command line arguments
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Print your fortune message for today
    #[arg(short, long, default_value = None)]
    user: Option<String>,

    /// Message JSON file path
    #[arg(short, long, env = "BYTE_COOKIES_JSON", hide_env_values = true, default_value = None)]
    json: Option<String>,

    /// Color mode (auto, always, never)
    #[arg(short, long, value_enum, default_value = "auto")]
    color: ColorMode,
}

/// Structure for one message in the JSON
#[derive(Debug, serde::Deserialize)]
struct ByteCookie {
    snippet: String,
    message: String,
}

/// Default embedded message JSON (gzip compressed)
const BYTE_COOKIES_GZ: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bytecookies.json.gz"));

/// Parse and get embedded JSON
fn get_embedded_cookies() -> Vec<ByteCookie> {
    let mut json = String::new();
    let mut decoder = flate2::read::GzDecoder::new(BYTE_COOKIES_GZ);
    decoder
        .read_to_string(&mut json)
        .expect("Failed to decode embedded gzip");
    serde_json::from_str(&json).expect("Embedded JSON parsing failed")
}

/// Read message JSON from specified file
fn get_cookies_from_file(file_path: &str) -> Vec<ByteCookie> {
    let mut file = std::fs::File::open(file_path)
        .unwrap_or_else(|e| panic!("Failed to open file '{}': {}", file_path, e));
    let mut json = String::new();
    file.read_to_string(&mut json)
        .unwrap_or_else(|e| panic!("Failed to read file '{}': {}", file_path, e));
    serde_json::from_str(&json)
        .unwrap_or_else(|e| panic!("JSON parsing failed for '{}': {}", file_path, e))
}

/// Decide random index for message
fn decide_random_index(count: usize) -> usize {
    let mut rng = rand::rng();
    rand::Rng::random_range(&mut rng, 0..count)
}

/// Decide message index from today's date and user name
fn decide_todays_index(count: usize, today: &str, user: &str) -> usize {
    let mut context = md5::Context::new();
    context.consume(user.as_bytes());
    context.consume(b":");
    context.consume(today.as_bytes());
    let digest = context.finalize();
    let num = u32::from_be_bytes(digest.0[0..4].try_into().unwrap());
    (num % count as u32) as usize
}

/// Determine if we should use color based on the argument and environment
fn is_color_enabled(color_arg: &ColorMode) -> bool {
    use std::io::IsTerminal;

    let no_color = std::env::var_os("NO_COLOR").is_some();
    let is_tty = std::io::stdout().is_terminal();
    match color_arg {
        ColorMode::Auto => is_tty && !no_color,
        ColorMode::Always => true,
        ColorMode::Never => false,
    }
}

/// Show fortune message for engineers
fn main() {
    let args = Args::parse();

    let cookies = if let Some(file_path) = args.json {
        get_cookies_from_file(&file_path)
    } else {
        get_embedded_cookies()
    };

    let cookie_index = if let Some(user) = args.user {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        decide_todays_index(cookies.len(), &today, &user)
    } else {
        decide_random_index(cookies.len())
    };

    let cookie = &cookies[cookie_index];

    let (cyan, yellow, reset) = if is_color_enabled(&args.color) {
        ("\x1b[36m", "\x1b[33m", "\x1b[0m")
    } else {
        ("", "", "")
    };
    println!("{}{}{}", cyan, cookie.snippet, reset);
    println!("{}{}{}", yellow, cookie.message, reset);
}
