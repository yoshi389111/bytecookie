use flate2::Compression;
use flate2::write::GzEncoder;
use std::{env, fs, io::Write, path::Path};

fn main() {
    // read the JSON file, and minify it.
    let json_raw = fs::read_to_string("assets/bytecookies.json").expect("Failed to read JSON file");
    let json_value: serde_json::Value = serde_json::from_str(&json_raw).expect("Invalid JSON");
    let minified = serde_json::to_string(&json_value).expect("Failed to minify JSON");

    // write the minified JSON to a gzipped file in the OUT_DIR
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("bytecookies.json.gz");
    let file = fs::File::create(&dest_path).expect("Failed to create output file");
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder
        .write_all(minified.as_bytes())
        .expect("Failed to write gzip");
    encoder.finish().expect("Failed to finish gzip");
}
