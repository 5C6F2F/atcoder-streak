use headless_chrome::Browser;
use line_notify::LineNotify;
use serde::Deserialize;
use std::{error::Error, fs};

#[tokio::main]
async fn main() {
    let mut current_exe = std::env::current_exe().expect("Failed to get current exe path");
    current_exe.pop();
    std::env::set_current_dir(current_exe).expect("Failed to set directory");

    let config = Config::new();
    let load_last_ac = config.load_last_ac;
    let compare_dates = config.compare_dates;
    let line_notifier = config.line_notifier;

    let last_ac = load_last_ac.load_last_ac().expect("Failed to load lastAC");

    if !compare_dates.is_streak_updated(last_ac) {
        line_notifier.send().await;
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    load_last_ac: LoadLastAC,
    compare_dates: CompareDates,
    line_notifier: LineNotifier,
}

impl Config {
    fn new() -> Self {
        let config_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");
        let config: Config = toml::from_str(&config_str).expect("Failed to parse config.toml");

        if config.load_last_ac.user_name.is_empty() {
            panic!("user name is empty");
        }
        if config.line_notifier.token.is_empty() {
            panic!("line notify token is empty");
        }

        config
    }
}

#[derive(Debug, Deserialize)]
struct LoadLastAC {
    user_name: String,
    address: String,
    user_id_selector: String,
    user_button_selector: String,
    last_ac_selector: String,
}

impl LoadLastAC {
    fn load_last_ac(&self) -> Result<String, Box<dyn Error>> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to(&self.address)?;

        tab.wait_for_element(&self.user_id_selector)?.click()?;

        tab.type_str(&self.user_name)?.press_key("Enter")?;

        tab.wait_for_element(&self.user_button_selector)?.click()?;

        let last_ac = tab
            .wait_for_element(&self.last_ac_selector)?
            .get_inner_text()?;

        Ok(last_ac)
    }
}

#[derive(Debug, Deserialize)]
struct CompareDates {
    extra_text: String,
    date_format: String,
}

impl CompareDates {
    fn is_streak_updated(&self, last_ac: String) -> bool {
        let mut date = self.extra_text.clone();
        let date_str = chrono::Utc::now().format(&self.date_format).to_string();
        date.push_str(&date_str);

        date == last_ac
    }
}

#[derive(Debug, Deserialize)]
struct LineNotifier {
    token: String,
    message: String,
}

impl LineNotifier {
    async fn send(&self) {
        let line_notifier = LineNotify::new(&self.token);
        match line_notifier.set_message(&self.message).send().await {
            Ok(_) => (),
            Err(e) => panic!("Failed to send line message: {}", e),
        }
    }
}
