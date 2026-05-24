//! Minimal internationalization (i18n) support for Warp.
//!
//! Provides a [`t!`] macro and [`translate`] function that return a translated
//! string for the current [`Language`] setting.  Only the settings UI is
//! translated in this initial implementation; extend the key lists below to
//! cover more of the application over time.

use settings::Setting;
use warpui::{AppContext, SingletonEntity};

use crate::settings::language::{Language, LanguageSettings};

/// Returns the current language setting from the app context.
pub fn current_language(app: &AppContext) -> Language {
    *LanguageSettings::as_ref(app).language.value()
}

/// Returns a translated string for the given key and language.
///
/// Falls back to the English string when a translation is missing.
pub fn translate(key: &str, language: Language) -> &'static str {
    match language {
        Language::EnglishUs => translate_en(key),
        Language::SimplifiedChinese => translate_zh_hans(key).unwrap_or_else(|| translate_en(key)),
    }
}

/// Look up an English string.  Always returns a non-empty string.
/// Returns the key itself (as a `'static` str) when no translation is found.
fn translate_en(key: &str) -> &'static str {
    match key {
        // Settings section labels
        "settings.section.appearance" => "Appearance",
        "settings.section.language" => "Language",

        // Language setting
        "settings.language.label" => "Display language",
        "settings.language.description" =>
            "Choose the language used throughout the Warp interface.",

        // Appearance page category labels
        "appearance.category.themes" => "Themes",
        "appearance.category.icon" => "Icon",
        "appearance.category.window" => "Window",
        "appearance.category.input" => "Input",
        "appearance.category.panes" => "Panes",
        "appearance.category.blocks" => "Blocks",
        "appearance.category.text" => "Text",
        "appearance.category.cursor" => "Cursor",
        "appearance.category.tabs" => "Tabs",

        // Unknown key: return a placeholder so missing translations are obvious.
        _ => "[missing translation]",
    }
}

/// Look up a Simplified-Chinese string.  Returns `None` when untranslated so
/// the caller can fall back to English.
fn translate_zh_hans(key: &str) -> Option<&'static str> {
    match key {
        // Settings section labels
        "settings.section.appearance" => Some("外观"),
        "settings.section.language" => Some("语言"),

        // Language setting
        "settings.language.label" => Some("显示语言"),
        "settings.language.description" =>
            Some("选择 Warp 界面使用的语言。"),

        // Appearance page category labels
        "appearance.category.themes" => Some("主题"),
        "appearance.category.icon" => Some("图标"),
        "appearance.category.window" => Some("窗口"),
        "appearance.category.input" => Some("输入"),
        "appearance.category.panes" => Some("面板"),
        "appearance.category.blocks" => Some("块"),
        "appearance.category.text" => Some("文本"),
        "appearance.category.cursor" => Some("光标"),
        "appearance.category.tabs" => Some("标签页"),

        _ => None,
    }
}

/// Convenience macro for translating a string key using an [`AppContext`].
///
/// # Example
///
/// ```ignore
/// let label = t!("settings.language.label", app);
/// ```
#[macro_export]
macro_rules! t {
    ($key:expr, $app:expr) => {
        $crate::i18n::translate($key, $crate::i18n::current_language($app))
    };
}
