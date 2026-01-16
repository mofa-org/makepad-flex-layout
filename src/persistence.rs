//! Persistence for shell preferences and layout state
//!
//! Provides save/load functionality for user preferences.

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::grid::{LayoutState, SplitterPositions};

/// Shell preferences for persistence
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ShellPreferences {
    /// Dark mode preference
    pub dark_mode: bool,

    /// Saved layout state
    pub layout: Option<LayoutState>,

    /// Saved splitter positions
    pub splitter_positions: Option<SplitterPositions>,
}

impl ShellPreferences {
    /// Get the preferences file path for an app
    pub fn get_path(app_id: &str) -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(app_id)
            .join("shell_preferences.json")
    }

    /// Load preferences from disk
    ///
    /// Returns default preferences if file doesn't exist or can't be parsed.
    pub fn load(app_id: &str) -> Self {
        let path = Self::get_path(app_id);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(prefs) = serde_json::from_str(&content) {
                    return prefs;
                }
            }
        }
        Self::default()
    }

    /// Save preferences to disk
    pub fn save(&self, app_id: &str) -> Result<(), std::io::Error> {
        let path = Self::get_path(app_id);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, content)
    }

    /// Set dark mode and save
    pub fn set_dark_mode(&mut self, app_id: &str, dark_mode: bool) -> Result<(), std::io::Error> {
        self.dark_mode = dark_mode;
        self.save(app_id)
    }

    /// Set layout state and save
    pub fn set_layout(
        &mut self,
        app_id: &str,
        layout: LayoutState,
    ) -> Result<(), std::io::Error> {
        self.layout = Some(layout);
        self.save(app_id)
    }

    /// Set splitter positions and save
    pub fn set_splitter_positions(
        &mut self,
        app_id: &str,
        positions: SplitterPositions,
    ) -> Result<(), std::io::Error> {
        self.splitter_positions = Some(positions);
        self.save(app_id)
    }
}

/// Convenience function to save layout state
pub fn save_layout(app_id: &str, state: &LayoutState) -> Result<(), std::io::Error> {
    let mut prefs = ShellPreferences::load(app_id);
    prefs.layout = Some(state.clone());
    prefs.save(app_id)
}

/// Convenience function to load layout state
pub fn load_layout(app_id: &str) -> Option<LayoutState> {
    ShellPreferences::load(app_id).layout
}

/// Convenience function to save dark mode preference
pub fn save_dark_mode(app_id: &str, dark_mode: bool) -> Result<(), std::io::Error> {
    let mut prefs = ShellPreferences::load(app_id);
    prefs.dark_mode = dark_mode;
    prefs.save(app_id)
}

/// Convenience function to load dark mode preference
pub fn load_dark_mode(app_id: &str) -> bool {
    ShellPreferences::load(app_id).dark_mode
}
