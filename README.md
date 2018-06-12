# rust-clamav

Bindings for [libclamav](https://www.clamav.net/) in Rust.

[![Build Status](https://travis-ci.org/icebergdefender/rust-clamav.svg?branch=master)](https://travis-ci.org/icebergdefender/rust-clamav)

# Overview

rust-clamav is a safe library for interacting with libclamav from Rust.
The low-level C API is wrapped in idomatic and safe Rust code.

# Documentation

 - [crates.io documentation](https://docs.rs/clamav/)

# Requirements

## Rust

We currently target the latest stable release of Rust (1.26), and Cargo (1.26).

## Linux
`libclamav.so.7` should be available on the `PATH` at runtime (or `LD_LIBRARY_PATH` if it lives somewhere non-standard).

Debian/Ubuntu minimal example:

`$ sudo apt install libclamav7`

### Freshclam

Freshclam can optionally be installed to keep the defintions in `/var/lib/clamav` up to date:

`$ sudo apt install clamav-freshclam`