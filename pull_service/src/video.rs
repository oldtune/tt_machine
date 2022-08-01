pub struct File {
    desc: &str,
    file_path: PathBuf,
    play_address: String,
    duration: u8,
    tt_id: String,
    origin_cover: String,
    format: String,
    stat: VideoStat,
}

pub struct FileStat {
    digg_count: u32,
    comment_count: u32,
    play_count: u32,
}
