use std::ptr::null;

use ffi::PropertyType;
use ffi::helper::config_find_widget;
use rofi_plugin_sys::{self as ffi, helper::theme_find_property};

struct Config {
    test: Option<String>,
}

#[allow(dead_code)]
struct Mode<'rofi> {
    entries: Vec<String>,
    config: Option<Config>,
    api: rofi_mode::Api<'rofi>,
}

impl Config {
    fn new(mode_name: &str) -> Option<Config> {
        let config = Config::load_config(mode_name);
        if let Err(e) = config {
            eprintln!("{}", e)
        }
        config.ok()
    }
    fn load_config(mode_name: &str) -> Result<Config, &str> {
        unsafe {
            println!("{}", mode_name);
            let cfg = config_find_widget(mode_name.as_ptr() as *const i8, null(), 1);
            if cfg.is_null() {
                return Err("config not loaded");
            }
            let test = theme_find_property(cfg, PropertyType::String, c"test".as_ptr(), 1);
            if test.is_null() {
                return Err("field missing or invalid type");
            }
            Ok(Self {
                test: Some((*(*test).value.s.cast::<String>()).clone()),
            })
        }
    }
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "websearch\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("websearch");
        let dummy = vec!["test".to_string()];

        Ok(Self {
            api,
            entries: dummy,
            config: Config::new(Self::NAME),
        })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        let entry = &self.entries[line];
        rofi_mode::format!("data: {}", entry)
    }

    fn react(
        &mut self,
        event: rofi_mode::Event,
        input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {
        match event {
            rofi_mode::Event::Cancel { selected: _ } => return rofi_mode::Action::Exit,
            rofi_mode::Event::Ok {
                alt: false,
                selected,
            } => {
                println!("Selected option {:?}", self.entries[selected]);
                return rofi_mode::Action::Exit;
            }
            // rofi_mode::Event::Ok {
            //     alt: true,
            //     selected,
            // } => {
            //     self.api.set_display_name(&*self.entries[selected]);
            // }
            rofi_mode::Event::CustomInput {
                alt: false,
                selected: _,
            } => {
                if !input.to_string().is_empty() {
                    self.entries.push(input.into());
                }
                input.clear();
            }
            // rofi_mode::Event::CustomInput {
            //     alt: true,
            //     selected: _,
            // } => {
            //     self.api.replace_display_name(mem::take(input));
            // }
            rofi_mode::Event::DeleteEntry { selected } => {
                self.entries.remove(selected);
            }
            rofi_mode::Event::Complete {
                selected: Some(selected),
            } => {
                input.clear();
                input.push_str(&self.entries[selected]);
            }
            rofi_mode::Event::CustomCommand {
                number,
                selected: _,
            } => {
                return rofi_mode::Action::SetMode(number as u16);
            }
            rofi_mode::Event::Complete { .. } => {}
            _ => {}
        }
        rofi_mode::Action::Reload
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        matcher.matches(&self.entries[line])
    }

    fn preprocess_input(&mut self, input: &str) -> rofi_mode::String {
        unsafe {
            ffi::view::reload();
        }
        input.into()
    }

    fn message(&mut self) -> rofi_mode::String {
        match &self.config {
            Some(c) => c.test.clone().unwrap().into(),
            None => "".into(),
        }
    }
}

rofi_mode::export_mode!(Mode);
