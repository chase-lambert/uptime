# Simple Uptime Monitor CLI

This is a very simple little uptime monitor CLI I use to monitor my main website. It sends a simple console message warning if anything other than status code 200 is received.

## Features

- **Lightweight and Simple**: Utilizes [ureq](https://github.com/algesten/ureq), a non-async simple, safe HTTP client.

## Setup

This repo includes a sample bash script to run it. Just replace the path where you install the CLI executable (I usually put such things in `/usr/local/bin/).

### Running with Cron

I use cron to run this every hour. For example, on Debian:

1. Open the crontab file with `crontab -e`.
2. Add the following line to run it on the hour and create its own log:

```bash
0 * * * * /path/to/uptime.sh >> /var/log/uptime.log 2>&1

```
