#!/usr/bin/env bash
# -*- coding: utf-8 -*-

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
