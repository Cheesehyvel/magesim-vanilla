use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub enum Race {
    #[default]
    Gnome,
    Human,
    Troll,
    Undead,
}

#[derive(Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum School {
    #[default]
    None,
    Arcane,
    Fire,
    Frost,
    Holy,
    Nature,
    Physical,
    Shadow,
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}