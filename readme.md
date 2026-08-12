# Proxy-Page

## ⚠️ Before you go any further...

This is a hobby project. I built it in my spare time because I wanted to learn something, not because I was trying to build the next unicorn startup. If you were hoping for enterprise-grade, battle-tested, SOC 2 compliant software... this ain't it, chief.

Here's the deal:
- 🐛 **Bugs?** Probably. I wrote this for fun, not for a code review from a senior staff engineer.
- 🔒 **Security?** I did my best, but I'm not a security researcher, I'm a person who likes computers. Don't put your production database credentials anywhere near this thing.
- 📦 **Stability?** Ehh. It works on my machine™.
- 🚨 **Warranty?** lol. lmao, even.

Basically: if this thing breaks, deletes your files, summons a minor demon, or otherwise ruins your day, that's on you for trusting a hobbyist's side project. Use at your own risk, and maybe don't run it as root.

That said — I had fun building this, and if you find it useful or interesting, awesome! Bug reports and PRs are welcome, though my response time varies between "immediately" and "geological timescale."

## About

Supposed to be a TCP Proxy working in rust, accept a TCP connection (or connections), then echo those connections back to a different server.
Currently, only hardcoded servers are supported, but I will add the ability to modify the servers through a configuration file without needing to rebuild the app.

## Usage

Currently, servers both for sending and receiving need to be changed/hard-coded. After that, rebuild the app and run as normal.

## License

<!-- MIT? All rights reserved? You do you -->