use std::process::{Command,exit};
use std::os::unix::process::CommandExt;
use std::fs;
use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct OciImageConfig {
    config: OciConfig,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct OciConfig {
    cmd: Option<Vec<String>>,
    entrypoint: Option<Vec<String>>,
    env: Option<Vec<String>>,
    //exposed_ports: Option<Vec<String>>,
    //volumes: Option<Vec<OciVolume>>,
}

#[derive(Deserialize, Debug)]
struct OciVolume {}

fn main() {

    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        eprintln!("Usage: {} CONFIG_PATH", argv[0]);
        exit(1);
    }

    let config_path = &argv[1];

    let config_str = fs::read_to_string(config_path)
        .expect("Failed reading config");

    let image_config: OciImageConfig = serde_json::from_str(&config_str)
        .expect("Failed to parse JSON");

    let mut args: Vec<String> = Vec::new();

    if let Some(entrypoint) = image_config.config.entrypoint {
        args.extend(entrypoint);
    }

    if let Some(cmd) = image_config.config.cmd {
        args.extend(cmd);
    }

    let mut envs: Vec<(String, String)> = Vec::new();

    if let Some(e) = image_config.config.env {
        envs = e.iter().filter_map(|v| {
            let parts: Vec<&str> = v.split("=").collect();

            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        }).collect();
    }

    println!("Args: {:?}", args);
    println!("Env: {:?}", envs);

    let err = Command::new("/sbin/switch_root")
        .arg("/mnt/run")
        .args(args)
        .env_clear()
        .envs(envs)
        .exec();
    println!("Error: {}", err);
}
