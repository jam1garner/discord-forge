#![feature(try_trait)]
#![feature(label_break_value)]
extern crate serenity;

mod converter;
mod arc_commands;
mod message_helper;
use message_helper::MessageHelper;

use std::sync::{Arc, Mutex};
use std::process::Command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::env;
use std::fs::{File, self};
use std::io::Write;
use std::path::PathBuf;
use std::collections::BTreeSet;

#[derive(Default)]
struct Handler {
    channel_id: Arc<Mutex<BTreeSet<ChannelId>>>
}

impl Handler {
    pub fn new<It: IntoIterator<Item=ChannelId>>(channels: It) -> Handler {
        Handler {
            channel_id: Arc::new(Mutex::new(channels.into_iter().collect())),
            ..Default::default()
        }
    }
}

use converter::SUPPORTED_TYPES;

static HELP_TEXT: &str = 
"%help - display this message\n\
%set_channel - set the channel to watch\n\
%update - update param labels and install paramxml if not installed\n\
%thanks - credits\n\
\n\
Arc commands\n\
%ls [folder] - list files/folders in arc
%get [file] - request a file from the arc
%find_song [song name query] - list songs for a given name
%get_song [song name query] - download the first song from %find_song
\n\
Include 'start,end' or 'start-end' for looping in wav -> nus3audio conversions";

static THANKS_TEXT: &str = 
"jam1garner - bot programming, libnus3audio, mscdec/msclang, etc.\n\
Arthur (@BenArthur_7) - motion_list_rs, sqb-rs, and much more\n\
Moosehunter, jam1garner, Raytwo, soneek - VGAudio lopus support\n\
RandomTalkingBush, DemonSlayerX8, jam1garner - data.arc hashes
soneek - nus3audio help\n\
TNN, Genwald - WAV/audio help\n\
Ploaj, SMG (ScanMountGoat) - ArcCross, SSBHLib\n\
Arthur, Dr. Hypercake, Birdwards, SMG, Meshima, TNN, Blazingflare, TheSmartKid - Param labels\n\
coolsonickirby, SushiiZ - testing help";

enum SetUnset {
    Set,
    Unset
}

use SetUnset::*;

fn save_channels(channel_ids: &BTreeSet<ChannelId>, message: &MessageHelper, owner: &User) {
    if let Err(e) = fs::write(
        CHANNELS_PATH,
        channel_ids.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    ) {
        message.say(
            MessageBuilder::new()
                .mention(owner)
                .push(" Failed to save channel ids:")
                .push_codeblock_safe(e.to_string(), None)
                .build()
        );
    };
}

fn set_or_unset_channel(handler: &Handler, message: &MessageHelper, set: SetUnset) {
    let owner = message.get_current_application_info().owner;
    let is_admin = message.member_permissions().administrator();
    if message.author == owner || is_admin {
        let arc = Arc::clone(&handler.channel_id);
        let mut channel_ids = arc.lock().unwrap();
        match set {
            Set => {
                channel_ids.insert(message.channel_id);
                message.say("Channel set");
                save_channels(&channel_ids, message, &owner);
            }
            Unset => {
                if channel_ids.remove(&message.channel_id) {
                    message.say("Channel unset");
                } else {
                    message.say("Channel was not set");
                }
            }
        }
    } else {
        message.reply("You do not have the proper permissions to set the channel.");
    }
}

