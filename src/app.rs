use super::config::Configuration;
use super::cve::Cve;
use console::style;
use serde_json::json;

pub fn run() {
    // Instantiate Config
    let config = Configuration::new();
    let start_time: String = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    println!("[+] Starting @ {}", style(start_time.clone()).green());

    let mut last_timestamp: String = start_time.clone();
    let mut last_cves: Vec<Cve> = Vec::new();

    loop {
        // Check for CVEs
        println!("[+] Checking for new CVEs...");
        let cves = check(last_timestamp.clone());

        if cves.len() == 0 {
            println!("[+] No new CVEs found. Waiting for {} minutes before next check...",style(&config.time.to_string()).green());
            // Update the starting timestamp for the next check
            last_timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            std::thread::sleep(std::time::Duration::from_secs(&config.time * 60));
            continue;
        } else {
            // Check that that the new CVEs are not in the previous list & contain at least one of the keywords
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
            message(&config.webhook, &cves);
            println!("[+] CVE's Sent. Waiting for {} minutes before next check...", style(&config.time.to_string()).green());

            // Update the list of previous CVE's for later comparison
            last_cves = cves;

            // Update the last timestamp for the next check
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
            i["cve"]["id"].to_string(),
            i["cve"]["descriptions"][0]["value"].to_string(),
            i["cve"]["publishedDate"].to_string(),
        );

        cves.push(cve);
    }
    cves
}

fn message(webhook: &String, cves: &Vec<Cve>) {
    // Instantiate Discord Webhook
    let client = reqwest::Client::new();

    // Create an embed of the new CVEs
    let embed = json!({
        "embeds": [
            {
                "title": "New CVEs",
                "description": cves.iter().map(|cve| format!("[{}]({})", cve.id, cve.description)).collect::<Vec<String>>().join("\n"),
                "color": 16711680
            }
        ]
    });

    // Send the embed to the Discord Webhook
    let result = client.post(webhook).json(&embed).send();

}