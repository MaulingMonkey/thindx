use mmrbi::{Command, CommandExt};

pub fn open(url: &str) {
    status!("Opening", "{}", url);

    let mut cmd = if cfg!(windows)      { Command::parse("cmd /C start \"\"").unwrap() }
    else if cfg!(target_os = "linux")   { Command::new("xdg-open") }
    else if cfg!(target_os = "macos")   { Command::new("open") }
    else                                { return error!("doc::insecure_open not implemented on this platform") };

    cmd.arg(url);
    let _ = cmd.status0().map_err(|err| error!("{}", err));
}
