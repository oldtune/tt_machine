use std::path::PathBuf;

pub fn read_config_from_config_file() -> ServiceConfiguration {}

pub struct ServiceConfiguration {
    db_connection: String,
    file_save_path: String,
    tiktok_url: String,
    file_tags_to_downoad: Vec<String>,
    quantity_for_each_tag: u8,
    tt_configuration: TTRequestConfiguration,
}

pub struct TTRequestConfiguration {
    scheme: String,
    host: String,
    file_name: String,
    query: TTRequestQuery,
}

pub struct TTRequestQuery {
    aid: u8,
    app_language: String,
    app_name: String,
    browser_language: String,
    browser_name: String,
    browser_online: bool,
    browser_platform: String,
    browser_version: String,
    channel: String,
    cookie_enabled: bool,
    device_id: String,
    device_platform: String,
    focus_state: bool,
    from_page: String,
    history_len: u8,
    is_fullscreen: bool,
    is_page_visible: bool,
    keyword: String,
    offset: u8,
    os: String,
    priority_region: String,
    referer: String,
    region: String,
    screen_height: u8,
    screen_width: u8,
    tz_name: String,
    web_search_code: String,
    webcast_language: String,
    msToken: String,
    x_bogus: String,
    signature: String,
}

impl ServiceConfiguraiton {
    pub fn parse_from_file() -> Self {
        let path = construct_configuraiton_path();

        Self {}
    }
}

pub fn construct_configuraiton_path() -> PathBuf {
    let path: PathBuf = PathBuf::new();
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let current_dir = current_dir.to_str().unwrap();
    path.push(current_dir);
    let config_file = "config";
    path.push(config_file);
    path.set_extension("toml");

    path
}
