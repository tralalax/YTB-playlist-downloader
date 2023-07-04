use youtube_dl::{YoutubeDl, YoutubeDlOutput};
use std::process::Command;

#[derive(Clone)]
#[allow(dead_code)]
pub enum VideoFormat {
    Audio,
    Video,
}

// VideoToDl is a single video to download
// url -> valid youtube url
// format -> Audio or Video
// dl_path -> exact path to download
pub struct VideoToDl {
    url: String,
    format: VideoFormat,
    dl_path: String,
    ffmpeg_path: String,
    youtube_dl_path: String,
}
#[allow(dead_code)]
impl VideoToDl {
    // REF DE PÄRTOUT FILS DE PUT NIQUE TA MERE
    pub fn new(url: String, format: VideoFormat, dl_path: String, ffmpeg_path: String, youtube_dl_path: String) -> Self {
        VideoToDl { url, format,  dl_path, ffmpeg_path, youtube_dl_path }
    }
}

#[derive(Clone)]
pub struct ConfigParams {
    pub ffmpeg_path: String,
    pub parent_dl_path: String,
    pub youtube_dl_path: String,
    pub prefered_vid_type: VideoFormat,
}
#[allow(dead_code)]
impl ConfigParams {
    pub fn new(ffmpeg_path: String, parent_dl_path: String, youtube_dl_path: String, prefered_vid_type: VideoFormat) -> Self {
        ConfigParams { ffmpeg_path, parent_dl_path, youtube_dl_path, prefered_vid_type }
    }
}
// impl Copy for ConfigParams {}
    



// path to yt-dlp.exe, required to download video
//const youtube_dl_path: &str = "D:\\yt-dlp.exe";
// bin/
//const ffmpeg_path: &str = "D:\\ffmpeg.exe";

#[allow(dead_code)]
pub fn download(video: VideoToDl) -> Result<youtube_dl::YoutubeDlOutput, youtube_dl::Error> {

    // let url: String = video.url;
    // let dl_path: String = video.dl_path;

    match video.format {

        VideoFormat::Audio => {
            match download_audio(video) {
                Ok(res) => return Ok(res),
                Err(e) => return Err(e),
            }
        },

        VideoFormat::Video => {
            match download_video(video) {
                Ok(res) => return Ok(res),
                Err(e) => return Err(e),
            }
        },
    };
}


// download video to mp3 format
fn download_audio(video: VideoToDl) -> Result<youtube_dl::YoutubeDlOutput, youtube_dl::Error> {

    let _ydl: YoutubeDlOutput = match YoutubeDl::new(video.url)
        .youtube_dl_path(video.youtube_dl_path)
        .extra_arg("-q")
        .extra_arg("-i")
        .extra_arg("--no-abort-on-error")
        //.extra_arg("--abort-on-unavailable-fragments")
        .extra_arg("--no-abort-on-unavailable-fragments")
        .extra_arg("--ignore-errors")
        // DOWNLOAD AUDIO FILE (require ffmpeg)
        .extract_audio(true) 
        .extra_arg("--audio-format")
        .extra_arg("mp3")
        .extra_arg("--ffmpeg-location")
        .extra_arg(video.ffmpeg_path)
        .extra_arg("--paths")
        .extra_arg(video.dl_path)
        .download(true)
        .run()
    {
        Ok(res) => return Ok(res),
        Err(e) => return Err(e),
    };
}

// download video to mp4 format
fn download_video(video: VideoToDl) -> Result<youtube_dl::YoutubeDlOutput, youtube_dl::Error> {

    let _ydl: YoutubeDlOutput = match YoutubeDl::new(video.url)
        .youtube_dl_path(video.youtube_dl_path)
        .extra_arg("-q")
        .extra_arg("-i")
        .extra_arg("--no-abort-on-error")
        //.extra_arg("--abort-on-unavailable-fragments")
        .extra_arg("--no-abort-on-unavailable-fragments")
        .extra_arg("--ignore-errors")

        .extra_arg("--paths")
        .extra_arg(video.dl_path)
        .download(true)
        .run()
    {
        Ok(res) => return Ok(res),
        Err(e) => return Err(e),
    };
}

/// get all video ID from a playlist
// TODO -> yt-dlp --flat-playlist --print id
// https://github.com/yt-dlp/yt-dlp/issues/2117
#[allow(dead_code)]
pub fn get_video_from_pl(pl_id: &str, youtube_dl_path: &String) -> Result<Vec<String>, String> {

    let pl_url: String = format!("https://www.youtube.com/playlist?list={}", pl_id);

    match Command::new(youtube_dl_path)
        .arg("-q")
        .arg("-i")
        .arg("--flat-playlist")
        .arg("--get-id")
        .arg(pl_url)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8(output.stdout).unwrap();
            let vec_ids: Vec<String> = stdout.trim_end().split('\n').map(|s| s.to_string()).collect();

            return Ok(vec_ids);
        },
        Err(err) => {return Err("bad handled error ssry ".to_string())},
    };

}

/// get playlist title
#[allow(dead_code)]
pub fn get_playlist_title(pl_id: &str, youtube_dl_path: &String) -> Result<String, String> {

    let pl_url: String = format!("https://www.youtube.com/playlist?list={}", pl_id);

    match Command::new(youtube_dl_path)
        .arg("-q")
        .arg("-i")
        .arg("--flat-playlist")
        .arg("--print")
        .arg("playlist_title")
        .arg(pl_url)
        .output()
    {
        Ok(output) => {

            let stdout = String::from_utf8(output.stdout).unwrap();

            let pl_name: String = stdout.split_once("\n").unwrap().0.trim().to_string();

            return Ok(pl_name);
        },
        Err(err) => {return Err("bad handled error ssry ".to_string())},
    };

}