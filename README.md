# BfBB

A library intended for creating tools capable of directly interacting with
_SpongeBob SquarePants: Battle for Bikini Bottom_

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/bfbb.svg
[crates-url]: https://crates.io/crates/bfbb
[docs-badge]: https://img.shields.io/docsrs/bfbb/latest
[docs-url]: https://docs.rs/bfbb
[actions-badge]: https://github.com/BfBBModdingTools/bfbb/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/BfBBModdingTools/bfbb/actions/workflows/ci.yml

## Compatibility

This library currently only supports directly interacting with the GameCube version
of the game, running in Dolphin Emulator (Windows, [Linux](#Linux), and [MacOS](#MacOS)). More platforms will
be added over time.

## Example Projects

- [BfBB Clash](https://github.com/999gary/BfBB-Clash) - A realtime multi-player mod

## Linux

Your app will need permissions to read external process memory. This can be achieved by running with root or by setting the ptrace capability on the binary `# setcap cap_sys_ptrace=eip <path/to/binary>`

## MacOS

Due to MacOS security features your app will need to be signed with the `com.apple.security.cs.debugger`
entitlement for the OS to allow attaching to Dolphin's process. Additionally Dolphin will need to be
resigned with the `com.apple.security.get-task-allow` entitlement.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in BfBB by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
