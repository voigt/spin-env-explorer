use anyhow::Result;
use spin_sdk::{
    config,
    http::{Request, Response},
    http_component,
};
use serde::{Serialize};
use serde_json;
use std::env;
use std::fs::read_dir;
use std::path::{PathBuf, Path};

#[derive(Serialize)]
struct KeyValue {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct Config {
    envvars: Vec::<KeyValue>,
    config: Vec::<KeyValue>,
    files: Vec::<String>,
    secret: String,
    version: String,
    age: u32,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_envvars(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    
    let mut config = Config {
        envvars: vec![],
        config: vec![],
        files: vec![],
        secret: "as".to_string(),
        version: "1.0.0".to_string(),
        age: 28,
    };

    // Retrieve environment variables
    env::vars()
        .for_each(|(k, v)| {
            // println!("{}: {}", &k, &v);
            config.envvars.push(KeyValue { key: k, value: v });
        });

    // Retrieve component.config values
    config.config.push(KeyValue{
        key: "config_one".to_string(),
        value: config::get("config_one").expect("Failed to acquire config_one"),
    });

    config.config.push(KeyValue{
        key: "config_two".to_string(),
        value: config::get("config_two").expect("Failed to acquire config_two"),
    });

    config.config.push(KeyValue{
        key: "ext_env_one".to_string(),
        value: config::get("ext_env_one").expect("Failed to acquire ext_env_one"),
    });

    config.config.push(KeyValue{
        key: "ext_env_two".to_string(),
        value: config::get("ext_env_two").expect("Failed to acquire ext_env_two"),
    });

    config.config.push(KeyValue{
        key: "environment".to_string(),
        value: config::get("environment").expect("Failed to acquire environment"),
    });
    
    let f = recurse_files("/");
    match f {
        Ok(list) => {
            list.into_iter().for_each(|file| {
                config.files.push(file.to_str().unwrap().to_string());
                // println!("{:?}", file)
            });

        },
        Err(e) => {}
    }

    let json = serde_json::to_string(&config).expect("Serialization failed");

    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some(json.into()))?)
}

fn recurse_files(path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            let mut subdir = recurse_files(entry.path())?;
            buf.append(&mut subdir);
        }

        if meta.is_file() {
            buf.push(entry.path());
        }
    }

    Ok(buf)
}
