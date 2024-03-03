use headless_chrome::Browser;
use serde::Deserialize;
use std::error::Error;

fn main() {
    let _last_ac = LoadLastAC::new().load_last_ac().expect("Failed to load lastAC");
}

#[derive(Deserialize)]
struct LoadLastAC {
    user_name: String,
    address: String,
    user_id_selector: String,
    user_button_selector: String,
    last_ac_selector: String,
}

impl LoadLastAC {
    fn new() -> Self {
        let config_str = std::fs::read_to_string("config.toml").expect("Failed to read config.toml");
        let load_info: LoadLastAC = toml::from_str(&config_str).expect("Failed to parse config.toml");

        if load_info.user_name.is_empty() {
            panic!("User name is empty");
        }

        load_info
    }

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
