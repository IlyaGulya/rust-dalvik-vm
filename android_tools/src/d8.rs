use std::{env, fs};
use std::path::Path;
use std::process::Command;

pub fn get_d8_path() -> String {
    let android_home = std::env::var("ANDROID_HOME").expect("ANDROID_HOME not set");
    let build_tools_version = find_latest_build_tools().expect("Unable to find build tools");
    let build_tools = format!("{}/build-tools/{}", android_home, build_tools_version);
    let d8 = format!("{}/d8", build_tools);
    d8
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
