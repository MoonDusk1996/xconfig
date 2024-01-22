use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, Value};
use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::path::PathBuf;

//xconfig json structure
#[derive(Debug, Serialize, Deserialize)]
struct XConfig {
    xmrig_config_file_path: String,
}

//xmrig json structure
#[derive(Debug, Deserialize, Serialize)]
struct XmrigConfig {
    cpu: CpuConfig,
}
#[derive(Debug, Deserialize, Serialize)]
struct CpuConfig {
    rx: Vec<u32>,
    enabled: bool,
}

pub fn create_xconfig() {
    let home_dir = home_dir().expect("Failed to get the home directory.");
    let xconfig_path = home_dir.join(".config/xconfig/teste.json");

    //creates a directory if it doesn't exist
    if let Some(parent) = xconfig_path.parent() {
        if !parent.exists() {
            if let Err(err) = fs::create_dir_all(parent) {
                eprintln!("Failed to create directory: {}", err);
                std::process::exit(1);
            }
        } else {
            return;
        }
    }

    //creating the file
    let content = r#"{"xmrig_config_file_path":"path_to_xmrig_config.json"}"#;
    match File::create(&xconfig_path) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(content.as_bytes()) {
                eprintln!("Failed to write to file: {}", err);
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Failed to create xconfig file: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn update_xmrig_config(mut cpu_selection: u32) {
    match get_xconfig() {
        Ok(xconfig) => match cpu_selection {
            0 => {
                //format path
                let formated_xmrig_path = if let Some(home_dir) = dirs::home_dir() {
                    if xconfig.xmrig_config_file_path.starts_with("~/") {
                        let suffix = &xconfig.xmrig_config_file_path[2..];
                        home_dir.join(suffix)
                    } else {
                        PathBuf::from(xconfig.xmrig_config_file_path)
                    }
                } else {
                    PathBuf::from(xconfig.xmrig_config_file_path)
                };

                //read xmrig config
                let json_str = match fs::read_to_string(&formated_xmrig_path) {
                    Ok(content) => content,
                    Err(err) => {
                        eprintln!("Error finding xmrig configuration file: {}", err);
                        return;
                    }
                };
                //deserializing JSON
                let mut xmrig_config: XmrigConfig =
                    serde_json::from_str(&json_str).expect("Error deserializing JSON");

                xmrig_config.cpu.enabled = false;

                //insert changes into JSON
                let mut json_value: Value =
                    serde_json::from_str(&json_str).expect("Error deserializing JSON");
                json_value["cpu"]["enabled"] = json!(xmrig_config.cpu.enabled);

                //serialize JSON
                let final_json =
                    serde_json::to_string_pretty(&json_value).expect("Erro serialize JSON");

                //write xmrig json file
                match fs::write(&formated_xmrig_path, final_json) {
                    Ok(()) => return,
                    Err(err) => eprintln!("Erro ao escrever o arquivo JSON: {}", err),
                }
            }

            1.. => {
                //format cpu_selection in vector
                cpu_selection -= 1;
                let cpu_vector: Vec<u32> = (0..=cpu_selection).collect();

                //format path
                let formated_xmrig_path = if let Some(home_dir) = dirs::home_dir() {
                    if xconfig.xmrig_config_file_path.starts_with("~/") {
                        let suffix = &xconfig.xmrig_config_file_path[2..];
                        home_dir.join(suffix)
                    } else {
                        PathBuf::from(xconfig.xmrig_config_file_path)
                    }
                } else {
                    PathBuf::from(xconfig.xmrig_config_file_path)
                };

                //read xmrig config
                let json_str = match fs::read_to_string(&formated_xmrig_path) {
                    Ok(content) => content,
                    Err(err) => {
                        eprintln!("Error finding xmrig configuration file: {}", err);
                        return;
                    }
                };
                //deserializing JSON
                let mut xmrig_config: XmrigConfig =
                    serde_json::from_str(&json_str).expect("Error deserializing JSON");

                //modify rx values
                xmrig_config.cpu.rx = cpu_vector;
                xmrig_config.cpu.enabled = true;

                //insert changes into JSON
                let mut json_value: Value =
                    serde_json::from_str(&json_str).expect("Erro ao desserializar JSON");
                json_value["cpu"]["rx"] = json!(xmrig_config.cpu.rx);
                json_value["cpu"]["enabled"] = json!(xmrig_config.cpu.enabled);

                //serialize JSON
                let final_json =
                    serde_json::to_string_pretty(&json_value).expect("Erro ao serializar JSON");

                //write xmrig json file
                match fs::write(&formated_xmrig_path, final_json) {
                    Ok(()) => return,
                    Err(err) => eprintln!("Erro ao escrever o arquivo JSON: {}", err),
                }
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err)
        }
    }
}

fn get_xconfig() -> Result<XConfig, io::Error> {
    let home_dir = home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get the home directory."))?;
    let xconfig_json_path = home_dir.join(".config/xconfig/teste.json");

    let content = fs::read_to_string(&xconfig_json_path).map_err(|err| {
        eprintln!("Failed to read xconfig file: {}", err);
        err
    })?;

    let xconfig = from_str::<XConfig>(&content).map_err(|err| {
        eprintln!("Failed to deserialize JSON: {}", err);
        io::Error::new(io::ErrorKind::Other, "Failed to deserialize JSON")
    })?;

    Ok(xconfig)
}
