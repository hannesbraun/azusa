use simple_config_parser::{Config, ConfigError};

pub struct PomodoroConfig {
    pub work: u64,
    pub short_break: u64,
    pub long_break: u64,
    pub long_break_after: u32,
}

const DEFAULT_POMODORO: PomodoroConfig = PomodoroConfig {
    work: 25,
    short_break: 4,
    long_break: 20,
    long_break_after: 4,
};

pub fn app_config() -> PomodoroConfig {
    get_config().map_or(DEFAULT_POMODORO, read_config)
}

fn read_config(cfg: Config) -> PomodoroConfig {
    PomodoroConfig {
        work: cfg
            .get::<u64>("pomodoro_time")
            .unwrap_or(DEFAULT_POMODORO.work),
        short_break: cfg
            .get::<u64>("short_break_time")
            .unwrap_or(DEFAULT_POMODORO.short_break),
        long_break: cfg
            .get::<u64>("long_break_time")
            .unwrap_or(DEFAULT_POMODORO.long_break),
        long_break_after: cfg
            .get::<u32>("pomodoros_before_long_break")
            .unwrap_or(DEFAULT_POMODORO.long_break_after),
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
