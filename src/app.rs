use super::config::Configuration;
use super::cve::Cve;
use super::banner::print_banner;
use crate::config::State;
use console::style;
use reqwest::blocking::Client;
use serde_json::json;

pub fn run() {
    // Instantiate application state to store config, last timestamp and last set of CVE's
    let mut app_state = State::new();

    // Print Banner
    print_banner();

    println!(
        "[+] Starting @ {}",
        style(app_state.last_timestamp.clone()).green()
    );

    loop {
        println!("[+] Checking for new CVEs...");

        // Query CVE's & Filter Results
        let cves = filter(
            &app_state.config,
            query_cves(app_state.last_timestamp.clone()),
        );
        match cves.len() {
            0 => {
                println!("[+] No new or relevant CVE's found. Waiting for {} minutes before next check...", style(&app_state.config.time.to_string()).green());
                app_state.last_timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
                std::thread::sleep(std::time::Duration::from_secs(&app_state.config.time * 60));
                continue;
            }
            _ => {
                message(&app_state.config.webhook, &cves);
                println!(
                    "[+] Sent {} new CVE's to Discord. Waiting for {} minutes before next check...",
                    style(cves.len().to_string()).green(),
                    style(&app_state.config.time.to_string()).green()
                );
                app_state.last_timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
                std::thread::sleep(std::time::Duration::from_secs(&app_state.config.time * 60));
            }
        }
    }
}

#[tokio::main]
async fn query_cves(last_timestamp: String) -> Vec<Cve> {
    // Instantiate CVE Vector
    let mut cves: Vec<Cve> = Vec::new();

    // Get current time in ISO 8601 format
    let current_time = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    // Query new CVE's based on timestamps
    let url = format!(
        "https://services.nvd.nist.gov/rest/json/cves/2.0/?pubStartDate={}&pubEndDate={}",
        last_timestamp, current_time
    );
    let result = reqwest::get(&url).await;
    let body = result.unwrap().text().await.unwrap();

    // Get ID and Description of each CVE
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    for i in json["vulnerabilities"].as_array().unwrap().iter() {
        let cve = Cve::new(
            i["cve"]["id"].as_str().unwrap().to_string(),
            i["cve"]["descriptions"][0]["value"]
                .as_str()
                .unwrap()
                .to_string(),
        );

        cves.push(cve);
    }
    cves
}

fn filter(config: &Configuration, cves: Vec<Cve>) -> Vec<Cve> {
    let mut cves: Vec<Cve> = cves
        .into_iter()
        .filter(|cve| {
            let keywords: Vec<&str> = config.keywords.split(",").collect();
            // Makes sure it contains the keywords
            for keyword in keywords {
                if cve
                    .description
                    .to_lowercase()
                    .contains(keyword.to_lowercase().as_str())
                {
                    return true;
                }
            }
            return false;
        })
        .collect();

    // Sort and remove duplicates
    cves.sort();
    cves.dedup();
    return cves;
}

fn message(webhook: &String, cves: &Vec<Cve>) {
    // Instantiate JSON Object
    let mut json = json!({
        "username": "CVE Feed",
        "content": "New CVE's Found!",
        "embeds": []
    });
    // Add CVE's to JSON Object
    for cve in cves {
        json["embeds"].as_array_mut().unwrap().push(json!({
            "title": cve.id,
            "description": cve.description.as_str(),
            "url": format!("https://nvd.nist.gov/vuln/detail/{}", cve.id.as_str()),
            "color": 185877,
        }));
    }
    //Send the json to the Discord Webhook
    let request = Client::new().post(webhook).json(&json).send();
    match request {
        Ok(_) => println!("[+] Message Sent!"),
        Err(e) => println!("[-] Error Sending Message: {}", e),
    }
}
