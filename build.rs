use std::io::Write;

/// embedded JSON file path
const JSON_PATH: &str = "assets/bytecookies.json";

fn main() {
    println!("cargo:rerun-if-changed={JSON_PATH}");

    // read the JSON file, and minify it.
    let json_raw = std::fs::read_to_string(JSON_PATH).expect("Failed to read JSON file");
    let json_value: serde_json::Value = serde_json::from_str(&json_raw).expect("Invalid JSON");
    let minified = serde_json::to_string(&json_value).expect("Failed to minify JSON");

    // write the minified JSON to a gzipped file in the OUT_DIR
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("bytecookies.json.gz");
    let file = std::fs::File::create(&dest_path).expect("Failed to create output file");
    let mut encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    encoder
        .write_all(minified.as_bytes())
        .expect("Failed to write gzip");
    encoder.finish().expect("Failed to finish gzip");
}
