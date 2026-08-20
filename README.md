# GitQuick

[![Build](https://github.com/gordziejonok/gitquick/actions/workflows/build.yml/badge.svg)](https://github.com/gordziejonok/gitquick/actions/workflows/build.yml)
[![Crates.io Version](https://img.shields.io/crates/v/gitquick)](https://crates.io/crates/gitquick)

`gq` is a command line tool that simplifies common `git` workflows.

## Installation

### Windows

1. Download the latest exe file from [GitHub](https://github.com/gordziejonok/gitquick/releases).
2. Add the path to the `gq.exe` file to your `Path` environment variable.

### Cargo

```
cargo install gitquick
```

## Configuration

Use the following command to configure `gq`:

```
gq config <NAME> <VALUE>
```

### Options

| NAME | VALUE | Description |
| - | - | - |
| `commit.conventional` | Bool | Enable [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/). Default is `false`. |
| `commit.types` | Comma separated types | Types for conventional commits. Default is `build, ci, docs, feat, fix, perf, refactor, style, test, revert`. |
| `commit.ticket` | Bool | Enable ticket number integration. Default is `false`. |
