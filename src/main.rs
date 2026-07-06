use anyhow::Context;
use clap::Parser;
use std::io::Read;

/// Color mode for output
#[derive(Clone, Debug, clap::ValueEnum)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

/// Command line arguments
#[derive(Debug, clap::Parser)]
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
static BYTE_COOKIES_GZ: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bytecookies.json.gz"));

/// Show fortune message for engineers
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let cookies = if let Some(file_path) = args.json {
        get_cookies_from_file(&file_path)?
    } else {
        get_embedded_cookies()?
    };

    if cookies.is_empty() {
        return Err(anyhow::anyhow!("No fortune messages available (cookie list is empty)"));
    }

    let cookie_index = if let Some(user) = args.user {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        decide_todays_index(cookies.len(), &today, &user)?
    } else {
        decide_random_index(cookies.len())
    };

    let ByteCookie { snippet, message } = &cookies[cookie_index];

    let (cyan, yellow, reset) = if is_color_enabled(&args.color) {
        ("\x1b[36m", "\x1b[33m", "\x1b[0m")
    } else {
        ("", "", "")
    };
    println!("{cyan}{snippet}{reset}");
    println!("{yellow}{message}{reset}");

    Ok(())
}

/// Parse and get embedded JSON
fn get_embedded_cookies() -> anyhow::Result<Vec<ByteCookie>> {
    let mut json = String::new();
    let mut decoder = flate2::read::GzDecoder::new(BYTE_COOKIES_GZ);
    decoder
        .read_to_string(&mut json)
        .with_context(|| "Failed to decompress embedded JSON")?;

    let result = serde_json::from_str(&json).with_context(|| "Failed to parse embedded JSON")?;

    Ok(result)
}

/// Read message JSON from specified file
fn get_cookies_from_file(file_path: &str) -> anyhow::Result<Vec<ByteCookie>> {
    let mut file = std::fs::File::open(file_path)
        .with_context(|| format!("Failed to open file '{}'", file_path))?;

    let mut json = String::new();
    file.read_to_string(&mut json)
        .with_context(|| format!("Failed to read file '{}'", file_path))?;

    serde_json::from_str(&json).with_context(|| format!("JSON parsing failed for '{}'", file_path))
}

/// Decide random index for message
fn decide_random_index(count: usize) -> usize {
    let mut rng = rand::rng();
    rand::Rng::random_range(&mut rng, 0..count)
}

/// Decide a message index from today's date and username
fn decide_todays_index(count: usize, today: &str, user: &str) -> anyhow::Result<usize> {
    assert!(count > 0);
    let count =
        u32::try_from(count).with_context(|| format!("Failed to convert '{}' to u32", count))?;

    let pseudorandom = {
        let mut context = md5::Context::new();
        context.consume(user.as_bytes());
        context.consume(b":");
        context.consume(today.as_bytes());
        let digest = context.finalize();
        u32::from_be_bytes(digest[0..4].try_into()?)
    };

    let index = (pseudorandom % count) as usize;

    Ok(index)
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
