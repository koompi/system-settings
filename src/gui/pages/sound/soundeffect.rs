#![allow(unused_variables)]
#![allow(dead_code)]
use crate::gui::styles::{buttons::ButtonStyle, containers::ContainerStyle};
use iced::{button, Align, Button, Column, Container, Element, Length, Row, Space, Text};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Toggler;
use std::fmt;
use std::path::PathBuf;
#[derive(Default, Debug, Clone)]
pub struct SndEffect {
    is_sound_effect: bool,
    sample_effects: Vec<(button::State, button::State, String)>,
    effect_tick: usize,
    enable_sound_effect: bool,
    sound_effecs: SettingsSoundEffect,
}

pub type SoundEffectErorr = Result<bool, std::io::Error>;

pub trait SoundEffect {
    fn play(&mut self, file: PathBuf) -> SoundEffectErorr;
    fn stop(&mut self, file: PathBuf) -> SoundEffectErorr;
    fn pause(&mut self, file: PathBuf) -> SoundEffectErorr;
    fn speed(&self) -> u32;
    fn volume(&self) -> u32;
}
#[derive(Debug, Clone)]
pub enum SndEffectMsg {
    SoundEffect(bool),
    TestSoundEffect(usize),
    EnableEffect(usize),
}
impl SndEffect {
    pub fn new() -> Self {
        let str_con = |f: &str| -> String { f.to_string() };
        let mut vec_sounds: Vec<String> = vec![
            str_con("Bootup"),
            str_con("Log out"),
            str_con("Shutdown"),
            str_con("Volume +/-"),
            str_con("Wake Up"),
            // str_con("Notifications"),
            // str_con("Low battery"),
            // str_con("Send icon in Launcher to Desktop"),
            // str_con("Empty Trash"),
            // str_con("Plug in"),
            // str_con("Plug out"),
            // str_con("Removeable device connected"),
            // str_con("Removable device removed"),
            // str_con("Error"),
        ];
        let mut vec_tuple: Vec<(button::State, button::State, String)> = Vec::new();
        vec_sounds.iter_mut().for_each(|name| vec_tuple.push((button::State::new(), button::State::new(), name.clone())));
        let mut sound_effect_hash: Vec<(SoundEffectType, PathBuf)> = Vec::new();

        match playback::read_directory(if cfg!(debug_assertions) {
            print!("run debug");
            std::path::PathBuf::new().join(&format!("{}/assets/sounds", env!("CARGO_MANIFEST_DIR")))
        } else {
            std::path::PathBuf::new().join(standart_path::sys_data_dir().unwrap().join("syssettings/sounds"))
        }) {
            Ok(mut path) =>
            {
                #[allow(const_item_mutation)]
                for (i, j) in SoundEffectType::ALL[..].iter_mut().zip(path.iter_mut()) {
                    sound_effect_hash.push((*i, j.to_path_buf()));
                }
            }
            Err(e) => println!("Error: {}", e),
        }
        Self {
            sound_effecs: SettingsSoundEffect {
                hash_sounds: sound_effect_hash,
                ..Default::default()
            },
            enable_sound_effect: true,
            sample_effects: vec_tuple,
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: SndEffectMsg) {
        match msg {
            SndEffectMsg::TestSoundEffect(idx) => {
                let _key = SoundEffectType::ALL[idx];
                let value = &self.sound_effecs.hash_sounds[idx].1;
                match playback::run(&value) {
                    Ok(()) => println!("sucesss"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            SndEffectMsg::SoundEffect(is_effect) => self.is_sound_effect = is_effect,
            SndEffectMsg::EnableEffect(idx) => self.effect_tick = idx,
        }
    }
    pub fn view(&mut self) -> Element<SndEffectMsg> {
        let effect_enable = self.enable_sound_effect;
        let current_tick = self.effect_tick;
        let sound_effects = Column::new()
            .spacing(10)
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(Text::new("SoundPage Effects"))
                        .push(Space::with_width(Length::Fill))
                        .push(Toggler::new(self.is_sound_effect, String::from(""), SndEffectMsg::SoundEffect)),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(if self.is_sound_effect {
                self.sample_effects.iter_mut().enumerate().fold(Column::new().spacing(10).align_items(Align::Center), |col_sound, (idx, (enable_state, state, name))| {
                    col_sound.push(
                        Row::new()
                            .align_items(Align::Center)
                            .spacing(10)
                            .width(Length::Fill)
                            .push(
                                Button::new(enable_state, Row::new().push(Text::new(name.as_str())))
                                    .width(Length::Fill)
                                    .style(ButtonStyle::Transparent)
                                    .on_press(SndEffectMsg::TestSoundEffect(idx)),
                            )
                            // .push(Space::with_width(Length::Fill))
                            .push(
                                Button::new(state, Icon::new(if effect_enable && current_tick == idx { '\u{f058}' } else { '\u{f111}' }))
                                    .padding(4)
                                    .style(ButtonStyle::Transparent)
                                    .on_press(SndEffectMsg::EnableEffect(idx)),
                            ),
                    )
                })
            } else {
                Column::new()
            });
        sound_effects.into()
    }
}

#[derive(Debug, Default, Clone)]
pub struct SettingsSoundEffect {
    file: std::path::PathBuf,
    hash_sounds: Vec<(SoundEffectType, PathBuf)>,
    effect_type: SoundEffectType,
    volume: u32,
    speed: u32,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SoundEffectType {
    Bootup,
    ShutDown,
    Logout,
    Wakeup,
    VolumnUpDown,
    // Notification,
    // LowBattery,
    // SendIconLauncher,
    // EmptyTrash,
    // Plugin,
    // Plugout,
    // RemoveDevConnected,
    // RemovableDevRemoved,
    // ErrorSound,
}
impl SoundEffectType {
    const ALL: [SoundEffectType; 5] = [
        SoundEffectType::Bootup,
        SoundEffectType::ShutDown,
        SoundEffectType::Logout,
        SoundEffectType::Wakeup,
        SoundEffectType::VolumnUpDown,
        // SoundEffectType::Notification,
        // SoundEffectType::LowBattery,
        // SoundEffectType::SendIconLauncher,
        // SoundEffectType::EmptyTrash,
        // SoundEffectType::Plugin,
        // SoundEffectType::Plugout,
        // SoundEffectType::RemoveDevConnected,
        // SoundEffectType::RemovableDevRemoved,
        // SoundEffectType::RemovableDevRemoved,
    ];
}
impl fmt::Display for SoundEffectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SoundEffectType::Bootup => "Bootup",
                SoundEffectType::ShutDown => "Shutdown",
                SoundEffectType::Logout => "Log out",
                SoundEffectType::Wakeup => "Wake Up",
                SoundEffectType::VolumnUpDown => "Volume +/-",
                // SoundEffectType::Notification => "Notifications",
                // SoundEffectType::LowBattery => "Low battery",
                // SoundEffectType::SendIconLauncher => "Send icon in Launcher to Desktop",
                // SoundEffectType::EmptyTrash => "Empty Trash",
                // SoundEffectType::Plugin => "Plug in",
                // SoundEffectType::Plugout => "Plug out",
                // SoundEffectType::RemoveDevConnected => "Removable device connected",
                // SoundEffectType::RemovableDevRemoved => "Removable device removed",
                // SoundEffectType::ErrorSound => "Error",
            }
        )
    }
}
impl Default for SoundEffectType {
    fn default() -> Self {
        SoundEffectType::Bootup
    }
}

impl SettingsSoundEffect {
    pub fn new() -> Self {
        Self {
            file: dirs::config_dir().unwrap(),
            ..Default::default()
        }
    }
}
impl SoundEffect for SettingsSoundEffect {
    fn play(&mut self, file: PathBuf) -> SoundEffectErorr {
        unimplemented!("Function is unimplemented");
    }
    fn pause(&mut self, file: PathBuf) -> SoundEffectErorr {
        unimplemented!("Function is unimplemented");
    }
    fn stop(&mut self, file: PathBuf) -> SoundEffectErorr {
        unimplemented!("Function is unimplemented");
    }
    fn speed(&self) -> u32 {
        unimplemented!("Function is unimplemented");
    }
    fn volume(&self) -> u32 {
        unimplemented!("Function is unimplemented");
    }
}

mod playback {

