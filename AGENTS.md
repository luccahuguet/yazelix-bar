# Agent Guidelines

Shared Yazelix agent workflow and release policy live in the main repo:

- https://github.com/Yazelix/nova/blob/main/AGENTS.md
- In sibling local checkouts, read `../yazelix/AGENTS.md` first

Only Zellij Status Kit-specific guidance belongs here.

## Local Scope

- This repo owns the standalone Zellij bar package and widget command.
- Keep the standalone preset generic; Yazelix session-specific widgets and cache paths belong in the main repo adapter.
- Preserve the package artifact names documented in the README.

## Local Commands

- `cargo test`
- `cargo build --release`
- `nix build .#yazelix_zellij_bar --no-link`

## Integration Notes

Main Yazelix consumes the package through its flake input and owns runtime-specific KDL rendering.
