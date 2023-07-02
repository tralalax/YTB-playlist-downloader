use config::Config;
use std::collections::HashMap;
use std::fs;
use std::path::Path;


#[allow(dead_code)]
pub fn read_config_file() -> HashMap<String, String> {

    // create a dict to return all value from config file
    let mut output_dict = HashMap::<String, String>::new();

    // read config file
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let toml_content = config.try_deserialize::<HashMap<String, String>>().unwrap();

    // get ffmpeg path, if not found -> default to root directory
    let ffmpeg_path = toml_content.get("ffmpeg_path");
    match ffmpeg_path {
        Some(val) => {
            output_dict.insert("ffmpeg_path".to_string(), val.to_string());
        },
        None => {
            log::error!("ffmpeg path not specified or incorrect in config file");
            std::process::exit(1)
        },
    }

    // get youtube_dl_path path, if not found -> default to root directory
    let youtube_dl_path = toml_content.get("youtube_dl_path");
    match youtube_dl_path {
        Some(val) => {
            output_dict.insert("youtube_dl_path".to_string(), val.to_string());
        },
        None => {
            log::error!("YTB DL path not specified or incorrect in config file");
            std::process::exit(1);
        },
    }

    // get download_path path, if not found -> create a output folder in root directory
    if let Some(download_path) = toml_content.get("download_path") {
        output_dict.insert("download_path".to_string(), download_path.to_string());
    }
    else {
        // create a output folder in root directory
        log::warn!("download path not found in config file, creating a output folder...");
            
        if Path::new("output").exists() {
            log::info!("found an output folder in root directory");
            output_dict.insert("download_path".to_string(), fs::canonicalize("output").unwrap().into_os_string().into_string().unwrap());
        }
        else {
            match fs::create_dir("output") {
                Ok(_) => {
                    log::info!("output folder created !");
                    // get full path
                    output_dict.insert("download_path".to_string(), fs::canonicalize("output").unwrap().into_os_string().into_string().unwrap());
                },
                Err(err) => {
                    log::error!("output folder not created : {}", err);
                    std::process::exit(1);
                }
            }
        }
    }

    // get download type (audio or video)
    let dl_type = toml_content.get("download_type");
    match dl_type {
        Some(val) => {
            if val == "audio" {
                output_dict.insert("download_type".to_string(), val.to_string());
            }
            else if val == "video" {
                output_dict.insert("download_type".to_string(), val.to_string());
            }
            else {
                log::error!("download type (audio/video) not specified or incorrect in config file");
            }
        },
        None => log::error!("download type (audio/video) not specified or incorrect in config file"),
    }

    return output_dict;

}



// read playlist.ini file to get playlist URL
#[allow(dead_code)]
pub fn read_playlist_config() -> Vec<String> {
    // read playlist file
    let config = Config::builder()
        .add_source(config::File::with_name("playlist"))
        .build()
        .unwrap();

    // content in config file
    let config_content = config.try_deserialize::<HashMap<String, String>>().unwrap();


    // check if config is empty
    if config_content.is_empty() {
        log::warn!("playlist.toml is empty, there is nothing to do here, exiting");
        std::process::exit(0);
    }
    
    // return a vector containing all playlist ID
    let mut playlist_id: Vec<String> = Vec::new();

    for config_line in config_content {
        // get only the playlist ID
        let pl_id = config_line.1.split_once("list=");

        let final_id: String = String::from(pl_id.unwrap().1);        
        
        // add the ID to a vector
        playlist_id.push(final_id);
    }

    return playlist_id;

}
