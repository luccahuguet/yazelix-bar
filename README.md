# Yazelix Zellij Bar

`yazelix-zellij-bar` is a standalone Zellij bar plugin package for users who want the Yazelix top-bar style without adopting the full Yazelix workspace

## Install Shape

The flake package is:

```bash
nix build github:luccahuguet/yazelix-zellij-bar#yazelix_zellij_bar
nix profile install github:luccahuguet/yazelix-zellij-bar#yazelix_zellij_bar
```

The package installs:

- `bin/yazelix_zellij_bar_widget`
- `share/yazelix_zellij_bar/zjstatus.wasm`
- `share/yazelix_zellij_bar/yazelix_zellij_bar.kdl`
- `share/yazelix_zellij_bar/yazelix_zellij_bar.template.kdl`
- `share/yazelix_zellij_bar/yazelix_runtime_bar.template.kdl`
- `share/yazelix_zellij_bar/examples/custom_command_widgets.kdl`
- `share/yazelix_zellij_bar/examples/standalone_zellij_layout.kdl`
- `share/yazelix_zellij_bar/examples/yazelix_runtime_widgets.kdl`
- `share/doc/yazelix_zellij_bar/README.md`

Use `yazelix_zellij_bar.kdl` as a Zellij layout plugin block. The template keeps `__YAZELIX_ZELLIJ_BAR_ZJSTATUS_WASM__` for users who want to substitute a different pinned `zjstatus.wasm`. The runtime template is for full Yazelix and embedders; vanilla Zellij users can ignore it. The example snippets are small blocks to copy into the plugin body rather than alternate full presets

## Minimal Zellij Layout Snippet

```kdl
layout {
    pane size=1 borderless=true {
        // Paste the contents of share/yazelix_zellij_bar/yazelix_zellij_bar.kdl here
    }
    pane
}
```

The packaged `yazelix_zellij_bar.kdl` already points at the package's `zjstatus.wasm` with a `file:` URL, and that wasm is installed from the repo's pinned `zjstatus` flake input

## Generic Boundary

The standalone default is intentionally generic:

- mode
- tabs
- session
- datetime
- Yazelix-branded colors and tab overflow behavior

It does not require:

- `yzx`
- `yzx_control`
- Yazelix runtime paths
- the Yazelix pane orchestrator
- Nushell
- full Yazelix installation

The optional provider widgets only need their own upstream facts:

- `codex_usage` and `claude_usage` use `tokenusage` when it is available on `PATH`
- `opencode_go_usage` reads OpenCode Go SQLite databases from default locations or explicit `--db` paths
- `cpu` and `ram` read a shared `sysinfo`-backed cache with a short freshness window

None of those widget commands require Yazelix runtime paths, `yzx_control`, or a Yazelix session cache

The full Yazelix runtime keeps zjstatus `{tabs}` as the integrated tab strip so Zellij owns live tab identity, focus, click handling, and bell events. When the consumer supplies an activity-capable zjstatus input, `{tabs}` renders markers from `pipe_tab_activity` without changing native tab names. `yazelix_zellij_bar_widget tabs` is a renderer probe for Yazelix activity snapshots, not the default tab strip

## Optional Command Widgets

Standalone users can add zjstatus command widgets directly in their own copied preset. Command stdout should be short plain text because the KDL format owns the style

The main customization knobs are:

- `format_left`, `format_center`, and `format_right` for widget order
- inline `#[fg=...]` and `#[bg=...]` style tags for color
- mode and tab format keys for labels and tab display
- `command_*_command`, `command_*_format`, and `command_*_interval` for custom command widgets

Generic zjstatus placeholders such as `{mode}`, `{tabs}`, `{session}`, and `{datetime}` work without Yazelix. The integrated Yazelix runtime additionally configures `tab_activity_pipe_name` for activity markers. To add a host/status command widget, start from:

```kdl
format_right "#[fg=#ff0088,bold]{session} {datetime} {command_host} #[fg=#00ccff,bold]yzx bar "

command_host_command "hostname -s"
command_host_format " #[fg=#00ff88,bold][{stdout}]"
command_host_interval "30"
```

The packaged `share/yazelix_zellij_bar/examples/custom_command_widgets.kdl` contains a slightly larger version of this pattern

## Complete Standalone Widget Layout

`share/yazelix_zellij_bar/examples/standalone_zellij_layout.kdl` is a complete plain Zellij layout that uses every bar-owned non-workspace widget:

```kdl
layout {
    pane size=1 borderless=true {
        plugin location="file:/absolute/path/to/share/yazelix_zellij_bar/zjstatus.wasm" {
            format_left "{mode} {tabs}"
            format_center ""
            format_right "#[fg=#ff0088,bold]{session}{command_claude_usage}{command_codex_usage}{command_opencode_go_usage}{command_cpu}{command_ram} #[fg=#00ccff,bold]yzx bar "

            command_claude_usage_command "yazelix_zellij_bar_widget claude"
            command_claude_usage_format "#[fg=#bb88ff,bold]{stdout}"
            command_claude_usage_interval "10"

            command_codex_usage_command "yazelix_zellij_bar_widget codex"
            command_codex_usage_format "#[fg=#bb88ff,bold]{stdout}"
            command_codex_usage_interval "10"

            command_opencode_go_usage_command "yazelix_zellij_bar_widget opencode_go"
            command_opencode_go_usage_format "#[fg=#bb88ff,bold]{stdout}"
            command_opencode_go_usage_interval "10"

            command_cpu_command "yazelix_zellij_bar_widget cpu"
            command_cpu_format " #[fg=#ff6600][cpu {stdout}]"
            command_cpu_interval "5"

            command_ram_command "yazelix_zellij_bar_widget ram"
            command_ram_format " #[fg=#ff6600][ram {stdout}]"
            command_ram_interval "5"
        }
    }
    pane
}
```

