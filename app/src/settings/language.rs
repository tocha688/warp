use serde::{Deserialize, Serialize};
use settings::macros::define_settings_group;
use settings::{RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};

/// The display language for the Warp UI.
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(description = "The display language for the Warp UI.", rename_all = "snake_case")]
pub enum Language {
    /// English (US)
    #[default]
    EnglishUs,
    /// Simplified Chinese (简体中文)
    SimplifiedChinese,
}

impl Language {
    /// Returns a human-readable label for the language, always in the native language.
    pub fn display_label(self) -> &'static str {
        match self {
            Language::EnglishUs => "English (US)",
            Language::SimplifiedChinese => "简体中文",
        }
    }
}

define_settings_group!(LanguageSettings, settings: [
    language: LanguageSetting {
        type: Language,
        default: Language::default(),
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        storage_key: "Language",
        toml_path: "appearance.language",
        description: "The display language for the Warp UI.",
    },
]);
