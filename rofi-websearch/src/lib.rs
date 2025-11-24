mod search;
use std::io;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

use search::SearchSitesData;

use rofi_mode::{Action, Event};
use rofi_plugin_sys as ffi;

struct Mode<'rofi> {
    previous_input: String,
    sites: SearchSitesData,
    entry: Box<str>,
    #[allow(dead_code)]
    api: rofi_mode::Api<'rofi>,
}

fn open_url(url: &str) {
    let mut cmd = Command::new("xdg-open");
    cmd.arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    // from: https://github.com/alacritty/alacritty/blob/b9c886872d1202fc9302f68a0bedbb17daa35335/alacritty/src/daemon.rs#L49
    let _ = unsafe {
        cmd.pre_exec(|| {
            match libc::fork() {
                -1 => return Err(io::Error::last_os_error()),
                0 => (),
                _ => libc::_exit(0),
            }

            if libc::setsid() == -1 {
                return Err(io::Error::last_os_error());
            }

            Ok(())
        })
        .spawn()
        .expect("Unable to launch url")
    }
    .wait();
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "websearch\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("websearch");

        let sites = SearchSitesData::init();
        let entry = "Search".into();

        Ok(Self {
            previous_input: String::new(),
            api,
            entry,
            sites,
        })
    }

    fn entries(&mut self) -> usize {
        1
    }

    fn entry_content(&self, _line: usize) -> rofi_mode::String {
        rofi_mode::format!("{}", self.entry)
    }

    fn react(&mut self, event: Event, input: &mut rofi_mode::String) -> Action {
        match event {
            Event::Ok {
                alt: _,
                selected: _,
            } => {
                if let Some(url) = self.sites.get_url_from_input(input) {
                    open_url(&url);
                }
                return Action::Exit;
            }
            Event::CustomCommand {
                number,
                selected: _,
            } => {
                return Action::SetMode(number as u16);
            }
            _ => (),
        }
        Action::Exit
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        if line == 0 {
            return true;
        }
        matcher.matches(&self.entry)
    }

    fn preprocess_input(&mut self, input: &str) -> rofi_mode::String {
        unsafe {
            ffi::view::reload();
        }
        self.previous_input = input.into();
        input.into()
    }

    fn message(&mut self) -> rofi_mode::String {
        self.sites
            .get_title_from_input(&self.previous_input)
            .unwrap_or(" ")
            .into()
    }
}

rofi_mode::export_mode!(Mode);
