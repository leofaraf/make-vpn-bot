use std::default;

use teloxide::{dispatching::dialogue::{self, InMemStorage}, dptree::di, handler, prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup, MessageId, ParseMode}};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

const BUTTON_BACK: &str = "Back";
const BUTTON_START: &str = "Let's start!";
const BUTTON_CONTINUE: &str = "Continue";

// Powered by MarkdownV2
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
    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(Update::filter_message().endpoint(start))
            .branch(Update::filter_callback_query().endpoint(pages))
    )
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn start(bot: Bot, msg: Message) -> HandlerResult {
    log::info!("start");
    bot.send_message(msg.chat.id, START_MESSAGE)
        .reply_markup(start_makeup())
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}

fn start_makeup() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new([
        [
            InlineKeyboardButton::callback(BUTTON_START, "1")
        ]
    ])
}

fn back_makeup(last_index: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new([
        [
            InlineKeyboardButton::callback(BUTTON_BACK, last_index)
        ]
    ])
}

fn makeup(back_index: &str, continue_index: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new([
        [
            InlineKeyboardButton::callback(BUTTON_BACK, back_index),
            InlineKeyboardButton::callback(BUTTON_CONTINUE, continue_index)
        ]
    ])
}

async fn pages(bot: Bot, callback: CallbackQuery) -> HandlerResult {
    let data = callback.data.unwrap();
    let msg = callback.message.unwrap();

    match data.as_str() {
        "0" => {
            bot.edit_message_text(msg.chat.id, msg.id, START_MESSAGE)
                .reply_markup(start_makeup())
                .parse_mode(ParseMode::Html)
                .await?;
        },
        "1" => {
            bot.edit_message_text(msg.chat.id, msg.id, SUMMARY_MESSAGE)
                .reply_markup(makeup("0", "2"))
                .parse_mode(ParseMode::Html)
                .await?;
        },
        "2" => {
            bot.edit_message_text(msg.chat.id, msg.id, MAKE_SERVER_MESSAGE)
                .reply_markup(makeup("1", "3"))
                .parse_mode(ParseMode::Html)
                .await?;
        },
        "3" => {
            bot.edit_message_text(msg.chat.id, msg.id, CONFIGURE_VPN_MESSAGE)
                .reply_markup(makeup("2", "4"))
                .parse_mode(ParseMode::Html)
                .await?;
        },
        "4" => {
            bot.edit_message_text(msg.chat.id, msg.id, SHARE_VPN_MESSAGE)
                .reply_markup(makeup("3", "5"))
                .parse_mode(ParseMode::Html)
                .await?;
        },
        "5" => {
            bot.edit_message_text(msg.chat.id, msg.id, END_MESSAGE)
                .reply_markup(back_makeup("4"))
                .parse_mode(ParseMode::Html)
                .await?;
        },
        _ => {}
    };

    bot.answer_callback_query(callback.id).await?;
    Ok(())
}