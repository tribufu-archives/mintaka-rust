#!/usr/bin/env sh

# Windows

echo "Checking for windows-aarch64"
New-Item "bin/windows/aarch64" -ItemType Directory -Force
cargo check --target aarch64-pc-windows-msvc

echo "Checking for windows-i586"
New-Item "bin/windows/i586" -ItemType Directory -Force

echo "Checking for windows-i686"
New-Item "bin/windows/i686" -ItemType Directory -Force
cargo check --target i686-pc-windows-msvc

echo "Checking for windows-x86_64"
New-Item "bin/windows/x86_64" -ItemType Directory -Force
cargo check --target x86_64-pc-windows-msvc

# Mac

echo "Checking for mac-aarch64"
New-Item "bin/mac/aarch64" -ItemType Directory -Force
cargo check --target aarch64-apple-darwin

echo "Checking for mac-i686"
New-Item "bin/mac/i686" -ItemType Directory -Force
cargo +nightly check -Z build-std --target i686-apple-darwin

echo "Checking for mac-x86_64"
New-Item "bin/mac/x86_64" -ItemType Directory -Force
cargo check --target x86_64-apple-darwin

# Linux

echo "Checking for linux-aarch64"
New-Item "bin/linux/aarch64" -ItemType Directory -Force
cargo check --target aarch64-unknown-linux-gnu

echo "Checking for linux-arm"
New-Item "bin/linux/arm" -ItemType Directory -Force
cargo check --target arm-unknown-linux-gnueabi

echo "Checking for linux-armv7"
New-Item "bin/linux/armv7" -ItemType Directory -Force
cargo check --target armv7-unknown-linux-gnueabihf

echo "Checking for linux-i586"
New-Item "bin/linux/i586" -ItemType Directory -Force
cargo check --target i586-unknown-linux-gnu

echo "Checking for linux-i686"
New-Item "bin/linux/i686" -ItemType Directory -Force
cargo check --target i686-unknown-linux-gnu

echo "Checking for linux-mips"
New-Item "bin/linux/mips" -ItemType Directory -Force
cargo check --target mips-unknown-linux-gnu

echo "Checking for linux-mips64"
New-Item "bin/linux/mips64" -ItemType Directory -Force
cargo check --target mips64-unknown-linux-gnuabi64

echo "Checking for linux-mips64el"
New-Item "bin/linux/mips64el" -ItemType Directory -Force
cargo check --target mips64el-unknown-linux-gnuabi64

echo "Checking for linux-mipsel"
New-Item "bin/linux/mipsel" -ItemType Directory -Force
cargo check --target mipsel-unknown-linux-gnu

echo "Checking for linux-powerpc"
New-Item "bin/linux/powerpc" -ItemType Directory -Force
cargo check --target powerpc-unknown-linux-gnu

echo "Checking for linux-powerpc64"
New-Item "bin/linux/powerpc64" -ItemType Directory -Force
cargo check --target powerpc64-unknown-linux-gnu

echo "Checking for linux-riscv64gc"
New-Item "bin/linux/riscv64gc" -ItemType Directory -Force
cargo check --target riscv64gc-unknown-linux-gnu

echo "Checking for linux-s390x"
New-Item "bin/linux/s390x" -ItemType Directory -Force
cargo check --target s390x-unknown-linux-gnu

echo "Checking for linux-sparc64"
New-Item "bin/linux/sparc64" -ItemType Directory -Force
cargo check --target sparc64-unknown-linux-gnu

echo "Checking for linux-x86_64"
New-Item "bin/linux/x86_64" -ItemType Directory -Force
cargo check --target x86_64-unknown-linux-gnu

# FreeBSD

echo "Checking for freebsd-aarch64"
New-Item "bin/freebsd/aarch64" -ItemType Directory -Force
cargo +nightly check -Z build-std --target aarch64-unknown-freebsd

echo "Checking for freebsd-i686"
New-Item "bin/freebsd/i686" -ItemType Directory -Force
cargo check --target i686-unknown-freebsd

echo "Checking for freebsd-x86_64"
New-Item "bin/freebsd/x86_64" -ItemType Directory -Force
cargo check --target x86_64-unknown-freebsd

# NetBSD

echo "Checking for netbsd-aarch64"
New-Item "bin/netbsd/aarch64" -ItemType Directory -Force
cargo +nightly check -Z build-std --target aarch64-unknown-netbsd

echo "Checking for netbsd-x86_64"
New-Item "bin/netbsd/x86_64" -ItemType Directory -Force
cargo check --target x86_64-unknown-netbsd

# OpenBSD

echo "Checking for openbsd-aarch64"
New-Item "bin/openbsd/aarch64" -ItemType Directory -Force
cargo +nightly check -Z build-std --target aarch64-unknown-openbsd

echo "Checking for openbsd-i686"
New-Item "bin/openbsd/i686" -ItemType Directory -Force
cargo +nightly check -Z build-std --target i686-unknown-openbsd

echo "Checking for openbsd-x86_64"
New-Item "bin/openbsd/x86_64" -ItemType Directory -Force
cargo +nightly check -Z build-std --target x86_64-unknown-openbsd

# Redox

echo "Checking for redox-aarch64"
New-Item "bin/redox/aarch64" -ItemType Directory -Force
cargo +nightly check -Z build-std --target aarch64-unknown-redox

echo "Checking for redox-x86_64"
New-Item "bin/redox/x86_64" -ItemType Directory -Force
cargo check --target x86_64-unknown-redox

# Fuchsia

echo "Checking for fuchsia-aarch64"
New-Item "bin/fuchsia/aarch64" -ItemType Directory -Force
cargo +nightly check -Z build-std --target aarch64-fuchsia

echo "Checking for fuchsia-x86_64"
New-Item "bin/fuchsia/x86_64" -ItemType Directory -Force
cargo +nightly check -Z build-std --target x86_64-fuchsia

# Android

echo "Checking for android-aarch64"
New-Item "bin/android/aarch64" -ItemType Directory -Force
cargo check --target aarch64-linux-android

echo "Checking for android-armv7"
New-Item "bin/android/armv7" -ItemType Directory -Force
cargo check --target armv7-linux-androideabi

echo "Checking for android-i686"
New-Item "bin/android/i686" -ItemType Directory -Force
cargo check --target i686-linux-android

echo "Checking for android-x86_64"
New-Item "bin/android/x86_64" -ItemType Directory -Force
cargo check --target x86_64-linux-android

# IOS

echo "Checking for ios-aarch64"
New-Item "bin/ios/aarch64" -ItemType Directory -Force
cargo check --target aarch64-apple-ios

echo "Checking for ios-x86_64"
New-Item "bin/ios/x86_64" -ItemType Directory -Force
cargo check --target x86_64-apple-ios

# Web

echo "Checking for wasm-wasm32"
New-Item "bin/wasm/wasm32" -ItemType Directory -Force
cargo check --target wasm32-unknown-unknown

echo "Checking for wasi-wasm32"
New-Item "bin/wasi/wasm32" -ItemType Directory -Force
cargo check --target wasm32-wasi

echo "Checking for emscripten-wasm32"
New-Item "bin/emscripten/wasm32" -ItemType Directory -Force
cargo check --target wasm32-unknown-emscripten
