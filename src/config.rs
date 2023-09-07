use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    // Path to Discord Webhook
    #[arg(short, long, help = "Discord Webhook URL", required = false, default_value = "")]
    pub webhook: String,

    // Path to JSON Config File
    #[arg(short, long, required = false, help = "The path to the JSON Config File", default_value = "")]
    pub config_file_path: String,

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