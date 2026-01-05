use std::collections::{HashMap, HashSet};
use tokio::io::{BufReader, AsyncReadExt, ErrorKind, AsyncWriteExt};
use tokio::process::{Command, Child, ChildStdin, ChildStdout};
use std::process::Stdio;
use std::str;
use tokio::fs::{File, OpenOptions};
use std::path::PathBuf;

use crate::utils::error::{GtrResult, GitError};

pub const SETTINGS_DIR: &str = ".gtr";

/// Checks if directory is a git repository, adds service folder to gitignore
pub async fn gtr_setup(dir: &PathBuf) -> GtrResult<()> {
    if !is_git(dir) { return Err(GitError::not_git_repo(dir)) };

    ignore(dir, &SETTINGS_DIR).await?;
    Ok(())
}

/// Selects only existing branches
pub async fn select_exsiting_branches(dir: &str, branches: &Vec<&String>) -> GtrResult<Vec<String>> {
    let availalbe: HashSet<String> = ls_remote(dir).await?.into_keys().collect();
    let requested: HashSet<String> = branches
        .iter()
        .map(|s| String::from("refs/heads/") + s)
        .collect();
    return Ok(availalbe.intersection(&requested).into_iter().map(|s| String::from(s)).collect());
}

/// Returns hash of Ref for each branch of given repository as well as current HEAD
pub async fn ls_remote(dir: &str) -> GtrResult<HashMap<String, String>> {
    let refs = match Command::new("git").arg("ls-remote").arg(dir).output().await {
        Ok(refs) => String::from_utf8(refs.stdout).unwrap(),
        Err(e) => { return Err(GitError::command_failed(Box::new(e))) }
    };

    Ok(refs.split("\n")
        .into_iter()
        .filter(|r| !String::from("\n").eq(r) && !String::from("").eq(r))
        .map(|r| {
            let s: Vec<&str> = r.split("\t").collect();
            return (String::from(s[1]), String::from(s[0]))
        })
        .collect()
    )
}

/// Generates necessary pack files
pub async fn upload_pack(dir: &PathBuf, want: &str, have: Option<&str>) -> GtrResult<()> {
    let pack_upload = start_pack_upload_process(dir).await?;

    let mut stdin = pack_upload.stdin.unwrap();
    let stdout = pack_upload.stdout.unwrap();

    let mut buf = BufReader::new(stdout);
    let initial_pack_data = request_pack_file(&mut buf, &mut stdin, want, have).await?;
    write_pack_file(dir, want, initial_pack_data, &mut buf).await?;

    Ok(())
}

/// Start git-upload-pack server
async fn start_pack_upload_process(dir: &PathBuf) -> GtrResult<Child> {
    let git_dir = dir.join(".git");
    match Command::new("git-upload-pack")
        .arg("--strict")
        .arg(git_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
            Ok(res) => Ok(res),
            Err(e) => Err(GitError::command_failed(Box::new(e)))
        }
}

/// Read a single pkt-line from the git process's stdout.
async fn read_pkt_line(buf: &mut BufReader<ChildStdout>) -> GtrResult<Option<Vec<u8>>> {
    let mut len_bytes = [0; 4];

    // Read the first 4 bytes
    buf.read_exact(&mut len_bytes).await.map_err(|e| GitError::pack_read_failed(Box::new(e)))?;

    // Check if these bytes are the PACK header
    if &len_bytes == b"PACK" {
        // It's the start of the packfile, return these bytes directly
        return Ok(Some(len_bytes.to_vec()));
    }

    // If not PACK, proceed with pkt-line length parsing
    let len_str = str::from_utf8(&len_bytes).map_err(|e| {
        GitError::pack_read_failed(Box::new(e))
    })?;
    let len = usize::from_str_radix(len_str, 16).map_err(|e| {
        GitError::pack_read_failed(Box::new(e))
    })?;

    if len == 0 { // '0000' pkt-line
        return Ok(None);
    }

    // The length includes the 4 bytes for the length itself, so subtract them
    let data_len = len - 4;
    let mut data = vec![0; data_len];
    buf.read_exact(&mut data).await.map_err(|e| GitError::pack_read_failed(Box::new(e)))?;

    Ok(Some(data))
}

/// Identify git pack server nack response
fn wait_for_nak(line: &str) -> bool {
    return !line.eq("NAK")
}

/// Identify git pack server ack response
fn ack_objects_continue(line: &str) -> bool {
    // Regex is only used here, so it's imported locally.
    use regex::Regex;
    let ack_regex = Regex::new("^ACK").unwrap();
    let is_ack = ack_regex.is_match(line);
    let con_regex = Regex::new("continue$").unwrap();
    let is_con = con_regex.is_match(line);

    return is_ack && !is_con
}

