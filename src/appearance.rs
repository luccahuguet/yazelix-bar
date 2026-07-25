pub const APPEARANCE_MODE_DARK: &str = "dark";
pub const APPEARANCE_MODE_LIGHT: &str = "light";
pub const APPEARANCE_MODE_AUTO: &str = "auto";

const RUNTIME_STYLE_SESSION_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_STYLE_SESSION__";
const RUNTIME_STYLE_BRAND_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_STYLE_BRAND__";
const RUNTIME_STYLE_TAB_TRUNCATE_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_STYLE_TAB_TRUNCATE__";
const RUNTIME_STYLE_DATETIME_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_STYLE_DATETIME__";
const RUNTIME_STYLE_WORKSPACE_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_STYLE_WORKSPACE__";
const RUNTIME_STYLE_USAGE_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_STYLE_USAGE__";
const RUNTIME_STYLE_SYSTEM_USAGE_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_STYLE_SYSTEM_USAGE__";
const RUNTIME_MODE_NORMAL_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_MODE_NORMAL__";
const RUNTIME_MODE_TMUX_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_MODE_TMUX__";
const RUNTIME_MODE_SESSION_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_MODE_SESSION__";
const RUNTIME_MODE_SCROLL_PLACEHOLDER: &str = "__YAZELIX_RUNTIME_MODE_SCROLL__";

pub fn default_appearance_mode() -> String {
    APPEARANCE_MODE_DARK.to_string()
}

pub fn normalize_appearance_arg(raw: &str) -> Option<&'static str> {
    let normalized = raw.trim();
    if normalized.eq_ignore_ascii_case(APPEARANCE_MODE_DARK) {
        Some(APPEARANCE_MODE_DARK)
    } else if normalized.eq_ignore_ascii_case(APPEARANCE_MODE_LIGHT) {
        Some(APPEARANCE_MODE_LIGHT)
    } else if normalized.eq_ignore_ascii_case(APPEARANCE_MODE_AUTO) {
        Some(APPEARANCE_MODE_AUTO)
    } else {
        None
    }
}

pub(crate) fn runtime_bar_appearance(raw: &str) -> &'static str {
    if raw.trim().eq_ignore_ascii_case(APPEARANCE_MODE_LIGHT) {
        APPEARANCE_MODE_LIGHT
    } else {
        APPEARANCE_MODE_DARK
    }
}

pub(crate) fn bar_style_for_appearance(raw: &str) -> &'static BarStyle {
    match runtime_bar_appearance(raw) {
        APPEARANCE_MODE_LIGHT => &LIGHT_BAR_STYLE,
        _ => &DARK_BAR_STYLE,
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BarStyle {
    pub(crate) session: &'static str,
    pub(crate) widget: &'static str,
    pub(crate) custom_text: &'static str,
    pub(crate) separator: &'static str,
    pub(crate) brand: &'static str,
    pub(crate) mode_normal: &'static str,
    pub(crate) mode_tmux: &'static str,
    pub(crate) mode_session: &'static str,
    pub(crate) mode_scroll: &'static str,
    pub(crate) tab_normal: &'static str,
    pub(crate) tab_bell: &'static str,
    pub(crate) tab_flashing_bell: &'static str,
    pub(crate) tab_active: &'static str,
    pub(crate) tab_truncate: &'static str,
    pub(crate) datetime: &'static str,
    pub(crate) workspace: &'static str,
    pub(crate) usage: &'static str,
    pub(crate) system_usage: &'static str,
}

impl BarStyle {
    pub(crate) fn template_replacements(self) -> [(&'static str, &'static str); 11] {
        [
            (RUNTIME_STYLE_SESSION_PLACEHOLDER, self.session),
            (RUNTIME_STYLE_BRAND_PLACEHOLDER, self.brand),
            (RUNTIME_STYLE_TAB_TRUNCATE_PLACEHOLDER, self.tab_truncate),
            (RUNTIME_STYLE_DATETIME_PLACEHOLDER, self.datetime),
            (RUNTIME_STYLE_WORKSPACE_PLACEHOLDER, self.workspace),
            (RUNTIME_STYLE_USAGE_PLACEHOLDER, self.usage),
            (RUNTIME_STYLE_SYSTEM_USAGE_PLACEHOLDER, self.system_usage),
            (RUNTIME_MODE_NORMAL_PLACEHOLDER, self.mode_normal),
            (RUNTIME_MODE_TMUX_PLACEHOLDER, self.mode_tmux),
            (RUNTIME_MODE_SESSION_PLACEHOLDER, self.mode_session),
            (RUNTIME_MODE_SCROLL_PLACEHOLDER, self.mode_scroll),
        ]
    }
}

pub(crate) const DARK_BAR_STYLE: BarStyle = BarStyle {
    session: "#[fg=#ff0088,bold]",
    widget: "#[fg=#00ff88,bold]",
    custom_text: "#[fg=#ffff00,bold]",
    separator: "#[fg=#6c7086,bold]",
    brand: "#[fg=#00ccff,bold]",
    mode_normal: "#[bg=#00ff88,fg=#000000,bold]",
    mode_tmux: "#[bg=#ffff00,fg=#000000,bold]",
    mode_session: "#[bg=#ff6600,fg=#000000,bold]",
    mode_scroll: "#[bg=#ff0088,fg=#ffffff,bold]",
    tab_normal: "#[fg=#ffff00]",
    tab_bell: "#[fg=#ff0088,bold]",
    tab_flashing_bell: "#[bg=#ff0088,fg=#ffffff,bold]",
    tab_active: "#[bg=#ff6600,fg=#000000,bold]",
    tab_truncate: "#[fg=#ff6600,bold]",
    datetime: "#[fg=#bb88ff,bold]",
    workspace: "#[fg=#00ff88,bold]",
    usage: "#[fg=#bb88ff,bold]",
    system_usage: "#[fg=#ff6600]",
};

pub(crate) const LIGHT_BAR_STYLE: BarStyle = BarStyle {
    session: "#[fg=#7c3f97,bold]",
    widget: "#[fg=#2f7d32,bold]",
    custom_text: "#[fg=#9a5a00,bold]",
    separator: "#[fg=#8c8fa1,bold]",
    brand: "#[fg=#1e66f5,bold]",
    mode_normal: "#[bg=#cfe8d4,fg=#1f5f32,bold]",
    mode_tmux: "#[bg=#f2e4bc,fg=#6f4a00,bold]",
    mode_session: "#[bg=#f0d7c2,fg=#8a3f00,bold]",
    mode_scroll: "#[bg=#ead4ec,fg=#6d3f73,bold]",
    tab_normal: "#[fg=#5c5f77]",
    tab_bell: "#[fg=#b4637a,bold]",
    tab_flashing_bell: "#[bg=#b4637a,fg=#fffaf3,bold]",
    tab_active: "#[bg=#ccd0da,fg=#303446,bold]",
    tab_truncate: "#[fg=#9a5a00,bold]",
    datetime: "#[fg=#7850a8,bold]",
    workspace: "#[fg=#2f7d32,bold]",
    usage: "#[fg=#7850a8,bold]",
    system_usage: "#[fg=#a24f00]",
};