impl EventHandler for Handler {
    fn message(&self, context: Context, message: Message) {
        let message = MessageHelper::new(message, context);
        
        if message.author.bot {
            return;
        }
        
        if !message.content.is_empty() && &message.content[0..1] == "%" {
            match message.content[1..].trim() {
                "update" => {
                    update(&message);
                    return;
                }
                "set_channel" => set_or_unset_channel(self, &message, Set),
                "unset_channel" => set_or_unset_channel(self, &message, Unset),
                "help" => {
                    let _ =
                    message.say(
                        MessageBuilder::new()
                            .push("Version 1.3\nCommands:")
                            .push_codeblock_safe(HELP_TEXT, None)
                            .push(format!("Supported types: {}", SUPPORTED_TYPES))
                            .build()
                    );
                }
                "thanks" => {
                    let _ =
                    message.say(
                        MessageBuilder::new()
                            .push("A big thanks to everyone who has in anyway helped:")
                            .push_codeblock_safe(THANKS_TEXT, None)
                            .build()
                    );
                }
                s @ "ls" | s if s.starts_with("ls ") => arc_commands::ls(s, &message),
                s if s.starts_with("get ") => arc_commands::get(s, &message),
                s if s.starts_with("find_song ") => arc_commands::find_song(s, &message),
                s if s.starts_with("get_song ") => arc_commands::get_song(s, &message),
                _ => {
                    message.say("Invalid command");
                    return;
                }
            }
        }
        {
            let enabled_channels = Arc::clone(&self.channel_id);
            if !enabled_channels.lock().unwrap().contains(&message.channel_id) {
                return;
            }
        }
        for attachment in &message.attachments {
            let content = match attachment.download() {
                Ok(content) => content,
                Err(why) => {
                    println!("Error downloading attachment: {:?}", why);
                    message.say("Error downloading attachment");

                    return;
                },
            };
            let path: PathBuf = ["/tmp/converter/", &attachment.filename].iter().collect(); 

            match std::fs::create_dir_all(format!("/tmp/converter")) {
                Ok(()) => {}
                Err(why) => {
                    println!("Error creating dir: {:?}", why);
                    message.say("Error creating dir");
                }
            }
            let mut file = match File::create(path.as_os_str()) {
                Ok(file) => file,
                Err(why) => {
                    println!("Error creating file: {:?}", why);
                    message.say("Error creating file");

                    return;
                },
            };

            if let Err(why) = file.write(&content) {
                println!("Error writing to file: {:?}", why);

                return;
            }
            
            match converter::extension(path.as_path()) {
                "mscsb" | "c" | "wav" => {
                    message.broadcast_typing();
                }
                _ => {}
            }
            match converter::convert(path, &message.content) {
                Ok(path) => {
                    let _ =
                    message.send_file(path.to_str().unwrap(), "Converted file")
                        .map_err(|e|{
                            message.say(
                                MessageBuilder::new()
                                    .push("Error sendinfg file: ")
                                    .push_codeblock_safe(e.to_string(), None)
                                    .build()
                            );
                        });
                    std::fs::remove_file(path).unwrap();
                }
                Err(why) => {
                    println!("Error converting file: {:?}", why);
                    message.say(
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

const MOTION_LABEL_PATH: &str = "motion_list_labels.txt";
const SQB_LABEL_PATH: &str = "sqb_labels.txt";
const CHANNELS_PATH: &str = "channels.txt";

fn load_channels() -> Vec<ChannelId> {
    fs::read_to_string(CHANNELS_PATH).ok()
        .map(|channels_file|{
            channels_file.split('\n')
                .map(|s| u64::from_str_radix(s, 10))
                .filter_map(Result::ok)
                .map(Into::into)
                .collect()
        })
        .unwrap_or_default()
}

fn main() {
    arc_commands::setup_songs();
    update_labels(&[MOTION_LABEL_PATH, SQB_LABEL_PATH]);
    let channels = load_channels();

    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler::new(channels))
        .expect("Error creating client");

    //client.with_framework(StandardFramework::new()
    //    .configure(|c| c.prefix("%"))
    //    .cmd("update", update));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn update_labels(label_paths: &[&str]) {
    hash40::set_labels(
        label_paths
            .iter()
            .map(|label| hash40::read_labels(label).unwrap())
            .flatten()
    )
}

fn update(message: &MessageHelper) {
    let update_output = Command::new("sh")
        .arg("update.sh")
        .output()
        .expect("Failed to run update");
    if update_output.status.success() {
        let out = std::str::from_utf8(&update_output.stdout[..]).unwrap();
        message.say(out);
    } else {
        let err = std::str::from_utf8(&update_output.stderr[..]).unwrap();
        message.say(
            MessageBuilder::new()
                .push("Error:")
                .push_codeblock_safe(err, None)
                .build()
        );
    }
    update_labels(&[MOTION_LABEL_PATH, SQB_LABEL_PATH]);
}
