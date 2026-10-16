//! The run's preset (Pint's `ConfigurationJsonRepository::preset()`: `--preset`, else pint.json's
//! `preset`, else "laravel"). Pint/laravel_blade formats embedded PHP with that preset's rules.
//! Set by the CLI after loading the config.
use std::sync::RwLock;

static PRESET: RwLock<Option<String>> = RwLock::new(None);

pub fn set(preset: &str) {
    *PRESET.write().unwrap() = Some(preset.to_string());
}

pub fn get() -> String {
    PRESET.read().unwrap().clone().unwrap_or_else(|| "laravel".into())
}
