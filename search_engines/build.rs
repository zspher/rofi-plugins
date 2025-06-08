use serde::Deserialize;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Deserialize)]
struct BuildSearchEntry {
    s: Option<String>,
    t: Option<String>,
    u: Option<String>,
}
static JSON: &str = include_str!("bangs.json");

fn format_option(opt_str: &Option<String>) -> String {
    match opt_str {
        Some(s) => format!("\"{}\"", s.escape_default()),
        None => "None".to_string(),
    }
}

fn main() {
    let arr: Vec<BuildSearchEntry> =
        serde_json::from_str(JSON).expect("The content of bangs.json is not a valid array.");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_items.rs");
    let mut file = BufWriter::new(File::create(&dest_path).unwrap());

    let mut map = phf_codegen::Map::new();

    for item in &arr {
        if let Some(key) = &item.t {
            let entry_code = format!(
                "SearchEntry {{ title: {s}, url: {u} }}",
                s = format_option(&item.s),
                u = format_option(&item.u),
            );
            map.entry(key, &entry_code);
        }
    }

    writeln!(
        file,
        "pub static SEARCH_ENTRIES: phf::Map<&'static str, SearchEntry<'static>> = {};",
        map.build()
    )
    .unwrap();

    println!("cargo:rerun-if-changed=bangs.json");
}
