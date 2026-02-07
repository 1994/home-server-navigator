use std::{env, fs, io::Write, path::PathBuf};
use walkdir::WalkDir;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let root_dir = manifest_dir
        .parent()
        .expect("backend should be in repo root")
        .to_path_buf();
    let frontend_dist = root_dir.join("frontend").join("dist");

    println!("cargo:rerun-if-changed={}", frontend_dist.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("out dir"));
    let output_file = out_dir.join("embedded_assets.rs");
    let mut output = fs::File::create(&output_file).expect("create embedded assets file");

    if !frontend_dist.exists() {
        writeln!(
            output,
            "pub static EMBEDDED_ASSETS: &[(&str, &[u8], &str)] = &[];"
        )
        .expect("write fallback assets module");
        return;
    }

    let mut entries = Vec::new();
    for entry in WalkDir::new(&frontend_dist)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path().to_path_buf();
        let relative = path
            .strip_prefix(&frontend_dist)
            .expect("strip prefix")
            .to_string_lossy()
            .replace('\\', "/");
        entries.push((relative, path));
    }

    entries.sort_by(|left, right| left.0.cmp(&right.0));

    writeln!(
        output,
        "pub static EMBEDDED_ASSETS: &[(&str, &[u8], &str)] = &["
    )
    .expect("write assets open");

    for (relative, absolute) in &entries {
        let include_path = absolute.to_string_lossy().replace('\\', "\\\\");
        let mime = mime_for(relative);
        writeln!(
            output,
            "(\"/{relative}\", include_bytes!(\"{include_path}\"), \"{mime}\"),"
        )
        .expect("write asset entry");
    }

    writeln!(output, "];").expect("write assets close");
}

fn mime_for(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".js") {
        "application/javascript; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if path.ends_with(".json") {
        "application/json; charset=utf-8"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else if path.ends_with(".txt") {
        "text/plain; charset=utf-8"
    } else if path.ends_with(".map") {
        "application/json; charset=utf-8"
    } else {
        "application/octet-stream"
    }
}
