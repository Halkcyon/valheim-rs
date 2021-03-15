use bindings::windows::win32::system_services::GenerateConsoleCtrlEvent;
use chrono::{Utc, SecondsFormat};
use crossbeam_channel::{Receiver, bounded};
use once_cell::sync::Lazy;
use regex::Regex;

const CTRL_BREAK_EVENT: u32 = 0x00000001;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?x)
(?P<month>\d{2})/(?P<day>\d{2})/(?P<year>\d{4})\s+
(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2}):\s+
(?P<text>.*)"
).unwrap());

pub fn ctrl_break(pid: u32) -> bool {
    println!("Emitting CTRL_BREAK to '{}'", pid);

    #[allow(unsafe_code)]
    unsafe {
        // If the function succeeds, the return value is nonzero.
        // https://docs.microsoft.com/en-us/windows/console/generateconsolectrlevent#return-value
        GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid).0 != 0
    }
}

pub fn ctrlc_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (tx, rx) = bounded(1);
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })?;

    Ok(rx)
}

pub fn process_line(s: &str) {
    if s.contains("(Filename:") || s.is_empty() {
        return;
    }

    let dt = if let Some(caps) = RE.captures(s) {
        (format!(
            "{}-{}-{}T{}:{}:{}.000Z",
            &caps["year"], &caps["month"], &caps["day"],
            &caps["hour"], &caps["minute"], &caps["second"]
        ), caps["text"].to_string())
    } else {
        (Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true), s.into())
    };

    println!("{} :: {}", dt.0, dt.1);
}