/// Complete message sent to server for packfile negotiation
async fn write_message(want: &str, have: Option<&str>, stdin: &mut ChildStdin) {
    write_pack_line(&format!("want {}", want), stdin).await;
    write_pack_line("", stdin).await;
    if let Some(have) = have {
        write_pack_line(&format!("have {}", have), stdin).await;
        write_pack_line("", stdin).await;
    }
    write_pack_line("done", stdin).await;
}

/// Write line to stdin for git pack communication
async fn write_pack_line(line: &str, stdin: &mut ChildStdin) {
    if "".eq(line) {
        stdin.write_all(String::from("0000").as_bytes()).await.unwrap()
    } else {
        let message = format!("{0:04x}{1}\n", line.as_bytes().len() + 4 + 1, line);
        stdin.write_all(message.as_bytes()).await.unwrap();
    }
}

/// Talk to git-upload-pack until it is ready to send pack files
// NOTE: https://github.com/git/git/blob/b594c975c7e865be23477989d7f36157ad437dc7/Documentation/technical/pack-protocol.txt#L346-L393
// NOTE: this is worth reading: https://github.com/git/git/blob/ebba6c0ca617352ceef5caa636ab243f0ef14cc3/Documentation/technical/pack-heuristics.txt
async fn request_pack_file(
    buf: &mut BufReader<ChildStdout>,
    stdin: &mut ChildStdin,
    want: &str,
    have: Option<&str>) -> GtrResult<Option<Vec<u8>>> // Now returns Option<Vec<u8>>
{
    let mut expect_nack = false;
    loop {
        match read_pkt_line(buf).await? {
            Some(data) => {
                // Check if it's the packfile header (first 4 bytes are 'PACK')
                if is_pack_header(&data) {
                    return Ok(Some(data)); // Return the pack header bytes
                }
                
                let line = String::from_utf8(data).map_err(|e| GitError::pack_read_failed(Box::new(e)))?;

                if !expect_nack { // Not expecting NAK yet, still capabilities
                    continue;
                }

                if let Some(_) = have { ack_objects_continue(&line); } else { wait_for_nak(&line); };
            },
            None => { // '0000' pkt-line received, end of capabilities/message
                if !expect_nack {
                    write_message(want, have, stdin).await;
                    expect_nack = true;
                } else {
                    break;
                }
            }
        }
    }

    Ok(None) // Negotiation complete, no pack header read yet
}

/// Store pack file to fs
async fn write_pack_file(dir: &PathBuf, want:  &str, initial_pack_data: Option<Vec<u8>>, buf: &mut BufReader<ChildStdout>) -> GtrResult<()> {
    let mut pack_content = initial_pack_data.unwrap_or_default(); // Prepend initial data if any
    match buf.read_to_end(&mut pack_content).await {
        Err(e) => return Err(GitError::pack_write_failed(Box::new(e))),
        Ok(_) => {
            let file_path = dir.join(format!("{want}.pack"));
            let mut file = File::create(file_path).await.unwrap();
            file.write_all(&pack_content).await.unwrap();
        }
    };

    Ok(())
}

fn is_pack_header(data: &[u8]) -> bool {
    data.len() >= 4 && &data[0..4] == b"PACK"
}


/// Add .gtr directory to gitignore in provided repository
async fn ignore(dir: &PathBuf, to_ignore: &str) -> GtrResult<()> {
    let gitignore_path = dir.join(".gitignore");
    match File::open(&gitignore_path).await {
        Ok(mut file) => {
            let mut data = String::new();
            file.read_to_string(&mut data).await.expect("Can not read file content");

            let gtr_ignored = data.split("\n").into_iter().any(|s| to_ignore.eq(s));
            if !gtr_ignored {
                store_in_gitignore(&gitignore_path, to_ignore).await?;
            }

            Ok(())
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                store_in_gitignore(&gitignore_path, to_ignore).await?;
                Ok(())
            },
            _ => return Err(GitError::ignore_failed(Box::new(e)))
        }
    }
}

/// Add gtr related files to gitignore
async fn store_in_gitignore(gitignore_path: &PathBuf, to_ignore: &str) -> GtrResult<()> {
    match OpenOptions::new().write(true).append(true).open(gitignore_path).await {
        Ok(mut file) => file.write_all((String::from("\n") + to_ignore).as_bytes()).await.unwrap(),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let mut file = File::create(&gitignore_path).await.unwrap();
                OpenOptions::new().write(true).append(true).open(gitignore_path).await.unwrap();
                file.write_all((String::from("\n") + to_ignore).as_bytes()).await.unwrap();
            },
            _ => return Err(GitError::ignore_failed(Box::new(e)))
        }
    }

    return Ok(())
}
/// Checks if provided directory is a git repository
pub fn is_git(dir: &PathBuf) -> bool {
    dir.join(".git").exists()
}

// 
// /// Clones git repo to bare repo with the same name + .git
// /// assumes that provided path points to git repo
// async fn clone_bare(dir: &PathBuf) -> GtrResult<String> {
//     // TODO:
//     // get dir name from path which can be either absolute or relative
//     // git  clone --bare ${dir} ${dir}.git
// }