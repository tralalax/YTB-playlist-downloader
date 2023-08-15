// uncomment to disable console
// #![windows_subsystem = "windows"]

mod downloader;
mod database;
mod config_manager;

// config manager
use crate::config_manager::{read_config_file, read_playlist_config};

// ytbdl
use crate::downloader::{VideoToDl, VideoFormat, ConfigParams ,download, get_video_from_pl, get_playlist_title, update_ytdlp};

// db
use database::{create_table, insert_new_video, VideoDB, get_video_from_db, table_exists, connect};

// logging
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

// other
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    // setup logging
    setup_logging();

    println!(r#"
    __     _________ ____       _____  _           __     ___      _____  _____ _______
    \ \   / /__   __|  _ \     |  __ \| |        /\\ \   / / |    |_   _|/ ____|__   __|
     \ \_/ /   | |  | |_) |    | |__) | |       /  \\ \_/ /| |      | | | (___    | |  
      \   /    | |  |  _ <     |  ___/| |      / /\ \\   / | |      | |  \___ \   | |  
       | |     | |  | |_) |    | |    | |____ / ____ \| |  | |____ _| |_ ____) |  | |  
       |_|     |_|  |____/     |_|    |______/_/    \_\_|  |______|_____|_____/   |_|  
"#
    );
    println!(r#"
         _____   ______          ___   _ _      ____          _____  ______ _____  
        |  __ \ / __ \ \        / / \ | | |    / __ \   /\   |  __ \|  ____|  __ \ 
        | |  | | |  | \ \  /\  / /|  \| | |   | |  | | /  \  | |  | | |__  | |__) |
        | |  | | |  | |\ \/  \/ / | . ` | |   | |  | |/ /\ \ | |  | |  __| |  _  / 
        | |__| | |__| | \  /\  /  | |\  | |___| |__| / ____ \| |__| | |____| | \ \ 
        |_____/ \____/   \/  \/   |_| \_|______\____/_/    \_\_____/|______|_|  \_\
"#
    );

    log::info!("Starting the Playlist Downloader...");

    // check environement before running
    start_env_check();

    // etablish connection with DB
    let db_con = connect();
    
    // read config file
    let config_stuff: HashMap<String, String> = read_config_file();
    
    let config_params: ConfigParams = ConfigParams::new(
        config_stuff.get("ffmpeg_path").unwrap().to_string(),
        config_stuff.get("download_path").unwrap().to_string(),
        config_stuff.get("youtube_dl_path").unwrap().to_string(),
        //if config_stuff.get("download_type").unwrap().to_string() == "video" {VideoFormat::Video} else {VideoFormat::Audio}
    );
    
    // update yt-dlp
    update_ytdlp(&config_stuff.get("youtube_dl_path").unwrap().to_string());
    

    // read playlist file
    //                      <playlist ID , download type>
    let playlist_id: HashMap<String, VideoFormat> = read_playlist_config();

    for (playlist, download_type) in playlist_id {
        log::info!("checking for new videos in playlist ID : {}", playlist);

        // check if the playlist is already in the DB
        let table_exist: bool = table_exists(&db_con, &playlist);


        if table_exist {
            // playlist is already in DB
            log::info!("This playlist is already in the database, checking for new video to download...");
            check_for_new_video(&db_con, &playlist, &config_params, download_type);
        }
        else {
            // playlist is not in DB
            log::info!("This playlist is not in the database, downloading the whole playlist...");
            download_whole_playlist(&db_con, playlist, &config_params, download_type);
        }
    }

}

/// setup loggerr
fn setup_logging() {

    // log file template
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} - {l} : {m}\n")))
        .build("output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info))
                   .unwrap();

    // init logger and handle errors
    match log4rs::init_config(config) {
        Ok(_) => log::info!("Logger initialized"),
        Err(err) => log::error!("Logger not initialized : {}", err)
    }

}

/// check if required file/folder is present for the app to start
fn start_env_check() {

    // check database folder
    if !Path::new("database").exists() {
        
        log::warn!("database folder not found, creating a new one...");

        // create database folder
        match fs::create_dir("database") {
            Ok(_) => {
                log::info!("database folder created !");
            },
            Err(err) => {
                log::error!("database folder not created, try creating it manualy : {}", err);
                std::process::exit(1);
            }
        }
    }

    // check playlist file
    if !Path::new("playlist.toml").exists() {

        log::warn!("playlist.toml not found, creating a new one...");

        // create the playlist.toml file
        match File::create("playlist.toml") {
            Ok(_) => log::info!("playlist.toml created"),
            Err(err) => {
                log::error!("playlist.toml file not created, try creating it manualy : {}", err);
                std::process::exit(1);
            }
        };
    }

    // check config file
    let config_content = format!(
        r#"
ffmpeg_path = ""
youtube_dl_path = ""
download_path = ""
download_type = "" # 'audio' or 'video'
"#
    );

    // check if config file exist
    if !Path::new("config.toml").exists() {

        log::warn!("config.toml not found, creating a new one...");

        // create the config.toml file
        let mut config_file = match File::create("config.toml") {
            Ok(file) => file,
            Err(err) => {
                log::error!("config.toml not created, try creating it manualy : {}", err);
                std::process::exit(1);
            }
        };

        // write file content
        match config_file.write_all(config_content.as_bytes()) {
            Ok(_) => {
                log::warn!("Config file writed with empty parameter, check config.toml and restart the app");
                std::process::exit(0)
            },
            Err(err) => log::error!("error writing config.toml file : {}", err)
        }
    }

}


/// get download path for the given playlist
fn get_download_path(pl_id: &String, config_dl_path: &String, ytbdl_path: &String) -> String {

    // get name of the playlist from the ID
    let pl_name: String = match get_playlist_title(pl_id, &ytbdl_path) {
        Ok(playlist_title) => playlist_title,
        Err(err) => {
            log::warn!("cannot get playlist name from youtube : {}", err);
            std::process::exit(1);
        }
    };

    // remove invalid caracter in the name
    let sanitized_pl_name: String = sanitize_name(&pl_name);

    // create the final download path
    let final_dl_path: String = format!("{config_dl_path}\\{}", sanitized_pl_name);

    // check if a folder with playlist name exist in config_dl_path
    if Path::new(&final_dl_path).exists() {
        // there is a folder named by the playlist name in the download path
        return final_dl_path;
    }
    else {
        // create a new folder for this playlist
        match fs::create_dir_all(&final_dl_path) {

            Ok(_) => return final_dl_path,
            Err(err) => {
                log::error!("folder for {pl_name} not created : {err}");
                return config_dl_path.to_string();
            }
        }
    }
}

/// sanitize the folder name to remove special char before creating the drectory
fn sanitize_name(folder_name: &String) -> String {
    // List of invalid characters in Windows folder names
    let invalid_chars: Vec<char> = vec!['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

    // Create a new String to hold the sanitized folder name
    let mut sanitized_name = String::new();

    // Iterate through each character in the folder name
    for c in folder_name.chars() {
        // Check if the character is not in the list of invalid characters
        if !invalid_chars.contains(&c) {
            sanitized_name.push(c);
        }
    }

    return sanitized_name;
}

// if the playlist is in DB, check for new video to download
fn check_for_new_video(db_con: &rusqlite::Connection, playlist_id: &String, config_params: &ConfigParams, download_type: VideoFormat) {

    // extract all video ID from the playlist
    let id_from_ytb: Vec<String> = match get_video_from_pl(&playlist_id, &config_params.youtube_dl_path) {
        Ok(result) => result,
        Err(err) => handle_error(format!("error while fetching video ID from the playlist on youtube : {}", err)),
    };

    // get all video ID from DB
    let id_from_db: Vec<String> = match get_video_from_db(db_con, playlist_id) {
        Ok(result) => result,
        Err(err) => handle_error(format!("error while fetching video ID from database : {}", err)),
    };

    // check for non-match betwen DB and playlist
    let video_to_dl: Vec<String> = compare_video_from_db_and_pl(id_from_db, id_from_ytb);

    // if there is no video to DL, return
    if video_to_dl.is_empty() {
        return;
    }

    // get download path
    let download_path: String = get_download_path(playlist_id, &config_params.parent_dl_path, &config_params.youtube_dl_path);

    // download each new video and save it in DB
    for video in video_to_dl {
        let vid_url: String = format!("https://www.youtube.com/watch?v={}", video);

        let video_dl: VideoToDl = VideoToDl::new(
            vid_url,
            download_type.clone(),
            download_path.clone(),
            config_params.ffmpeg_path.clone(),
            config_params.youtube_dl_path.clone());

        let video_db: VideoDB = VideoDB::new(String::from(video), playlist_id.clone());
        
        match download(video_dl)
        {
            Ok(_) => {
                // add video ID in DB
                insert_new_video(db_con, video_db);
            },
            Err(err) => log::error!("downloading a video : {}", err),
        }
    }
}


fn compare_video_from_db_and_pl(id_from_db: Vec<String>, id_from_ytb: Vec<String>) -> Vec<String> {

    // compare the 2 vectors to find non-match -> new video to download
    // else return an empty vector

    // result returned vector of ID to download
    let mut id_to_dl: Vec<String> = Vec::new();

    // loop over the vector of ID from the DB
    // TODO loop limitter
    for i in id_from_ytb {

        // check if the
        if id_from_db.contains(&i) {
        }
        else {
            log::info!("new video to download detected : {i}");
            id_to_dl.push(i)
        }
    }

    return id_to_dl;
}


fn handle_error(msg: String) -> Vec<String> {
    log::error!("{msg}");
    Vec::new()
}


// if the playlist is not in DB, download the whole playlist and save it in DB
fn download_whole_playlist(db_con: &rusqlite::Connection, playlist_id: String, config_params: &ConfigParams, download_type: VideoFormat) {
    // create a new table for this playlist
    create_table(db_con, &playlist_id);

    // get download path
    let download_path: String = get_download_path(&playlist_id, &config_params.parent_dl_path, &config_params.youtube_dl_path);

    // extract all video ID from the playlist
    let result = get_video_from_pl(&playlist_id, &config_params.youtube_dl_path);
    // download all videos
    match result {
        Ok(result) => {

            // loop over the vector of video ID and download them
            for i in result {

                let vid_url: String = format!("https://www.youtube.com/watch?v={}", i);

                let video_dl: VideoToDl = VideoToDl::new(
                    vid_url,
                    download_type.clone(),
                    download_path.clone(),
                    config_params.ffmpeg_path.clone(),
                    config_params.youtube_dl_path.clone());
                
                let video_db: VideoDB = VideoDB::new(String::from(i), playlist_id.clone());

                match download(video_dl)
                {
                    Ok(_) => {
                        // add video ID in DB
                        insert_new_video(db_con, video_db);
                    },
                    Err(err) => log::error!("downloading a video : {}", err),
                };
            }
        },
        Err(err) => log::error!("while fetching video ID from the playlist on youtube : {}", err),
    }
}


// APP SCHEM

// take 1st playlist in config
// extract every video ID in it
// check if playlist is in DB
//      yes -> check for video ID that is not in DB
//          yes(there is) -> download them ; check if succes
//                yes(succes) -> add ID in db
//                no(fail) -> show error, don't save them in DB, log them
//          no(there isn't) -> return
//      no -> download all video, add video ID in a new DB table
// goto start
