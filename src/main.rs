use headless_chrome::{Browser, LaunchOptionsBuilder};
use serde::Deserialize;
use serenity::{
    all::{ChannelId, Context, CreateMessage, EventHandler, GatewayIntents, Ready},
    async_trait, Client,
};
use std::{error::Error, fs, str::FromStr};

#[tokio::main]
async fn main() {
    let mut current_exe = std::env::current_exe().expect("Failed to get current exe path");
    current_exe.pop();
    std::env::set_current_dir(current_exe).expect("Failed to set directory");

    let config = Config::new();
    let load_last_ac = config.load_last_ac;
    let compare_dates = config.compare_dates;
    let discord_notifier = config.discord_notifier;

    let last_ac = load_last_ac.load_last_ac().expect("Failed to load lastAC");

    if !compare_dates.is_streak_updated(last_ac) {
        discord_notifier.send().await;
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    load_last_ac: LoadLastAC,
    compare_dates: CompareDates,
    discord_notifier: DiscordNotifier,
}

impl Config {
    fn new() -> Self {
        let config_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");
        let config: Config = toml::from_str(&config_str).expect("Failed to parse config.toml");

        if config.load_last_ac.user_name.is_empty() {
            panic!("user name is empty");
        }
        if config.discord_notifier.token.is_empty() {
            panic!("Discord API token is empty");
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
        let browser = Browser::new(LaunchOptionsBuilder::default().build()?)?;
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
struct DiscordNotifier {
    token: String,
    channel_id: String,
    message: String,
}

impl DiscordNotifier {
    async fn send(&self) {
        let handler = Handler {
            channel_id: self.channel_id.clone(),
            message: self.message.clone(),
        };

        let mut client = Client::builder(&self.token, GatewayIntents::empty())
            .event_handler(handler)
            .await
            .expect("Failed to create client");

        // 10秒後にclientをシャットダウン
        let shard_manager = client.shard_manager.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            shard_manager.shutdown_all().await;
        });

        client.start().await.expect("Failed to start event handler");
    }
}

struct Handler {
    channel_id: String,
    message: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let channel_id = ChannelId::from_str(&self.channel_id).expect("Failed to parse channel id");
        channel_id
            .send_message(&ctx.http, CreateMessage::new().content(&self.message))
            .await
            .expect("Failed to send message");
    }
}
