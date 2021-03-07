#!/usr/bin/env bash
# -*- coding: utf-8 -*-

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
    echo "Visit: https://www.npmjs.com/ or just check in your preferred package manager"
    exit 1
fi

# Check if wkhtmltopdf is installed
if ! command -v wkhtmltopdf &>/dev/null; then
    echo "You need wkhtmltopdf (with libraries) to build the application"
    echo "Official Releases: https://wkhtmltopdf.org/downloads.html"
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
else
    echo "Building executable"
    cargo build --release
fi
