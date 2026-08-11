# Agent Guidelines

Shared Yazelix agent workflow and release policy live in the main repo:

- https://github.com/Yazelix/nova/blob/main/AGENTS.md
- In sibling local checkouts, read `../nova/AGENTS.md` first

Only Nova Bar-specific guidance belongs here.

## Local Scope

- This repo owns the standalone Nova Bar package and widget command.
- Keep the standalone preset generic; Nova session-specific widgets and cache paths belong in the main repo adapter.
- Preserve the package artifact names documented in the README.

## Local Commands

- `cargo test`
- `cargo build --release`
- `nix build .#nova_bar --no-link`

## Integration Notes

Nova consumes the package through its flake input and owns runtime-specific KDL rendering.
