use serenity::utils::MessageBuilder;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashMap;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::sync::RwLock;
use lazy_static::lazy_static;
use super::MessageHelper;
use hash40::to_hash40;

static ARC_HASH_STRINGS: &str = include_str!("hash40s.tsv");
static PARAM_HASH_STRINGS: &str = include_str!("ParamLabels.csv");

lazy_static! {
    static ref ARC_NAMES: HashMap<u64, &'static str> = {
        ARC_HASH_STRINGS
            .split('\n')
            .filter_map(|line|{
                let split: Vec<&'static str> = line.split('\t').collect();
                if let &[hash, string] = &split[..] {
                    Some((u64::from_str_radix(hash, 16).ok()?, string))
                } else {
                    None
                }
            })
            .chain(
                ARC_HASH_STRINGS
                    .split('\n')
                    .filter_map(|line|{
                        let split: Vec<&'static str> = line.split('\t').collect();
                        if let &[hash, string] = &split[..] {
                            Some((u64::from_str_radix(hash, 16).ok()? & 0xFFFF_FFFF, string))
                        } else {
                            None
                        }
                    })
            )
            .collect()
    };

    static ref PARAM_NAMES: HashMap<u64, &'static str> = {
        PARAM_HASH_STRINGS
            .split('\n')
            .filter_map(|line|{
                let split: Vec<&'static str> = line.split(',').collect();
                if let &[hash, string] = &split[..] {
                    let hash = hash.trim_start_matches("0x");
                    Some((u64::from_str_radix(hash, 16).ok()?, string))
                } else {
                    None
                }
            })
            .chain(
                PARAM_HASH_STRINGS
                    .split('\n')
                    .filter_map(|line|{
                        let split: Vec<&'static str> = line.split(',').collect();
                        if let &[hash, string] = &split[..] {
                            let hash = hash.trim_start_matches("0x");
                            Some((u64::from_str_radix(hash, 16).ok()? & 0xFFFF_FFFF, string))
                        } else {
                            None
                        }
                    })
            )
            .collect()
    };
}

pub fn hash(s: &str, message: &MessageHelper) {
    let text = (&s[5..]).trim();

    if text == "check_param_hashes" {
        message.say(format!("Loaded {} param hashes", PARAM_NAMES.len()));
    }

    if text.starts_with("0x") || text.chars().all(|c| c.is_ascii_hexdigit()) {
        let text = text.trim_start_matches("0x");
        if let Ok(num) = u64::from_str_radix(text, 16) {
            let arc_str = ARC_NAMES.get(&num);
            let param_str = PARAM_NAMES.get(&num);

            if arc_str.is_none() && param_str.is_none() {
                message.say(format!("No matching string found for hash 0x{:X}", num));
            } else {
                if let Some(arc_str) = arc_str {
                    message.say(
                        MessageBuilder::new()
                            .push("ARC match:")
                            .push_codeblock_safe(arc_str, None)
                            .build()
                    );
                }
                if let Some(param_str) = param_str {
                    message.say(
                        MessageBuilder::new()
                            .push("Param match:")
                            .push_codeblock_safe(param_str, None)
                            .build()
                    );
                }
            }
        } else {
            message.say("Failed to parse. Invalid hex literal.");
        }
    } else {
        let text = text.trim_matches('"');
        let hash = to_hash40(text);
        let hash = ((hash.strlen() as u64) << 0x20) + hash.crc() as u64;
        message.say(
            MessageBuilder::new()
                .push_codeblock_safe(format!("0x{:x}", hash), None)
                .build()
        );
    }
}

const SONG_NAME_CSV: &str = include_str!("song_name_to_file.tsv");

lazy_static! {
    static ref SONG_NAME_TO_FILE: RwLock<Option<HashMap<&'static str, Vec<String>>>>
        = RwLock::new(None);
}


pub fn setup_songs() {
    let s: HashMap<_, _> =
        SONG_NAME_CSV
            .split('\n')
            .filter_map(|a| {
                let a: Vec<&'static str> = a.split('\t').collect();
                if a.len() < 2 {
                    None
                } else {
                    let key = a[0];
                    let values = a[1..].iter()
                                        .map(|s| format!("bgm_{}.nus3audio", s))
                                        .collect();
                    Some((key, values))
                }
            })
            .collect();

    *SONG_NAME_TO_FILE.write().unwrap() = Some(s);
}

fn to_arc_path(s: &str) -> Option<PathBuf> {
    let path = Path::new(s);
    if path.components().any(|c| c == std::path::Component::ParentDir) {
        return None;
    }

    let path: PathBuf = path.components()
        .filter(|c| c != &std::path::Component::RootDir)
        .collect();

    Some([Path::new("/arc"), &path].iter().collect())
}


