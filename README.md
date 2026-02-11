# Muscle

A lightweight CLI for managing Bicep monorepos on Azure.

## What it does

Muscle helps you organize, build, and publish Bicep modules in a monorepo structure. Less ceremony, more infrastructure.

## Installation

```bash
cargo install muscle
```

## Quick Start

Initialize a new Bicep monorepo:

```bash
muscle init --author "Your Name" --version 0.1.0
```

List all modules:

```bash
muscle module list
```

Show details for a specific module:

```bash
muscle module show my-module
```

## Commands

### `muscle init`

Bootstrap a new Bicep monorepo with the standard structure.

```bash
muscle init [OPTIONS]

Options:
  --include-modules <INCLUDE_MODULES>  Glob pattern to include existing modules
  -f, --force                          Overwrite existing files
  -a, --author <AUTHOR>                Module author [default: "John Doe"]
  -v, --version <VERSION>              Initial version [default: 0.1.0]
```

**Including existing modules:**

Use `--include-modules` with a glob pattern containing `**/main.bicep` to discover modules in your directory structure:

```bash
muscle init --include-modules "modules/**/main.bicep"
```

This will find modules like:

```
modules/
├── aks/
│   └── main.bicep
└── action-groups/
    └── main.bicep
```

The pattern must include `**` (recursive wildcard) and end with a `.bicep` file.

### `muscle module list`

List all modules in the monorepo.

```bash
muscle module list [OPTIONS]

Options:
  -p, --pretty    Pretty-print output
```

### `muscle module show`

Show detailed information about a specific module.

```bash
muscle module show [OPTIONS] <NAME>

Arguments:
  <NAME>    Module name

Options:
  -p, --pretty    Pretty-print output
```

## Global Options

```bash
-r, --root <ROOT>    Path to monorepo root [env: MUSCLE_ROOT] [default: .]
```

Set `MUSCLE_ROOT` to avoid passing `--root` every time:

```bash
export MUSCLE_ROOT=/path/to/your/monorepo
```
