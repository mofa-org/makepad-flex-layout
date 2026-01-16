//! Shell configuration

/// Configuration for the app shell
#[derive(Clone, Debug)]
pub struct ShellConfig {
    /// Window title
    pub title: String,

    /// Initial window size (width, height)
    pub window_size: (f64, f64),

    /// Show/hide header
    pub show_header: bool,

    /// Show/hide footer
    pub show_footer: bool,

    /// Show/hide left sidebar
    pub show_left_sidebar: bool,

    /// Show/hide right sidebar
    pub show_right_sidebar: bool,

    /// Initial left sidebar width
    pub left_sidebar_width: f64,

    /// Initial right sidebar width
    pub right_sidebar_width: f64,

    /// Initial footer height
    pub footer_height: f64,

    /// Maximum number of rows in panel grid
    pub max_rows: usize,

    /// Maximum slots per row in panel grid
    pub max_slots_per_row: usize,

    /// Enable panel close button
    pub enable_panel_close: bool,

    /// Enable panel maximize button
    pub enable_panel_maximize: bool,

    /// Enable panel drag-and-drop
    pub enable_panel_drag: bool,

    /// Enable layout persistence
    pub enable_persistence: bool,

    /// Start in dark mode
    pub dark_mode: bool,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            title: "App Shell".to_string(),
            window_size: (1400.0, 900.0),
            show_header: true,
            show_footer: true,
            show_left_sidebar: true,
            show_right_sidebar: true,
            left_sidebar_width: 280.0,
            right_sidebar_width: 300.0,
            footer_height: 100.0,
            max_rows: 3,
            max_slots_per_row: 9,
            enable_panel_close: true,
            enable_panel_maximize: true,
            enable_panel_drag: true,
            enable_persistence: false,
            dark_mode: false,
        }
    }
}

impl ShellConfig {
    /// Create a new builder for ShellConfig
    pub fn builder() -> ShellConfigBuilder {
        ShellConfigBuilder::default()
    }
}

/// Builder for ShellConfig
#[derive(Default)]
pub struct ShellConfigBuilder {
    config: ShellConfig,
}

impl ShellConfigBuilder {
    /// Set the window title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    /// Set the window size
    pub fn window_size(mut self, width: f64, height: f64) -> Self {
        self.config.window_size = (width, height);
        self
    }

    /// Hide the header
    pub fn hide_header(mut self) -> Self {
        self.config.show_header = false;
        self
    }

    /// Hide the footer
    pub fn hide_footer(mut self) -> Self {
        self.config.show_footer = false;
        self
    }

    /// Hide the left sidebar
    pub fn hide_left_sidebar(mut self) -> Self {
        self.config.show_left_sidebar = false;
        self
    }

    /// Hide the right sidebar
    pub fn hide_right_sidebar(mut self) -> Self {
        self.config.show_right_sidebar = false;
        self
    }

    /// Set the left sidebar width
    pub fn left_sidebar_width(mut self, width: f64) -> Self {
        self.config.left_sidebar_width = width;
        self
    }

    /// Set the right sidebar width
    pub fn right_sidebar_width(mut self, width: f64) -> Self {
        self.config.right_sidebar_width = width;
        self
    }

    /// Set the footer height
    pub fn footer_height(mut self, height: f64) -> Self {
        self.config.footer_height = height;
        self
    }

    /// Enable dark mode by default
    pub fn dark_mode(mut self) -> Self {
        self.config.dark_mode = true;
        self
    }

    /// Enable layout persistence
    pub fn enable_persistence(mut self) -> Self {
        self.config.enable_persistence = true;
        self
    }

    /// Disable panel close buttons
    pub fn disable_panel_close(mut self) -> Self {
        self.config.enable_panel_close = false;
        self
    }

    /// Disable panel maximize buttons
    pub fn disable_panel_maximize(mut self) -> Self {
        self.config.enable_panel_maximize = false;
        self
    }

    /// Disable panel drag-and-drop
    pub fn disable_panel_drag(mut self) -> Self {
        self.config.enable_panel_drag = false;
        self
    }

    /// Build the ShellConfig
    pub fn build(self) -> ShellConfig {
        self.config
    }
}
