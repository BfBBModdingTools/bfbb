# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking

- Refactored `GameInterface` into a struct that is generic over any backend-implementation
- Refactored `DolphinInterface` into a struct that implements `InterfaceProvider<Backend=DolphinBackend>`

### Additions

- Added `InterfaceProvider` trait for types that are caple of providing a game interface.
- Added `MockInterface` for writing testing logic without using a real backend.
- Added `GameVar` and `GameVarMut` traits to represent accessible regions of game memory in a strongly-typed manner.
- Added `InterfaceError::HookingFailed` for when a hooking attempt fails. `InterfaceError::Unhooked`
  represents when a previously hooked interface becomes unhooked.

### Fixed

- `GameInterface::unlock_task` will now only set a task's counter to `1` if it was previously `0`. This will preserve a
  value of `3` which can also correspond to an "silver" task (e.g. Infestation at the Krusty Krab)

### Removals

- Removed several methods on `GameInterface` that would simply get/set memory regions that are now represented as `GameVar`s

## [0.2.1] - 2022-09-22

### Fixed

- Fixed `DolphinInterface::unlock_powers` writing values of the wrong size.

## [0.2.0] - 2022-09-22

### Additions

- Added functions for manipulating game state.
- Added function for unlocking a task.

## [0.1.0] - 2022-06-20

- Initial Release

[unreleased]: https://github.com/BfBBModdingTools/bfbb/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/BfBBModdingTools/bfbb/releases/tag/v0.2.1
[0.2.0]: https://github.com/BfBBModdingTools/bfbb/releases/tag/v0.2.0
[0.1.0]: https://github.com/BfBBModdingTools/bfbb/releases/tag/v0.1.0
