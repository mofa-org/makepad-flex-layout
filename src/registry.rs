//! Panel Registry - central place to register panel definitions
//!
//! Provides a registry for panel definitions with content providers,
//! enabling semantic panel identification and content injection.

use std::collections::HashMap;

/// Defines a panel type that can be instantiated in the grid
#[derive(Clone, Debug)]
pub struct PanelDefinition {
    /// Unique semantic ID for this panel (e.g., "file_browser", "editor", "console")
    pub id: String,

    /// Display title for the panel header
    pub title: String,

    /// Whether the panel can be closed
    pub closable: bool,

    /// Whether the panel can be maximized (main grid only)
    pub maximizable: bool,

    /// Whether the panel can be fullscreened (footer grid only)
    pub fullscreenable: bool,
}

impl PanelDefinition {
    /// Create a new panel definition with default settings
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            closable: true,
            maximizable: true,
            fullscreenable: false,
        }
    }

    /// Set whether the panel can be closed
    pub fn with_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set whether the panel can be maximized
    pub fn with_maximizable(mut self, maximizable: bool) -> Self {
        self.maximizable = maximizable;
        self
    }

    /// Set whether the panel can be fullscreened
    pub fn with_fullscreenable(mut self, fullscreenable: bool) -> Self {
        self.fullscreenable = fullscreenable;
        self
    }

    /// Create a footer panel definition (fullscreenable, not maximizable)
    pub fn footer(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            closable: true,
            maximizable: false,
            fullscreenable: true,
        }
    }
}

/// Registry for panel definitions
#[derive(Default)]
pub struct PanelRegistry {
    definitions: HashMap<String, PanelDefinition>,
    /// Ordered list of panel IDs for consistent iteration
    panel_order: Vec<String>,
}

impl PanelRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a panel definition
    ///
    /// If a panel with the same ID already exists, it will be replaced.
    pub fn register(&mut self, def: PanelDefinition) {
        let id = def.id.clone();
        if !self.definitions.contains_key(&id) {
            self.panel_order.push(id.clone());
        }
        self.definitions.insert(id, def);
    }

    /// Register multiple panel definitions at once
    pub fn register_all(&mut self, defs: impl IntoIterator<Item = PanelDefinition>) {
        for def in defs {
            self.register(def);
        }
    }

    /// Get panel definition by ID
    pub fn get(&self, id: &str) -> Option<&PanelDefinition> {
        self.definitions.get(id)
    }

    /// Get all registered panel IDs in registration order
    pub fn panel_ids(&self) -> impl Iterator<Item = &String> {
        self.panel_order.iter()
    }

    /// Get all panel definitions in registration order
    pub fn panels(&self) -> impl Iterator<Item = &PanelDefinition> {
        self.panel_order.iter().filter_map(|id| self.definitions.get(id))
    }

    /// Get the number of registered panels
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Remove a panel definition
    pub fn remove(&mut self, id: &str) -> Option<PanelDefinition> {
        if let Some(def) = self.definitions.remove(id) {
            self.panel_order.retain(|i| i != id);
            Some(def)
        } else {
            None
        }
    }

    /// Clear all panel definitions
    pub fn clear(&mut self) {
        self.definitions.clear();
        self.panel_order.clear();
    }

    /// Create a default registry with numbered panels for testing
    pub fn with_default_panels(count: usize) -> Self {
        let mut registry = Self::new();
        for i in 0..count {
            registry.register(PanelDefinition::new(
                format!("panel_{}", i),
                format!("Panel {}", i + 1),
            ));
        }
        registry
    }

    /// Create a default footer registry with numbered panels for testing
    pub fn with_default_footer_panels(count: usize) -> Self {
        let mut registry = Self::new();
        for i in 0..count {
            registry.register(PanelDefinition::footer(
                format!("footer_panel_{}", i),
                format!("Footer {}", i + 1),
            ));
        }
        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_definition() {
        let def = PanelDefinition::new("editor", "Code Editor")
            .with_closable(false)
            .with_maximizable(true);

        assert_eq!(def.id, "editor");
        assert_eq!(def.title, "Code Editor");
        assert!(!def.closable);
        assert!(def.maximizable);
    }

    #[test]
    fn test_registry_operations() {
        let mut registry = PanelRegistry::new();

        registry.register(PanelDefinition::new("files", "Files"));
        registry.register(PanelDefinition::new("editor", "Editor"));
        registry.register(PanelDefinition::new("console", "Console"));

        assert_eq!(registry.len(), 3);
        assert!(registry.get("files").is_some());
        assert!(registry.get("unknown").is_none());

        // Check order
        let ids: Vec<&String> = registry.panel_ids().collect();
        assert_eq!(ids, vec!["files", "editor", "console"]);

        // Remove
        registry.remove("editor");
        assert_eq!(registry.len(), 2);
        assert!(registry.get("editor").is_none());
    }

    #[test]
    fn test_default_panels() {
        let registry = PanelRegistry::with_default_panels(9);
        assert_eq!(registry.len(), 9);
        assert!(registry.get("panel_0").is_some());
        assert!(registry.get("panel_8").is_some());
    }
}
