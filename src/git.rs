use std::collections::{HashMap, HashSet};
use std::io::{Read, ErrorKind, Write};
use std::process::Command;
use std::str;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

static SETTINGS_DIR: &str = ".gtr";

/// Selects only existing branches
pub fn select_exsiting_branches(dir: &str, branches: &Vec<&String>) -> Vec<String> {
    let availalbe: HashSet<String> = ls_remote(dir).into_keys().collect();
    let requested: HashSet<String> = branches.iter().map(|s| String::from("refs/heads/") + s).collect();
    return availalbe.intersection(&requested).into_iter().map(|s| String::from(s)).collect()
}

/// Checks if directory is a git repository, adds service folder to gitignore
pub fn setup(dir: &str) {
    is_git(dir);
    ignore_gtr(dir);
}

/// Returns hash of Ref for each branch of given repository as well as current HEAD
pub fn ls_remote(dir: &str) -> HashMap<String, String> {
    let refs = Command::new("git").arg("ls-remote").arg(dir).output().unwrap();
    let refs = String::from_utf8(refs.stdout).unwrap();
    return refs
        .split("\n")
        .into_iter()
        .filter(|r| !String::from("\n").eq(r) && !String::from("").eq(r))
        .map(|r| {
            let s: Vec<&str> = r.split("\t").collect();
            return (String::from(s[1]), String::from(s[0]))
        })
        .collect();
}

// TODO: add function for generating git pack. See:
// https://github.com/git/git/blob/b594c975c7e865be23477989d7f36157ad437dc7/Documentation/technical/pack-protocol.txt#L346-L393
pub fn upload_pack(dir: &str, want: &str, have: Option<&str>) {
    let git_dir_path = Path::new(dir).join("./dir");
    let mut pack_upload = Command::new("git-upload-pack").arg("--strict").arg(git_dir_path).spawn().unwrap();

    let message = create_pack_message(want, have);
    pack_upload.stdin.as_mut().unwrap().write(message.as_bytes()).unwrap();
}

// TODO: test me
/// Creates message to be written to git-upload-pack process stdin
fn create_pack_message(want: &str, have: Option<&str>) -> String {
    let mut message: String = "want ".to_owned();
    message.push_str(want);
    message.push_str("\n");
    match have {
        None => message.push_str("done\n"),
        Some(have) => {
            message.push_str("have ");
            message.push_str(have);
            message.push_str("\n");
            message.push_str("done\n");
        }
    }

    return message
}

/// Add .gtr directory to gitignore in provided repository
fn ignore_gtr(dir: &str) {
    let gitignore_path = Path::new(dir).join(".gitignore");
    match File::open(&gitignore_path) {
        Ok(mut file) => {
            let mut data = String::new();
            file.read_to_string(&mut data).expect("Can not read file content");

            let gtr_ignored = data.split("\n").into_iter().any(|s| String::from(SETTINGS_DIR).eq(s));
            if !gtr_ignored { store_in_gitignore(&gitignore_path); }
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => store_in_gitignore(&gitignore_path),
            _ => panic!("Unrecognized error {e}")
        }
    }
}

fn store_in_gitignore(gitignore_path: &PathBuf) {
    let store = |mut file: File| { file.write_all((String::from("\n") + SETTINGS_DIR).as_bytes()).unwrap() };

    match OpenOptions::new().write(true).append(true).open(gitignore_path) {
        Ok(file) => store(file),
        Err(_) => {
            let file = File::create(&gitignore_path).unwrap();
            OpenOptions::new().write(true).append(true).open(gitignore_path).unwrap();
            store(file);
        }
    }
}
/// Panics if provided directory is not a git repository
fn is_git(dir: &str) {
    if !Path::new(dir).join(".git").exists() {
        panic!("Not a git repository");
    }
}