pub fn ls(s: &str, message: &MessageHelper) {
    let args: Vec<_> = s[2..].trim().split(' ').collect();
    let (path, page) = if let &[path] = args.as_slice() {
        (path, 1usize)
    } else if let &[path, page, ..] = args.as_slice() {
        (
            path,
            match usize::from_str_radix(page, 10) {
                Ok(page) => page,
                Err(_) => {
                    message.say(
                        &MessageBuilder::new()
                            .push("Error:")
                            .push_codeblock_safe("Invalid page number. Use format 'ls [path] [page]'", None)
                            .build()
                    );
                    return;
                }
            }
        )
    } else {
        message.say(
            MessageBuilder::new()
                .push("Error:")
                .push_codeblock_safe("Invalid page number. Use format 'ls [path] [page]'", None)
                .build()
        );
        return;
    };
    
    let path = match to_arc_path(path.trim()) {
        Some(path) => path,
        None => {
            message.say(
                MessageBuilder::new()
                    .push("Error:")
                    .push_codeblock_safe("Invalid path", None)
                    .build()
            );
            return;
        }
    };

    const NUM_LINES: usize = 15;

    let (pages, result) =
        Command::new("ls").arg("-lh").arg(&path).output()
            .map(|out|{
                let output = String::new() +
                    std::str::from_utf8(&out.stdout).unwrap() +
                    std::str::from_utf8(&out.stderr).unwrap();
                let output = output.split('\n').collect::<Vec<_>>();
                let line_count = output.len();
                let page_count = (line_count + (NUM_LINES - 1)) / NUM_LINES;
                (
                    page_count,
                    output
                        .into_iter()
                        .skip((page - 1) * NUM_LINES)
                        .take(NUM_LINES)
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            })
            .unwrap_or_else(|e| (0, e.to_string()));

    message.say(
        MessageBuilder::new()
            .push(path.to_str().unwrap())
            .push(format!(" Page {}/{}", page, pages))
            .push_codeblock_safe(result, None)
            .build()
    );
}

pub fn get(s: &str, message: &MessageHelper) {
    let path = match to_arc_path(s[3..].trim()) {
        Some(path) => path,
        None => {
            message.say(
                MessageBuilder::new()
                    .push("Error:")
                    .push_codeblock_safe("Invalid path", None)
                    .build()
            );
            return;
        }
    };

    message.send_file(path.to_str().unwrap(), path.to_str().unwrap())
        .map_err(|e| {
            message.say(
                MessageBuilder::new()
                    .push(format!("Error getting '{}':", path.to_str().unwrap()))
                    .push_codeblock_safe(e.to_string(), None)
                    .build()
            );
        })
        .unwrap();
}

pub fn find_song(s: &str, message: &MessageHelper) {
    let name = s[9..].trim().trim_matches('"');
    let matcher = SkimMatcherV2::default();
    let song_name_to_file = 
        SONG_NAME_TO_FILE
            .read()
            .unwrap();
    let song_name_to_file = 
        song_name_to_file
            .as_ref()
            .unwrap();
    let mut songs: Vec<(i64, &str)> =
        song_name_to_file
            .keys()
            .filter_map(|song_name|{
                matcher.fuzzy_match(song_name, name)
                    .map(|score| (score, *song_name))
            })
            .collect();

    songs.sort_by_cached_key(|a| -a.0);

    const LINES: usize = 15;

    if songs.is_empty() {
        message.say(format!("Song '{}' not found", name));
        return;
    }

    let songs = songs
        .into_iter()
        .take(LINES)
        .map(|a|{
            format!(
                "{} - {}",
                a.1,
                song_name_to_file
                    .get(a.1)
                    .unwrap()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    message.say(
        MessageBuilder::new()
            .push(format!("Found for '{}':", name))
            .push_codeblock_safe(songs, None)
            .build()
    );
}

pub fn get_song(s: &str, message: &MessageHelper) {
    let name = s[8..].trim().trim_matches('"');
    let matcher = SkimMatcherV2::default();
    let song_name_to_file = 
        SONG_NAME_TO_FILE
            .read()
            .unwrap();
    let song_name_to_file = 
        song_name_to_file
            .as_ref()
            .unwrap();
    let mut songs: Vec<(i64, &str)> =
        song_name_to_file
            .keys()
            .filter_map(|song_name|{
                matcher.fuzzy_match(song_name, name)
                    .map(|score| (score, *song_name))
            })
            .chain(song_name_to_file.keys().filter(
                |s| s.trim() == name.trim()).map(|a| (std::i64::MAX - 1, *a))
            )
            .collect();
    
    if songs.is_empty() {
        message.say(format!("Song '{}' not found", name));
        return;
    }

    songs.sort_by_cached_key(|a| -a.0);

    let file_names = song_name_to_file.get(songs[0].1).unwrap();

    let file_names: Vec<_> = 
        file_names
            .into_iter()
            .map(|file_name| format!("/arc/stream:/sound/bgm/{}", file_name))
            .collect();
    let file_names: Vec<_> = file_names.iter().map(|a| &**a).collect();
    dbg!(&file_names);

    for (i, file_name) in file_names.into_iter().enumerate() {
        let content = if i == 0 {
            format!("Song '{}':", name)
        } else {
            String::new()
        };
        message.send_file(
            file_name,
            content
        ).map_err(|e| {
            message.say(
                MessageBuilder::new()
                    .push(format!("Error uploading '{}':", file_name))
                    .push_codeblock_safe(e.to_string(), None)
                    .build()
            );
        })
        .unwrap();
    }
}
