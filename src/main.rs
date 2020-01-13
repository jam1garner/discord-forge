#![feature(try_trait)]
#![feature(label_break_value)]
extern crate serenity;

mod converter;

use std::sync::{Arc, Mutex};
use std::process::Command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};
use std::collections::BTreeSet;

#[derive(Default)]
struct Handler {
    channel_id: Arc<Mutex<BTreeSet<ChannelId>>>
}

impl Handler {
    pub fn new() -> Handler {
        Default::default()
    }
}

use converter::SUPPORTED_TYPES;

static HELP_TEXT: &str = 
"%help - display this message\n\
%set_channel - set the channel to watch\n\
%update - update param labels and install paramxml if not installed\n\
%donate - information on donations";

impl EventHandler for Handler {
    fn message(&self, _context: Context, message: Message) {
        if message.author.bot {
            return;
        }
        if !message.content.is_empty() && &message.content[0..1] == "%" {
            match &message.content[1..] {
                "update" => {
                    update(message);
                    return;
                }
                "set_channel" => {
                    let owner = serenity::http::raw::get_current_application_info()
                                .unwrap()
                                .owner;
                    if message.author == owner {
                        let arc = Arc::clone(&self.channel_id);
                        let mut channel_ids = arc.lock().unwrap();
                        channel_ids.insert(message.channel_id);
                        message.channel_id.say("Channel set").unwrap();
                    } else {
                        let _ = message.reply("You do not have the proper permissions to set the channel.");
                    }
                }
                "help" => {
                    let _ =
                    message.channel_id.say(
                        MessageBuilder::new()
                            .push("Version 1.3\nCommands:")
                            .push_codeblock_safe(HELP_TEXT, None)
                            .push(format!("Supported types: {}", SUPPORTED_TYPES))
                            .build()
                    );
                }
                "donate" => {
                    let _ =
                    message.channel_id.say(
                        MessageBuilder::new()
                            .push("Donation Options:\nhttps://ko-fi.com/jam1garner\nhttps://paypal.me/jam1garner\nAny donations to help cover server costs are much appreciated.")
                            .build()
                    );
                } 
                _ => {}
            }
        }
        {
            let enabled_channels = Arc::clone(&self.channel_id);
            if !enabled_channels.lock().unwrap().contains(&message.channel_id) {
                return;
            }
        }
        for attachment in message.attachments {
            let content = match attachment.download() {
                Ok(content) => content,
                Err(why) => {
                    println!("Error downloading attachment: {:?}", why);
                    message.channel_id.say("Error downloading attachment").unwrap();

                    return;
                },
            };
            let path: PathBuf = ["/tmp/converter/", &attachment.filename].iter().collect(); 

            match std::fs::create_dir_all(format!("/tmp/converter")) {
                Ok(()) => {}
                Err(why) => {
                    println!("Error creating dir: {:?}", why);
                    message.channel_id.say("Error creating dir").unwrap();
                }
            }
            let mut file = match File::create(path.as_os_str()) {
                Ok(file) => file,
                Err(why) => {
                    println!("Error creating file: {:?}", why);
                    message.channel_id.say("Error creating file").unwrap();

                    return;
                },
            };

            if let Err(why) = file.write(&content) {
                println!("Error writing to file: {:?}", why);

                return;
            }
            
            match converter::extension(path.as_path()) {
                "mscsb" | "c" => {
                    let _ = message.channel_id.broadcast_typing();
                }
                _ => {}
            }
            match converter::convert(path) {
                Ok(path) => {
                    let _ = message.channel_id.send_files(vec![path.to_str().unwrap()], |m| m
                        .content("Converted file")
                    );
                    std::fs::remove_file(path).unwrap();
                }
                Err(why) => {
                    println!("Error converting file: {:?}", why);
                    let _ = message.channel_id.say(
                        MessageBuilder::new()
                            .push("Error converting file:")
                            .push_codeblock_safe(why.message, None)
                            .build()
                    );
                }
            }
        }
    }
}

const LABEL_PATH: &str = "motion_list_labels.txt";

fn main() {
    update_labels(LABEL_PATH);

    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler::new())
        .expect("Error creating client");

    //client.with_framework(StandardFramework::new()
    //    .configure(|c| c.prefix("%"))
    //    .cmd("update", update));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn update_labels(label_path: &str) {
    match hash40::read_labels(label_path) {
        Ok(labels) => hash40::set_labels(labels),
        Err(e) => println!("Error loading labels: {}", e),
    }
}

fn update(message: Message) {
    let update_output = Command::new("sh")
        .arg("update.sh")
        .output()
        .expect("Failed to run update");
    if update_output.status.success() {
        let out = std::str::from_utf8(&update_output.stdout[..]).unwrap();
        message.channel_id.say(out).unwrap();
    } else {
        let err = std::str::from_utf8(&update_output.stderr[..]).unwrap();
        message.channel_id.say(
            MessageBuilder::new()
                .push("Error:")
                .push_codeblock_safe(err, None)
                .build()
        ).unwrap();
    }
    update_labels(LABEL_PATH);
}
