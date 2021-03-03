#!/usr/bin/env bash
# -*- coding: utf-8 -*-

echo "Building assets"
cd src/assets
npm run compile
cd ../../

if [[ "$1" == "deb" ]]; then
    echo "Building debian package"
    cargo deb
else
    echo "Building executable"
    cargo b --release
fi
