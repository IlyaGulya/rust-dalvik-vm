use std::{env, fs};
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    // let bytecode = File::open("data/bytecode.txt").expect("Unable to open data/bytecode.txt");
    download_jre();
    extract_stdlib();
    strip_stdlib();
    dex_stdlib();
}

fn strip_stdlib() {
    let mut command = std::process::Command::new("proguard");
    let _ = &command.arg("@proguard-runtime.pro");
    let output = command.output().expect("Unable to execute d8");
    assert!(output.status.success());
}

fn download_jre() {
    fs::create_dir_all("toolkit/jre").expect("Unable to create jre directory");
    let mut downloader = downloader::downloader::Builder::default()
        .download_folder("toolkit/".as_ref())
        .build()
        .expect("Unable to create downloader");

    let download = downloader::download::Download::new("https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u392-b08/OpenJDK8U-jre_x86-32_windows_hotspot_8u392b08.zip");

    downloader
        .download(&[download])
        .expect("Unable to download jre");
}

fn extract_stdlib() {
    if File::open("toolkit/jre/lib/rt.jar").is_ok() {
        return;
    }
    let path = "toolkit/OpenJDK8U-jre_x86-32_windows_hotspot_8u392b08.zip";
    let file =
        File::open(path)
            .expect(format!("Unable to open {}", path).as_str());
    zip_extract::extract(file, "toolkit/jre".as_ref(), true)
        .expect("Unable to extract jre");
}

fn dex_stdlib() {
    if File::open("toolkit/runtime.dex").is_ok() {
        return;
    }
    let android_home = std::env::var("ANDROID_HOME").expect("ANDROID_HOME not set");
    let build_tools_version = find_latest_build_tools().expect("Unable to find build tools");
    let build_tools = format!("{}/build-tools/{}", android_home, build_tools_version);
    let d8 = format!("{}/d8", build_tools);

    let dex_zip_path = "toolkit/runtime.dex.zip";

    let mut command = Command::new(d8);
    let _ = &command.arg("toolkit/runtime.jar");
    let _ = &command.arg("--output");
    let _ = &command.arg(dex_zip_path);

    let _ = command.output().expect("Unable to execute d8");

    let dex = File::open(dex_zip_path).expect(format!("Unable to open {}", dex_zip_path).as_str());
    zip_extract::extract(dex, "toolkit".as_ref(), true)
        .expect("Unable to extract dex");

    fs::rename("toolkit/classes.dex", "toolkit/runtime.dex").expect("Unable to rename dex");
}

fn try_execute_d8(d8_path: &str) -> bool {
    if !Path::new(d8_path).exists() {
        return false;
    }
    let mut command = Command::new(d8_path);
    let _ = &command.arg("--help");
    let output = command.output().expect("Unable to execute d8");
    output.status.success()
}

fn find_latest_build_tools() -> Option<String> {
    let sdk_path = env::var("ANDROID_SDK_ROOT").or_else(|_| env::var("ANDROID_HOME")).ok()?;
    let build_tools_path = Path::new(&sdk_path).join("build-tools");
    let mut latest_version = None;

    if let Ok(entries) = fs::read_dir(build_tools_path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let version = path.file_name()?.to_str()?;
                let is_newer = latest_version.as_ref().map_or(true, |v: &String| v.as_str() < version);
                let d8_path = format!("{}/d8", path.to_str()?);
                if is_newer && try_execute_d8(&d8_path) {
                    latest_version = Some(version.to_string());
                }
            }
        }
    }

    latest_version
}