Replace the `zjstatus.wasm` path with the installed package path. If you install through Nix profiles, `nix profile list` shows the profile entry, and the installed files live under that package output

Provider widgets maintain their own cache, lock, freshness, and error-backoff files under `$XDG_CACHE_HOME/yazelix_zellij_bar` or `$HOME/.cache/yazelix_zellij_bar`. CPU/RAM share one short-lived system-usage cache so command-widget bursts do not resample system metrics in every process. Use `--cache` only when overriding the default. Yazelix may omit it because the full runtime exports `YAZELIX_STATUS_BAR_CACHE_PATH`

## Standalone Fact Renderers

The Rust crate also exposes renderer helpers for embedders that want Yazelix-style widget text without a Yazelix runtime:

- `render_zjstatus_bar_segments` for editor, shell, terminal, custom text, and widget-tray placeholders
- `render_zjstatus_tab_label_formats` for full and compact tab labels
- `render_tab_activity_label` and `render_tab_activity_name` for plain `idle`, `busy`, or `alert` tab activity text
- `render_native_tab_strip` and `render_status_cache_tab_strip_widget` for diagnostic or standalone tab text from all-tab activity facts
- `render_yazelix_runtime_plugin_block` for the full integrated Yazelix zjstatus plugin block from typed runtime config, `appearance_mode`, and the child-owned runtime KDL template
- `render_codex_usage_status_widget` for cached Codex usage facts
- `render_windowed_agent_usage_status_widget` for Claude, OpenCode Go, or another cached windowed provider

Minimal cached provider example:

```rust
use yazelix_zellij_bar::{
    AgentUsageDisplay, AgentUsagePeriod, WindowedAgentUsageFacts,
    render_windowed_agent_usage_status_widget,
};

let text = render_windowed_agent_usage_status_widget(
    "claude",
    &WindowedAgentUsageFacts {
        five_hour_tokens: Some(15_456_373),
        weekly_tokens: Some(66_610_005),
        five_hour_remaining_percent: Some(49),
        weekly_remaining_percent: Some(80),
        ..WindowedAgentUsageFacts::default()
    },
    &[AgentUsagePeriod::FiveHour, AgentUsagePeriod::Weekly],
    AgentUsageDisplay::Both,
);
```

Those helpers are facts-in, styled-text-out. They do not read `~/.config/yazelix`, `~/.local/share/yazelix`, `yzx_control`, Yazelix session cache files, tokenusage, OpenCode databases, or pane-orchestrator state

## Yazelix-Specific Widgets

Workspace remains Yazelix-only because it is derived from Yazelix session facts and pushed into a `pipe_workspace` widget by the Yazelix pane orchestrator. Terminal display is also Yazelix-only in the integrated runtime because `yazelix_zellij_bar_widget term` reads the running session's `YAZELIX_SESSION_TERMINAL` value instead of trusting a shared generated layout. The Yazelix badge is also runtime-only: its `version` command reads the exact packaged version and `stable`, `main`, or `edge` channel from `runtime_identity.json` and renders `NOVA <compact-version> <CHANNEL>`

CPU, RAM, Codex, Claude, and OpenCode Go widgets are bar-owned standalone commands. Yazelix-only integration for those widgets is limited to generated layout wiring and default cache-path derivation from the full runtime

The full Yazelix runtime consumes this child repo for integrated zjstatus plugin rendering and the integrated standalone package. The child repo packages `zjstatus.wasm` from its pinned `zjstatus` flake input so the package does not require manual artifact copying. The standalone pin supports native bell tab formatting. A consumer that configures `tab_activity_pipe_name` must override that input with the compatible activity extension; canonical Yazelix does so while keeping tab names in native Zellij state

`yazelix_zellij_bar_widget render-yazelix-runtime --json <config>` accepts typed runtime config from Yazelix and returns the complete child-owned zjstatus plugin block rendered from `yazelix_runtime_bar.template.kdl`. The runtime config includes `appearance_mode` so the child can own dark and light status-bar palettes. The integrated runtime template uses zjstatus `{tabs}` for live Zellij tab state, upstream terminal-bell styling, and pipe-fed Yazelix activity markers; the standalone `tabs` widget is a renderer probe for the activity snapshot contract, not the default tab strip. Yazelix core still owns workspace facts, session config, pane-orchestrator activity snapshots, and runtime path resolution; this repo owns widget rendering, tab formatting, activity-label text, pipe/command-widget KDL, and the generic zjstatus plugin shape

When the main Yazelix repo forwards `#yazelix_zellij_bar`, it may make this repo's `zjstatus` input follow Yazelix's own `zjstatus` pin. Standalone users get the pin recorded in this repo's `flake.lock`

Use `share/yazelix_zellij_bar/examples/yazelix_runtime_widgets.kdl` only inside a full Yazelix runtime or after replacing the Yazelix-only workspace/version helpers. Use `share/yazelix_zellij_bar/examples/standalone_zellij_layout.kdl` for plain Zellij

## Current Limit

Zellij/zjstatus presets do not currently have a native include or variables layer. Edit the standalone KDL directly for brand, color, order, and command-widget changes; copy `yazelix_zellij_bar.template.kdl` when substituting a different pinned `zjstatus.wasm`. The integrated Yazelix runtime uses a separate template so standalone customizations stay plain Zellij KDL

## Release Process

Maintainers update the pinned zjstatus input deliberately, then validate:

```bash
nix flake lock --update-input zjstatus
nix build .#yazelix_zellij_bar
cargo test
```

If the standalone preset grows beyond zjstatus configuration, the next step is a real plugin decision rather than forking zjstatus by default
