use std::env;
use std::sync::RwLock;

static WHEN: RwLock<When> = RwLock::new(When::Auto);

#[derive(Clone, Copy, Debug)]
pub enum When {
    Auto,
    Always,
    Never,
}

impl When {
    pub const fn is_enabled(self) -> bool {
        matches!(self, Self::Auto | Self::Always)
    }
}

pub fn init() {
    let when = if is_env_no_color_set() {
        When::Never
    } else {
        When::Auto
    };
    set_when(when);
}

pub fn when() -> When {
    *WHEN.read().unwrap()
}

pub fn set_when(when: When) {
    *WHEN.write().unwrap() = when;
}

pub fn is_enabled() -> bool {
    when().is_enabled()
}

pub fn is_env_no_color_set() -> bool {
    env::var("NO_COLOR").is_ok()
}
