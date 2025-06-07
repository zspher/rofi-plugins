use rofi_plugin_sys as ffi;

#[allow(dead_code)]
struct Mode<'rofi> {
    entries: Vec<String>,
    api: rofi_mode::Api<'rofi>,
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "websearch\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("websearch");
        let dummy = vec!["test".to_string()];

        Ok(Self {
            api,
            entries: dummy,
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
}

rofi_mode::export_mode!(Mode);
