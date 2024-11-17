use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
    let video_pattern = Regex::new(r"(?i)^(.*)S(\d+)E(\d+)(.*)\.(mkv|mp4)$").unwrap();
    let name_pattern = Regex::new(r"(?i)^(.*)\.(mkv|mp4)$").unwrap();
    let subtitle_pattern = Regex::new(r"(?i)^(.*)S(\d+)E(\d+)(.*)\.(srt|ass)$").unwrap();

    let video_files = fs::read_dir("./").unwrap();

    for video in video_files {
        let video_path = video.unwrap().path();
        let video_name = video_path.file_name().unwrap().to_str().unwrap();
        if let Some(caps) = video_pattern.captures(video_name) {
            let season = &caps[2];
            let episode = &caps[3];

            let subtitle_files = fs::read_dir("./").unwrap();
            for subtitle in subtitle_files {
                let subtitle_path = subtitle.unwrap().path();
                let subtitle_name = subtitle_path.file_name().unwrap().to_str().unwrap();

                if let Some(sub_caps) = subtitle_pattern.captures(subtitle_name) {
                    let sub_season = &sub_caps[2];
                    let sub_episode = &sub_caps[3];
                    let sub_hz = &sub_caps[5];

                    if season == sub_season && episode == sub_episode {
                        if let Some(name_caps) = name_pattern.captures(video_name) {
                            println!("{:?}", &name_caps[1]);
                            let new_subtitle_name = format!("{}{}.{}", &name_caps[1], ".zh-CN", sub_hz);
                            let new_subtitle_path = Path::new(&new_subtitle_name);

                            fs::rename(&subtitle_path, &new_subtitle_path).unwrap();
                            println!("Renamed {:?} to {:?}", subtitle_path, new_subtitle_path);
                        }
                    }
                }
            }
        }
    }
}