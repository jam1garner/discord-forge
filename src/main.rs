#[macro_use] extern crate serenity;

mod converter;

use std::process::Command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};
use serenity::framework::standard::StandardFramework;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, message: Message) {
        if message.author.bot {
            return;
        }
        for attachment in message.attachments {
            let content = match attachment.download() {
                Ok(content) => content,
                Err(why) => {
                    println!("Error downloading attachment: {:?}", why);
                    let _ = message.channel_id.say("Error downloading attachment");

                    return;
                },
            };
            let path: PathBuf = ["/tmp/converter/", &attachment.filename].iter().collect(); 

            match std::fs::create_dir_all(format!("/tmp/converter")) {
                Ok(()) => {}
                Err(why) => {
                    println!("Error creating dir: {:?}", why);
                    let _ = message.channel_id.say("Error creating dir");
                }
            }
            let mut file = match File::create(path.as_os_str()) {
                Ok(file) => file,
                Err(why) => {
                    println!("Error creating file: {:?}", why);
                    let _ = message.channel_id.say("Error creating file");

                    return;
                },
            };

            if let Err(why) = file.write(&content) {
                println!("Error writing to file: {:?}", why);

                return;
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

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("%"))
        .cmd("update", update));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(update(_context, message) {
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
});