    use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
    use std::fs::read_dir;
    use std::path::PathBuf;
    use std::time::Duration;
    // NOTE: You probably want to investigate the
    // mixer feature for real use cases.
    struct Sound {
        data: Vec<u8>,
        volume: f32,
        pos: usize,
    }
    pub fn read_directory(in_path: std::path::PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut list_sounds: Vec<PathBuf> = Vec::new();
        if in_path.exists() {
            for path in read_dir(in_path)? {
                let dir = path?;
                list_sounds.push(dir.path());
            }
        } else {
            let paths = read_dir(in_path)?;
        }
        Ok(list_sounds)
    }
    pub fn make_dir(in_path: &std::path::PathBuf, name: &str) -> Result<bool, std::io::Error> {
        std::fs::create_dir(in_path.join(name))?;
        Ok(true)
    }
    impl AudioCallback for Sound {
        type Channel = u8;

        fn callback(&mut self, out: &mut [u8]) {
            for dst in out.iter_mut() {
                // With channel type u8 the "silence" value is 128 (middle of the 0-2^8 range) so we need
                // to both fill in the silence and scale the wav data accordingly. Filling the silence
                // once the wav is finished is trivial, applying the volume is more tricky. We need to:
                // * Change the range of the values from [0, 255] to [-128, 127] so we can multiply
                // * Apply the volume by multiplying, this gives us range [-128*volume, 127*volume]
                // * Move the resulting range to a range centered around the value 128, the final range
                //   is [128 - 128*volume, 128 + 127*volume] – scaled and correctly positioned
                //
                // Using value 0 instead of 128 would result in clicking. Scaling by simply multiplying
                // would not give correct results.
                let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
                let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
                let scaled = (scaled_signed_float + 128.0) as u8;
                *dst = scaled;
                self.pos += 1;
            }
        }
    }
    pub fn run(path: &std::path::PathBuf) -> Result<(), String> {
        let sdl_context = sdl2::init().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1), // mono
            samples: None,     // default
        };
        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                let wav = AudioSpecWAV::load_wav(path).expect("Could not load test WAV file");
                let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
                let data = cvt.convert(wav.buffer().to_vec());
                // initialize the audio callback
                Sound { data: data, volume: 0.50, pos: 0 }
            })
            .unwrap();
        // Start playback
        device.resume();
        // std::thread::spawn(|| {
        // Play for a second
        std::thread::sleep(Duration::from_millis(1_000));
        // });

        // Device is automatically closed when dropped

        Ok(())
    }
}

mod standart_path {
    use std::path::PathBuf;
    pub fn sys_data_dir() -> Option<PathBuf> {
        Some(PathBuf::new().join("/usr/share/"))
    }
}
