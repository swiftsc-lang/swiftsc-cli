# SwiftSC-Lang CLI

This directory is reserved for the standalone CLI tool.

## Current Implementation

The CLI is currently implemented in `/swiftsc-compiler/swiftsc-driver/`.

## Commands

- `swiftsc init` - Initialize project
- `swiftsc build` - Compile to WASM
- `swiftsc check` - Type check
- `swiftsc test` - Run tests
- `swiftsc deploy` - Deploy contracts

See `/docs/cli.md` for complete documentation.

## Future

This directory may be used for a standalone CLI package separate from the compiler workspace.
