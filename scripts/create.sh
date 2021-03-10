#!/usr/bin/env bash
# -*- coding: utf-8 -*-

# Script for compiling examples for mdpdf
# under GPLv2.0 @ Cobalt <https://cobalt.rocks>

# Check if mdpdf is installed
if ! command -v mdpdf &>/dev/null; then
    echo "You need mdpf to build the examples"
    echo "Just use 'make build' in the root of this repository"
    exit 1
fi

THEMES=(lime night light)

convert() {
    mdpdf --theme $1 example.md "example-$1.pdf"
}

for theme in ${THEMES[@]}; do
    convert "$theme"
done
