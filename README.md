This is a very simple little uptime monitor cli I use to monitor my main website.
It sends a small console message warning if anything other than status code `200`
is recieved.

The code uses [ureq](https://github.com/algesten/ureq) which is a non async
`simple, safe HTTP client`.

This repo includes a sample bash script to run it. Just replace the path
where you install the cli executable (I usually put such things in /usr/local/bin/).

I use `cron` to run this every hour.
For example, on debian:
`crontab -e` to open the crontab file
Add the following line to run it on the hour and create it's own log:
`0 * * * * /path/to/uptime.sh >> /var/log/uptime.log 2>&1`
