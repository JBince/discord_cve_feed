use super::config::Configuration;
use super::cve::Cve;
use console::style;
use reqwest::blocking::Client;
use serde_json::json;

pub fn run() {

    // Instantiate Config & emtpy variables for comparison
    let config = Configuration::new();
    let start_time: String = String::from("2023-09-08T06:31:38Z");
    let mut last_timestamp: String = start_time.clone();
    let mut last_cves: Vec<Cve> = Vec::new();

    println!("[+] Starting @ {}", style(start_time.clone()).green());

    loop {
        // Check for CVEs
        println!("[+] Checking for new CVEs...");
        let cves = check(last_timestamp.clone());

        if cves.len() == 0 {
            println!(
                "[+] No new CVEs found. Waiting for {} minutes before next check...",
                style(&config.time.to_string()).green()
            );
            last_timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            std::thread::sleep(std::time::Duration::from_secs(&config.time * 60));
            continue;
        } else {
            // Filter CVEs & send new ones to Discord
            let cves = filter(&config, cves, last_cves);
            message(&config.webhook, &cves);
            println!(
                "[+] CVE's Sent. Waiting for {} minutes before next check...",
                style(&config.time.to_string()).green()
            );

            last_cves = cves;
            last_timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            std::thread::sleep(std::time::Duration::from_secs(&config.time * 60));
        }
    }
}

#[tokio::main]
async fn check(last_timestamp: String) -> Vec<Cve> {
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

    // Handle Response
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    for i in json["vulnerabilities"].as_array().unwrap().iter() {
        let cve = Cve::new(
            i["cve"]["id"].as_str().unwrap().to_string(),
            i["cve"]["descriptions"][0]["value"].as_str().unwrap().to_string(),
        );

        cves.push(cve);
    }
    cves
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

fn filter(config: &Configuration, cves: Vec<Cve>, last_cves: Vec<Cve>) -> Vec<Cve> {
    let cves: Vec<Cve> = cves
        .into_iter()
        .filter(|cve| {
            !last_cves.contains(cve) && {
                let keywords: Vec<&str> = config.keywords.split(",").collect();
                for keyword in keywords {
                    if cve.description.contains(keyword) {
                        return true;
                    }
                }
                return false;
            }
        })
        .collect();

    println!(
        "[+] Found {} new CVEs. Sending to discord...",
        style(cves.len().to_string()).green()
    );
    cves
}
