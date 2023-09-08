<h1 align="center">
  <br>
  <a>Discord CVE Feed</a>
  <br>
</h1>
<p align="center">
  Send updates on new and relevant CVEs directly to your Discord server
</p>

## What does it do?
`discord_cve_feed` is intended to be a simple way to keep track of new CVE's as their published based on your tech stack.
You can provide new 

## How does it work?

### Help Menu
```
Usage: discord_cve_feed [OPTIONS]

Options:
  -w, --webhook <WEBHOOK>    Discord Webhook URL [default: ]
  -k, --keywords <KEYWORDS>  A comma separated list of keywords to search for [default: ]
  -t, --time <TIME>          Time in minutes between each check [default: 15]
  -h, --help                 Print help
  -V, --version              Print version
```
### Example Use

```
.\discord_cve_feed -w '<DISCORD_WEBHOOK>' -t 10 -k 'sql,wordpress,xss,command execution,lfi,rfi,rce,web'

    ____  _                          __   _______    ________   ______              __
   / __ \(_)_____________  _________/ /  / ____/ |  / / ____/  / ____/__  ___  ____/ /
  / / / / / ___/ ___/ __ \/ ___/ __  /  / /    | | / / __/    / /_  / _ \/ _ \/ __  /
 / /_/ / (__  ) /__/ /_/ / /  / /_/ /  / /___  | |/ / /___   / __/ /  __/  __/ /_/ /
/_____/_/____/\___/\____/_/   \__,_/   \____/  |___/_____/  /_/    \___/\___/\__,_/
https://github.com/JBince                               ver: 0.1.0
[+] Starting @ 2023-09-08T20:22:57Z
[+] Checking for new CVEs...
[+] No new or relevant CVE's found. Waiting for 10 minutes before next check...
...
[+] Checking for new CVEs...
[+] Message Sent!
[+] Sent 4 new CVE's to Discord. Waiting for 10 minutes before next check...
```
![image](https://github.com/JBince/discord_cve_feed/assets/66454005/4c955533-6f5f-4ffa-8709-4a2ec2bb852d)


## Installation
```
git clone https://github.com/JBince/discord_cve_feed.git
cd discord_cve_feed
cargo build --release
```
### *You will need a Discord Webhook for this to work!*
In your Discord server:
**Server Settings** -> **Integrations** -> **Webhooks** -> **New Webook**
