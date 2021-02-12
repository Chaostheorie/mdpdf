#!/usr/bin/env bash
# -*- coding: utf-8 -*-
# Chck if all dependencies are installed and build when possible
# by Cobalt <https://cobalt.rocks> under GPLv3.0

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

# Check if wkhtmltopdf is installed
if ! command -v wkhtmltopdf &>/dev/null; then
    echo "You need wkhtmltopdf (with libraries) to build the application"
    echo "Official Releases: https://wkhtmltopdf.org/downloads.html"
    exit 1
fi

# Check if system is debian based
if [ "$(grep -Ei 'debian|buntu|mint' /etc/*release)" ]; then
    if ! command -v cargo &>/dev/null; then
        echo "You need cargo deb to build deb packages."
        echo "Use: cargo install cargo-deb"
        exit 1
    else
        make deb
        echo "Output package to: target/debian. Please use dpkg or apt to install"
    fi
else
    make build
    echo "Done building. The binary is at target/release/mdpdf"
fi

echo "Thank you for using mdpdf. If you have an issue or find a bug please report it [GITHUB]"
exit 0
