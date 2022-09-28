use fltk::enums::Color;
use simple_config_parser::{Config, ConfigError};
use crate::mainview::ColorTheme;

pub struct PomodoroConfig {
    pub work: u64,
    pub short_break: u64,
    pub long_break: u64,
    pub long_break_after: u32,
    pub audio_file: Option<String>,
    pub color_theme: ColorTheme
}
pub const DEFAULT_AUDIO: &str = "alert.mp3";

const DEFAULT_THEME: ColorTheme = ColorTheme {
    break_color: Color::from_hex(0x3f7cac),
    idle_color: Color::from_hex(0x993955),
    work_color: Color::from_hex(0x69995d),
};

const DEFAULT_CFG: PomodoroConfig = PomodoroConfig {
    work: 25,
    short_break: 4,
    long_break: 20,
    long_break_after: 4,
    audio_file: None,
    color_theme: DEFAULT_THEME
};

pub fn app_config() -> PomodoroConfig {
    get_config().map_or(DEFAULT_CFG, read_config)
}

fn read_config(cfg: Config) -> PomodoroConfig {
    PomodoroConfig {
        work: cfg.get::<u64>("pomodoro_time").unwrap_or(DEFAULT_CFG.work),
        short_break: cfg
            .get::<u64>("short_break_time")
            .unwrap_or(DEFAULT_CFG.short_break),
        long_break: cfg
            .get::<u64>("long_break_time")
            .unwrap_or(DEFAULT_CFG.long_break),
        long_break_after: cfg
            .get::<u32>("pomodoros_before_long_break")
            .unwrap_or(DEFAULT_CFG.long_break_after),
        audio_file: cfg.get_str("alert").ok(),
        color_theme: ColorTheme {
            break_color: cfg.get_str("break_color")
                .map(|s| u32::from_str_radix(&s, 16).unwrap())
                .map_or(DEFAULT_THEME.break_color, |i| Color::from_hex(i)),
            idle_color: cfg.get_str("idle_color")
                .map(|s| u32::from_str_radix(&s, 16).unwrap()).
                map_or(DEFAULT_THEME.idle_color, |i| Color::from_hex(i)),
            work_color: cfg.get_str("work_color")
                .map(|s| u32::from_str_radix(&s, 16).unwrap())
                .map_or(DEFAULT_THEME.work_color, |i| Color::from_hex(i)),
        }
    }
}

fn get_config() -> Result<Config, ConfigError> {
    let paths = [".azusa", "~/.azusa"];

    let mut cfg = Err(ConfigError::NoFileDefined);
    for path in paths {
        cfg = Config::new().file(path);
        if cfg.is_ok() {
            return cfg;
        }
    }

    cfg
}
