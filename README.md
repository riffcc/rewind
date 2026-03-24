# rewind

A TUI for [Replay](https://github.com/riffcc/replay) — watch, control, and understand autonomous software orchestration.

## What is Rewind?

Rewind is the interface to Replay. It lets you:

- **Watch** Replay work in real time — see which issue it picked up, what the agent is reading, what it's writing, tool calls as they happen
- **Control** the loop — pause, skip, retry, adjust parameters
- **Inspect** results — diffs, build output, test results, agent reasoning
- **Manage** issues — browse the Beads queue, create issues, set priorities

## The twist

Rewind is also Replay's testbed. Every feature request against Rewind is a real issue that Replay can solve. The benchmark isn't synthetic — it's "can Replay improve its own GUI?"

## Usage

```
rewind                    # watch Replay in the current repo
rewind --target ../myapp  # watch Replay working on another project
```

## Stack

- **Rust** + **ratatui** for the TUI
- **Beads** for issue tracking
- Talks to Replay's engine (shared crate or IPC)
