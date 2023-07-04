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
2. Install [latest release of yt-dlp](https://github.com/yt-dlp/yt-dlp/releases/latest) and the [latest release of ffmpeg](https://github.com/yt-dlp/FFmpeg-Builds/releases/tag/latest)

3. Run the executable, it will create a config.toml file, a playlist.toml file, a database folder and a log file.
When all files have been created, the application closes. You can then configure the playlist.toml and config.toml files, see Configuration below.

## Run at startup :
You can create a shortcut to the application and place it in your `%appdata%\Microsoft\Windows\Startup Menu\Programs\Startup` directory. By doing so, the application will start when your computer starts up and automatically check your playlist for new videos to download.

# Configuration
After launching the application for the first time, you need to edit the config.toml file. You need to specify the path to yt-dlp and ffmpeg, which you can download here : 
[YT-DLP](https://github.com/yt-dlp/yt-dlp) [ffmpeg](https://github.com/yt-dlp/FFmpeg-Builds). You can also provide a download path, where the downloaded file will be stored. If no path is specified, the download will default to the current directory.
And you can specify a preferred file type: `audio` is for audio files in .mp3 format ; `video` is for video files (image+sound).

**Note :** ffmpeg is only required for the `audio` format

After configuring the config.toml file, you can add the playlist URL to the playlist.toml file. Here's an example of how it should be written:
```toml
playlist1 = "https://www.youtube.com/playlist?list=PLHtyfDv32xnEBJiyxKaiDXGCaw974vJbu"
example2 = "https://www.youtube.com/watch?v=5W8vqbZhxSo&list=PLHtyfD554dzeSDnEP5uM4N6Jy9sBtAKyPpUp7&index=3"
blablablabla = "https://www.youtube.com/watch?v=5W8vqbZhxSo&list=PsdGFEgsfYHGdp45xnEP5uM4N6Jy9setAKyP58p7"
```
the name of the key doesn't matter, put whatever you want, the only important thing is the URL after the `=` sign.
Make sure you insert the URL of the playlist between `"` and `"`.

