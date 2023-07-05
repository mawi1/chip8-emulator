use std::collections::HashMap;
use std::fs;

use anyhow::{anyhow, Context, Ok};
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode;

use chip8_emulator_lib::emulator::{self, Key};

#[derive(Deserialize, Debug)]
pub struct TomlConfig {
    pixel_size: u32,
    on_color: (u8, u8, u8),
    keys: TomlKeys,
}

impl TomlConfig {
    fn to_config(&self) -> anyhow::Result<Config> {
        let config = Config {
            pixel_size: self.pixel_size,
            on_color: self.on_color,
            keys: self.keys.to_keys()?,
        };
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlKeys {
    key_0: String,
    key_1: String,
    key_2: String,
    key_3: String,
    key_4: String,
    key_5: String,
    key_6: String,
    key_7: String,
    key_8: String,
    key_9: String,
    key_a: String,
    key_b: String,
    key_c: String,
    key_d: String,
    key_e: String,
    key_f: String,
}

impl TomlKeys {
    fn to_keys(&self) -> anyhow::Result<HashMap<VirtualKeyCode, emulator::Key>> {
        fn str_to_virtkeycode(s: &str) -> anyhow::Result<VirtualKeyCode> {
            match s {
                "Key1" => Ok(VirtualKeyCode::Key1),
                "Key2" => Ok(VirtualKeyCode::Key2),
                "Key3" => Ok(VirtualKeyCode::Key3),
                "Key4" => Ok(VirtualKeyCode::Key4),
                "Key5" => Ok(VirtualKeyCode::Key5),
                "Key6" => Ok(VirtualKeyCode::Key6),
                "Key7" => Ok(VirtualKeyCode::Key7),
                "Key8" => Ok(VirtualKeyCode::Key8),
                "Key9" => Ok(VirtualKeyCode::Key9),
                "Key0" => Ok(VirtualKeyCode::Key0),
                "A" => Ok(VirtualKeyCode::A),
                "B" => Ok(VirtualKeyCode::B),
                "C" => Ok(VirtualKeyCode::C),
                "D" => Ok(VirtualKeyCode::D),
                "E" => Ok(VirtualKeyCode::E),
                "F" => Ok(VirtualKeyCode::F),
                "G" => Ok(VirtualKeyCode::G),
                "H" => Ok(VirtualKeyCode::H),
                "I" => Ok(VirtualKeyCode::I),
                "J" => Ok(VirtualKeyCode::J),
                "K" => Ok(VirtualKeyCode::K),
                "L" => Ok(VirtualKeyCode::L),
                "M" => Ok(VirtualKeyCode::M),
                "N" => Ok(VirtualKeyCode::N),
                "O" => Ok(VirtualKeyCode::O),
                "P" => Ok(VirtualKeyCode::P),
                "Q" => Ok(VirtualKeyCode::Q),
                "R" => Ok(VirtualKeyCode::R),
                "S" => Ok(VirtualKeyCode::S),
                "T" => Ok(VirtualKeyCode::T),
                "U" => Ok(VirtualKeyCode::U),
                "V" => Ok(VirtualKeyCode::V),
                "W" => Ok(VirtualKeyCode::W),
                "X" => Ok(VirtualKeyCode::X),
                "Y" => Ok(VirtualKeyCode::Y),
                "Z" => Ok(VirtualKeyCode::Z),
                "Escape" => Ok(VirtualKeyCode::Escape),
                "F1" => Ok(VirtualKeyCode::F1),
                "F2" => Ok(VirtualKeyCode::F2),
                "F3" => Ok(VirtualKeyCode::F3),
                "F4" => Ok(VirtualKeyCode::F4),
                "F5" => Ok(VirtualKeyCode::F5),
                "F6" => Ok(VirtualKeyCode::F6),
                "F7" => Ok(VirtualKeyCode::F7),
                "F8" => Ok(VirtualKeyCode::F8),
                "F9" => Ok(VirtualKeyCode::F9),
                "F10" => Ok(VirtualKeyCode::F10),
                "F11" => Ok(VirtualKeyCode::F11),
                "F12" => Ok(VirtualKeyCode::F12),
                "F13" => Ok(VirtualKeyCode::F13),
                "F14" => Ok(VirtualKeyCode::F14),
                "F15" => Ok(VirtualKeyCode::F15),
                "F16" => Ok(VirtualKeyCode::F16),
                "F17" => Ok(VirtualKeyCode::F17),
                "F18" => Ok(VirtualKeyCode::F18),
                "F19" => Ok(VirtualKeyCode::F19),
                "F20" => Ok(VirtualKeyCode::F20),
                "F21" => Ok(VirtualKeyCode::F21),
                "F22" => Ok(VirtualKeyCode::F22),
                "F23" => Ok(VirtualKeyCode::F23),
                "F24" => Ok(VirtualKeyCode::F24),
                "Snapshot" => Ok(VirtualKeyCode::Snapshot),
                "Scroll" => Ok(VirtualKeyCode::Scroll),
                "Pause" => Ok(VirtualKeyCode::Pause),
                "Insert" => Ok(VirtualKeyCode::Insert),
                "Home" => Ok(VirtualKeyCode::Home),
                "Delete" => Ok(VirtualKeyCode::Delete),
                "End" => Ok(VirtualKeyCode::End),
                "PageDown" => Ok(VirtualKeyCode::PageDown),
                "PageUp" => Ok(VirtualKeyCode::PageUp),
                "Left" => Ok(VirtualKeyCode::Left),
                "Up" => Ok(VirtualKeyCode::Up),
                "Right" => Ok(VirtualKeyCode::Right),
                "Down" => Ok(VirtualKeyCode::Down),
                "Back" => Ok(VirtualKeyCode::Back),
                "Return" => Ok(VirtualKeyCode::Return),
                "Space" => Ok(VirtualKeyCode::Space),
                "Compose" => Ok(VirtualKeyCode::Compose),
                "Caret" => Ok(VirtualKeyCode::Caret),
                "Numlock" => Ok(VirtualKeyCode::Numlock),
                "Numpad0" => Ok(VirtualKeyCode::Numpad0),
                "Numpad1" => Ok(VirtualKeyCode::Numpad1),
                "Numpad2" => Ok(VirtualKeyCode::Numpad2),
                "Numpad3" => Ok(VirtualKeyCode::Numpad3),
                "Numpad4" => Ok(VirtualKeyCode::Numpad4),
                "Numpad5" => Ok(VirtualKeyCode::Numpad5),
                "Numpad6" => Ok(VirtualKeyCode::Numpad6),
                "Numpad7" => Ok(VirtualKeyCode::Numpad7),
                "Numpad8" => Ok(VirtualKeyCode::Numpad8),
                "Numpad9" => Ok(VirtualKeyCode::Numpad9),
                "NumpadAdd" => Ok(VirtualKeyCode::NumpadAdd),
                "NumpadDivide" => Ok(VirtualKeyCode::NumpadDivide),
                "NumpadDecimal" => Ok(VirtualKeyCode::NumpadDecimal),
                "NumpadComma" => Ok(VirtualKeyCode::NumpadComma),
                "NumpadEnter" => Ok(VirtualKeyCode::NumpadEnter),
                "NumpadEquals" => Ok(VirtualKeyCode::NumpadEquals),
                "NumpadMultiply" => Ok(VirtualKeyCode::NumpadMultiply),
                "NumpadSubtract" => Ok(VirtualKeyCode::NumpadSubtract),
                "AbntC1" => Ok(VirtualKeyCode::AbntC1),
                "AbntC2" => Ok(VirtualKeyCode::AbntC2),
                "Apostrophe" => Ok(VirtualKeyCode::Apostrophe),
                "Apps" => Ok(VirtualKeyCode::Apps),
                "Asterisk" => Ok(VirtualKeyCode::Asterisk),
                "At" => Ok(VirtualKeyCode::At),
                "Ax" => Ok(VirtualKeyCode::Ax),
                "Backslash" => Ok(VirtualKeyCode::Backslash),
                "Calculator" => Ok(VirtualKeyCode::Calculator),
                "Capital" => Ok(VirtualKeyCode::Capital),
                "Colon" => Ok(VirtualKeyCode::Colon),
                "Comma" => Ok(VirtualKeyCode::Comma),
                "Convert" => Ok(VirtualKeyCode::Convert),
                "Equals" => Ok(VirtualKeyCode::Equals),
                "Grave" => Ok(VirtualKeyCode::Grave),
                "Kana" => Ok(VirtualKeyCode::Kana),
                "Kanji" => Ok(VirtualKeyCode::Kanji),
                "LAlt" => Ok(VirtualKeyCode::LAlt),
                "LBracket" => Ok(VirtualKeyCode::LBracket),
                "LControl" => Ok(VirtualKeyCode::LControl),
                "LShift" => Ok(VirtualKeyCode::LShift),
                "LWin" => Ok(VirtualKeyCode::LWin),
                "Mail" => Ok(VirtualKeyCode::Mail),
                "MediaSelect" => Ok(VirtualKeyCode::MediaSelect),
                "MediaStop" => Ok(VirtualKeyCode::MediaStop),
                "Minus" => Ok(VirtualKeyCode::Minus),
                "Mute" => Ok(VirtualKeyCode::Mute),
                "MyComputer" => Ok(VirtualKeyCode::MyComputer),
                "NavigateForward" => Ok(VirtualKeyCode::NavigateForward),
                "NavigateBackward" => Ok(VirtualKeyCode::NavigateBackward),
                "NextTrack" => Ok(VirtualKeyCode::NextTrack),
                "NoConvert" => Ok(VirtualKeyCode::NoConvert),
                "OEM102" => Ok(VirtualKeyCode::OEM102),
                "Period" => Ok(VirtualKeyCode::Period),
                "PlayPause" => Ok(VirtualKeyCode::PlayPause),
                "Plus" => Ok(VirtualKeyCode::Plus),
                "Power" => Ok(VirtualKeyCode::Power),
                "PrevTrack" => Ok(VirtualKeyCode::PrevTrack),
                "RAlt" => Ok(VirtualKeyCode::RAlt),
                "RBracket" => Ok(VirtualKeyCode::RBracket),
                "RControl" => Ok(VirtualKeyCode::RControl),
                "RShift" => Ok(VirtualKeyCode::RShift),
                "RWin" => Ok(VirtualKeyCode::RWin),
                "Semicolon" => Ok(VirtualKeyCode::Semicolon),
                "Slash" => Ok(VirtualKeyCode::Slash),
                "Sleep" => Ok(VirtualKeyCode::Sleep),
                "Stop" => Ok(VirtualKeyCode::Stop),
                "Sysrq" => Ok(VirtualKeyCode::Sysrq),
                "Tab" => Ok(VirtualKeyCode::Tab),
                "Underline" => Ok(VirtualKeyCode::Underline),
                "Unlabeled" => Ok(VirtualKeyCode::Unlabeled),
                "VolumeDown" => Ok(VirtualKeyCode::VolumeDown),
                "VolumeUp" => Ok(VirtualKeyCode::VolumeUp),
                "Wake" => Ok(VirtualKeyCode::Wake),
                "WebBack" => Ok(VirtualKeyCode::WebBack),
                "WebFavorites" => Ok(VirtualKeyCode::WebFavorites),
                "WebForward" => Ok(VirtualKeyCode::WebForward),
                "WebHome" => Ok(VirtualKeyCode::WebHome),
                "WebRefresh" => Ok(VirtualKeyCode::WebRefresh),
                "WebSearch" => Ok(VirtualKeyCode::WebSearch),
                "WebStop" => Ok(VirtualKeyCode::WebStop),
                "Yen" => Ok(VirtualKeyCode::Yen),
                "Copy" => Ok(VirtualKeyCode::Copy),
                "Paste" => Ok(VirtualKeyCode::Paste),
                "Cut" => Ok(VirtualKeyCode::Cut),
                _ => Err(anyhow!("Invalid Keycode: {}.", s)),
            }
        }

        let mut keys = HashMap::with_capacity(15);
        keys.insert(str_to_virtkeycode(&self.key_0)?, emulator::Key::Key0);
        keys.insert(str_to_virtkeycode(&self.key_1)?, emulator::Key::Key1);
        keys.insert(str_to_virtkeycode(&self.key_2)?, emulator::Key::Key2);
        keys.insert(str_to_virtkeycode(&self.key_3)?, emulator::Key::Key3);
        keys.insert(str_to_virtkeycode(&self.key_4)?, emulator::Key::Key4);
        keys.insert(str_to_virtkeycode(&self.key_5)?, emulator::Key::Key5);
        keys.insert(str_to_virtkeycode(&self.key_6)?, emulator::Key::Key6);
        keys.insert(str_to_virtkeycode(&self.key_7)?, emulator::Key::Key7);
        keys.insert(str_to_virtkeycode(&self.key_8)?, emulator::Key::Key8);
        keys.insert(str_to_virtkeycode(&self.key_9)?, emulator::Key::Key9);
        keys.insert(str_to_virtkeycode(&self.key_a)?, emulator::Key::KeyA);
        keys.insert(str_to_virtkeycode(&self.key_b)?, emulator::Key::KeyB);
        keys.insert(str_to_virtkeycode(&self.key_c)?, emulator::Key::KeyC);
        keys.insert(str_to_virtkeycode(&self.key_d)?, emulator::Key::KeyD);
        keys.insert(str_to_virtkeycode(&self.key_e)?, emulator::Key::KeyE);
        keys.insert(str_to_virtkeycode(&self.key_f)?, emulator::Key::KeyF);

        Ok(keys)
    }
}

pub struct Config {
    pub pixel_size: u32,
    pub on_color: (u8, u8, u8),
    pub keys: HashMap<VirtualKeyCode, Key>,
}

impl Default for Config {
    fn default() -> Self {
        let keys = HashMap::from([
            (VirtualKeyCode::Key0, emulator::Key::Key0),
            (VirtualKeyCode::Key1, emulator::Key::Key1),
            (VirtualKeyCode::Key2, emulator::Key::Key2),
            (VirtualKeyCode::Key3, emulator::Key::Key3),
            (VirtualKeyCode::Key4, emulator::Key::Key4),
            (VirtualKeyCode::Key5, emulator::Key::Key5),
            (VirtualKeyCode::Key6, emulator::Key::Key6),
            (VirtualKeyCode::Key7, emulator::Key::Key7),
            (VirtualKeyCode::Key8, emulator::Key::Key8),
            (VirtualKeyCode::Key9, emulator::Key::Key9),
            (VirtualKeyCode::A, emulator::Key::KeyA),
            (VirtualKeyCode::B, emulator::Key::KeyB),
            (VirtualKeyCode::C, emulator::Key::KeyC),
            (VirtualKeyCode::D, emulator::Key::KeyD),
            (VirtualKeyCode::E, emulator::Key::KeyE),
            (VirtualKeyCode::F, emulator::Key::KeyF),
        ]);

        Self {
            pixel_size: 10,
            on_color: (0, 0, 255),
            keys,
        }
    }
}

pub fn load() -> anyhow::Result<Config> {
    fn use_default_config() -> anyhow::Result<Config> {
        println!("No config file found, using default configuration.");
        Ok(Config::default())
    }

    if let Some(app_dirs) = AppDirs::new(Some("chip8-emulator"), true) {
        let config_file_path = app_dirs.config_dir.join("config.toml");
        if config_file_path.exists() {
            let toml_str = fs::read_to_string(&config_file_path).context(format!(
                "Could not open file: {}.",
                config_file_path.as_path().display()
            ))?;
            let toml_comfig: TomlConfig =
                toml::from_str(&toml_str).context("Could not parse configuration file.")?;
            toml_comfig.to_config()
        } else {
            use_default_config()
        }
    } else {
        use_default_config()
    }
}
