#!/usr/bin/env bash
# -*- coding: utf-8 -*-

# Build script for mdpdf
# This script is supposed to be called from the makefile
# This script uses the first argument as build target
# options: deb = debian package with cargo-deb,
# musl = cargo with linux 64bit musl, dev = unpotimized current arch,
# otherwise = optimized current arch

# Ensuring all the programs are installed
# Check if cargo is installed
if ! command -v cargo &>/dev/null; then
    echo "You need cargo (with rustup) to build the application"
    echo "Visit: https://rustup.rs/"
    exit 1
fi

# Check if make is installed
if ! command -v make &>/dev/null; then
    echo "You need GNU Make to build the application"
    echo "Visit: https://www.gnu.org/software/make/ or just check in your preferred package manager"
    exit 1
fi

# Check if npm is installed
if ! command -v npm &>/dev/null; then
    echo "You need npm to build the application assets"
    echo "Either use npm from your package manager or visit https://www.npmjs.com/ "
    exit 1
fi

# Check if wkhtmltopdf is installed
if ! command -v wkhtmltopdf &>/dev/null; then
    echo "You need wkhtmltopdf (with libraries) to build the application"
    echo "Official Releases: https://wkhtmltopdf.org/downloads.html"
    echo "You want to download the wkhtmltox package"
    exit 1
fi

echo "Building assets"
cd src/assets
npm i
npm run compile
cd ../../

if [[ "$1" == "deb" ]]; then
    echo "Building debian package"
    cargo deb
elif [[ "$1" == "dev" ]]; then
    echo "Building dev executable"
    cargo build
elif [[ "$1" == "musl" ]]; then
    echo "Building executable with musl"
    echo "Please ensure you have musl-tools (musl-gcc) and musl-dev installed"
    echo "You will need the relevant toolchain: (rustup) rustup target add x86_64-unknown-linux-musl"
    cargo build --release --target x86_64-unknown-linux-musl
else
    echo "Building executable"
    cargo build --release
fi
