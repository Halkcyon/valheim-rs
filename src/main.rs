//! Crate level docs!

#![deny(unsafe_code)]
#![warn(rust_2018_idioms)]
#![warn(rustdoc)]
#![warn(missing_docs)]
#![warn(clippy::all)]

mod control;
mod valheim;

use anyhow::Result;
use crossbeam_channel::select;
use rpassword::prompt_password_stdout;
use std::{
    env, fs,
    io::{self, BufRead, BufReader},
    os::windows::process::CommandExt,
    process::{Command, Stdio},
    thread,
    time::Duration,
};
use structopt::StructOpt;
use tui::backend::CrosstermBackend;
use tui::Terminal;

const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

// TODO! parse logs and emit events to control UI elements

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let opts = dbg!(Opts::from_args());

    dbg!(opts.to_args("example"));

    let appid = fs::read_to_string(opts.path.to_string() + "\\steam_appid.txt")?;
    let appid = appid.trim();

    dbg!(&appid);

    let out = io::stdout();
    let backend = CrosstermBackend::new(out);
    let mut _terminal = Terminal::new(backend)?;

    let pwd = if let Ok(pwd) = env::var("VALHEIM_PWD") {
        pwd
    } else {
        prompt_password_stdout("password: ")?
    };

    assert!(pwd.len() >= 5);
    assert!(!opts.name.contains(&pwd));

    let ctrl_c = control::ctrlc_channel()?;

    let mut child = Command::new(opts.path.to_string() + "\\valheim_server.exe")
        .env_clear()
        .envs(env::vars_os())
        .env("SteamAppId", appid)
        .creation_flags(CREATE_NEW_PROCESS_GROUP)
        .stdout(Stdio::piped())
        .args(&opts.to_args(&pwd))
        .spawn()?;

    let pid = child.id();
    println!("PID: {}", pid);

    let stdout = BufReader::new(child.stdout.take().expect("failed to take stdout"));

    let t = thread::spawn(move || {
        for line in stdout.lines() {
            match line {
                Ok(line) => control::process_line(line.trim()),
                Err(err) => eprintln!("{:?}", err),
            }
        }
    });

    loop {
        select! {
            recv(ctrl_c) -> _ => {
                if !control::ctrl_break(pid) {
                    eprintln!("Sending CTRL_BREAK failed!");
                }

                break;
            }
            default(Duration::from_millis(500)) => {}
        }
    }

    t.join().ok();

    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "valheim",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Opts {
    /// Directory containing `valheim_server.exe`
    #[structopt(
        long,
        short,
        env = "VALHEIM_EXE_PATH",
        parse(from_os_str),
        default_value
    )]
    path: valheim::Path<valheim::Server>,

    /// Directory for `worlds/`, created if it doesn't exist
    #[structopt(
        long,
        short,
        env = "VALHEIM_SAVES_PATH",
        parse(from_os_str),
        default_value
    )]
    saves: valheim::Path<valheim::Saves>,

    /// Name of the server as it appears to Steam
    #[structopt(env = "VALHEIM_SERVER_NAME")]
    name: String,

    /// Savefile name
    #[structopt(long, short, env = "VALHEIM_WORLD_NAME", default_value = "Dedicated")]
    world: String,

    /// Port to bind the game server to
    #[structopt(long, short = "b", env = "VALHEIM_PORT", default_value)]
    port: valheim::UnreservedPort,

    /// Controls whether to display the server in the community list
    #[structopt(long, short)]
    enable_community_server: bool,

    #[structopt(skip)]
    password: Option<String>,
}

impl Opts {
    pub fn to_args(&self, pwd: &str) -> Vec<String> {
        vec![
            "-nographics".into(),
            "-batchmode".into(),
            "-name".into(),
            self.name.clone(),
            "-port".into(),
            self.port.to_string(),
            "-world".into(),
            self.world.clone(),
            "-password".into(),
            pwd.into(),
            "-savedir".into(),
            self.saves.to_string(),
            "-public".into(),
            (self.enable_community_server as u8).to_string(),
        ]
    }
}
