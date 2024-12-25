# YTB-Playlist-Downloader

YTB-Playlist-Downloader is a program written in rust to automatically download videos you add to a YouTube playlist.
It automatically checks whether any new videos have been added to a YouTube playlist since the program was last run, and if so, downloads them.

This tool can be useful if you want to have an up-to-date backup of your YouTube playlist on your machine.

If you use/like it, I'd really appreciate it if you add a star to this repo! :D

It relies on [YT-DLP](https://github.com/yt-dlp/yt-dlp) and [ffmpeg](https://github.com/yt-dlp/FFmpeg-Builds) to download and convert files. The video IDs of a playlist are stored in a SQLite database.

### Current feature:
- Support multiple playlists
- Choice between audio and video
- Sub-folder for each playlist
- log file

__Feel free to contribute or suggest features !__

# Installation :
1. Install the [latest release](github.com/tralalax/YTB-playlist-downloader/releases/latest) or build from source : ```cargo build --release```
2. Install the [latest release of yt-dlp](https://github.com/yt-dlp/yt-dlp/releases/latest) and the [latest release of ffmpeg](https://github.com/yt-dlp/FFmpeg-Builds/releases/tag/latest)
3. Run the executable, it will create a config.toml file, a playlist.toml file, a database folder and a log file.
When all files have been created, the application closes. You can then configure the playlist.toml and config.toml files, see Configuration below.

## Run at startup (windows) :
You can create a shortcut to the application and place it in your `%appdata%\Microsoft\Windows\Startup Menu\Programs\Startup` directory. By doing so, the application will start when your computer starts up and automatically check your playlist for new videos to download.

## Run with Cron jobs (linux) :
You might want to install this program on a Linux server to back up your YouTube playlist. A good idea would be to periodically update those backups by running this program with a Cron job.
The following example runs the program every week :
```sh
0 0 * * Sun cd /path/to/ytb_pl_dl-1-2_linux_x86_64_gnu && ./ytb_pl_dl-1-2_linux_x86_64_gnu
```

# Configuration
After launching the application for the first time, you need to edit the config.toml file. You need to specify the path to yt-dlp and ffmpeg, which you can download here : 
[YT-DLP](https://github.com/yt-dlp/yt-dlp) [ffmpeg](https://github.com/yt-dlp/FFmpeg-Builds). You can also provide a download path, where the downloaded file will be stored. If no path is specified, the download will default to the current directory.

After configuring the `config.toml` file, you can add the playlist URL to the playlist.toml file. Here's an example of how it should be written:
```toml
"https://www.youtube.com/playlist?list=PLHtyfDv32xnEVdkhAVsA5sFyjcOv" = "audio"
"https://www.youtube.com/playlist?list=PLHtyfDv32xnG0E3_NzqO5w7JcA_h" = "video"
"https://www.youtube.com/playlist?list=PL0usNGW1865yE7D83hLohy0gakwx" = "audio"
```
the name of the key must be a youtube playlist URL.


the value after the `=` sign must be `audio` or `video` to specify whether you want to download the video from the associated playlist as an audio or video file.

(if you specify anything other than `audio` or `video`, the `audio` type will be chosen by default)

Make sure you insert the URL of the playlist and the download type between `"` and `"`.

# Additional notes
- if you ever need to clarify which videos have been downloaded or not, you can delete or edit the database in `database/playlist.db`
- on Windows, logs are stored in a file named `output.log` ; on Linux, everything is printed to stdout
