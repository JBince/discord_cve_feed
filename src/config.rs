use clap::Parser;

pub struct State {
    pub config: Configuration,
    pub last_timestamp: String,
}

impl State {
    pub fn new() -> Self {
        State {
            config: Configuration::new(),
            last_timestamp: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    // Path to Discord Webhook
    #[arg(short, long, help = "Discord Webhook URL", required = false, default_value = "")]
    pub webhook: String,

    // Comma separated list of keywords
    #[arg(short, long, required = false, help = "A comma separated list of keywords to search for", default_value = "")]
    pub keywords: String,

    // Time in minutes between each check
    #[arg(short, long, required = false, help = "Time in minutes between each check", default_value_t = 15)]
    pub time: u64,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration::parse()
    }
}