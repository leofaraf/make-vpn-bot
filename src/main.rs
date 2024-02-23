use telepages::prelude::*;

const START_MESSAGE: &str = r"<b>We will help make your VPN</b>

<b>Pros</b>
- security: only you'll be able read your data, completely opensource
- private: only you'll be using internet (only your devices for server ip) 
- user-friendly: you'll be able download app in windows, macos, android (playmarket), ios(appstore), linux. also you'll be able share your vpn with QR-code

<b>Cons:</b>
- cost: will be cost like 2-4$ per mounth (dependens on hosting that you'll choice for rent a serer)";
const SUMMARY_MESSAGE: &str = r"<b>Our steps:</b>

- Make a server
- Configure a vpn client
- Share it from pc to mobile";
const MAKE_SERVER_MESSAGE: &str = r"<b>Step 1: Make a server</b>
* If you're have a server you can skip this step

- Go to hosting site (like DigitalOcean or TimewebCloud)
- Rent a most cheap server in another location where will be your VPN
- Copy server data (ip, login, password)";
const CONFIGURE_VPN_MESSAGE: &str = r"<b>Step 2: Configure a vpn client</b>

- Go to vpn client <a href='https://amnezia.org/en'>website</a>
- Install on desktop (if're wanna easy share it to your mobile) and press on button 'Configure your server'
- Paste your data and press 'Continue' in a few minutes it'll be installed
- Press on 'Connect'. <a href='https://whatismyipaddress.com/'>Check your ip</a>. Are you in another location? Great!";
const SHARE_VPN_MESSAGE: &str = r"<b>Step 3: Share your vpn to mobile</b>

- Mobile: Intall <a href='https://amnezia.org/en'>same vpn client</a>
- Desktop: Click on second tab (share icon). Click to share.
- Mobile: Open vpn client and click to make server by QR code. Scan code from desktop. Great, just use it!";
const END_MESSAGE: &str = r"<b>Thanks for watching!</b>

- dev tg: https://t.me/leofaraf
- dev github: https://github.com/leofaraf";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = TelegramBot::from_env();

    bot.repl(vec![
        Page::builder()
            .text(Some(START_MESSAGE.into()))
            .build(),
        Page::builder()
            .text(Some(SUMMARY_MESSAGE.into()))
            .build(),
        Page::builder()
            .text(Some(MAKE_SERVER_MESSAGE.into()))
            .build(),
        Page::builder()
            .text(Some(CONFIGURE_VPN_MESSAGE.into()))
            .build(),
        Page::builder()
            .text(Some(SHARE_VPN_MESSAGE.into()))
            .build(),
        Page::builder()
            .text(Some(END_MESSAGE.into()))
            .build(),
    ]).await;
}