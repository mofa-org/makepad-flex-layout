//! Layout state for the panel grid
//!
//! Provides serializable state for persisting grid layout across sessions.

use serde::{Deserialize, Serialize};

/// Layout mode for the panel grid
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum LayoutMode {
    /// Automatic grid based on window count
    #[default]
    AutoGrid,
    /// Horizontal stack (all panels in one row)
    HStack,
    /// Vertical stack (all panels in one column)
    VStack,
    /// Tabbed view (one panel visible at a time)
    Tabbed,
}

impl LayoutMode {
    /// Get display name for this layout mode
    pub fn name(&self) -> &'static str {
        match self {
            LayoutMode::AutoGrid => "Auto Grid",
            LayoutMode::HStack => "Horizontal",
            LayoutMode::VStack => "Vertical",
            LayoutMode::Tabbed => "Tabbed",
        }
    }
}

/// Serializable layout state for persistence
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutState {
    /// Panel assignments per row (by panel ID as u64)
    pub row_assignments: Vec<Vec<u64>>,

    /// Which panels are visible (panel IDs as u64)
    pub visible_panels: Vec<u64>,

    /// Currently maximized panel (if any)
    pub maximized_panel: Option<u64>,

    /// Current layout mode
    pub layout_mode: LayoutMode,

    /// Selected tab index (for tabbed mode)
    pub selected_tab: usize,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            row_assignments: vec![
                vec![0, 1, 2],
                vec![3, 4, 5],
                vec![6, 7, 8],
            ],
            visible_panels: (0..9).collect(),
            maximized_panel: None,
            layout_mode: LayoutMode::AutoGrid,
            selected_tab: 0,
        }
    }
}

impl LayoutState {
    /// Create a new layout state with the given number of panels
    pub fn with_panel_count(count: usize) -> Self {
        let mut state = Self::default();
        state.visible_panels = (0..count as u64).collect();

        // Distribute panels across rows (roughly equal)
        let panels_per_row = (count + 2) / 3; // Ceiling division by 3
        state.row_assignments = vec![Vec::new(), Vec::new(), Vec::new()];

        for (i, panel_id) in (0..count as u64).enumerate() {
            let row = i / panels_per_row;
            if row < 3 {
                state.row_assignments[row].push(panel_id);
            }
        }

        state
    }

    /// Get the total number of visible panels
    pub fn visible_count(&self) -> usize {
        self.visible_panels.len()
    }

    /// Check if a panel is visible
    pub fn is_visible(&self, panel_id: u64) -> bool {
        self.visible_panels.contains(&panel_id)
    }

    /// Find which row contains a panel
    pub fn find_panel_row(&self, panel_id: u64) -> Option<(usize, usize)> {
        for (row_idx, row) in self.row_assignments.iter().enumerate() {
            if let Some(col_idx) = row.iter().position(|&id| id == panel_id) {
                return Some((row_idx, col_idx));
            }
        }
        None
    }

    /// Get visible panels in a specific row
    pub fn visible_in_row(&self, row: usize) -> Vec<u64> {
        if row >= self.row_assignments.len() {
            return vec![];
        }
        self.row_assignments[row]
            .iter()
            .filter(|id| self.visible_panels.contains(id))
            .copied()
            .collect()
    }

    /// Close a panel (mark as not visible)
    pub fn close_panel(&mut self, panel_id: u64) {
        self.visible_panels.retain(|&id| id != panel_id);

        // Remove from row assignments
        for row in &mut self.row_assignments {
            row.retain(|&id| id != panel_id);
        }

        // If closing the maximized panel, exit maximize mode
        if self.maximized_panel == Some(panel_id) {
            self.maximized_panel = None;
        }
    }

    /// Move a panel from one position to another
    pub fn move_panel(&mut self, panel_id: u64, target_row: usize, target_col: usize) {
        // Find current position
        let Some((src_row, src_col)) = self.find_panel_row(panel_id) else {
            return;
        };

        // Don't do anything if dropping at the same position
        if src_row == target_row && src_col == target_col {
            return;
        }

        // Ensure target row exists
        while self.row_assignments.len() <= target_row {
            self.row_assignments.push(Vec::new());
        }

        // Remove from source row
        self.row_assignments[src_row].remove(src_col);

        // Calculate insert position in target row
        let visible_in_target = self.visible_in_row(target_row).len();
        let mut insert_col = target_col.min(visible_in_target);

        // If same row but target was after source, adjust for removal
        if src_row == target_row && target_col > src_col {
            insert_col = insert_col.saturating_sub(1);
        }

        // Insert at target position
        self.row_assignments[target_row].insert(insert_col, panel_id);
    }
}

/// Splitter positions for persistence
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SplitterPositions {
    pub left_sidebar: f64,
    pub right_sidebar: f64,
    pub footer: f64,
}

impl Default for SplitterPositions {
    fn default() -> Self {
        Self {
            left_sidebar: 280.0,
            right_sidebar: 300.0,
            footer: 100.0,
        }
    }
}
