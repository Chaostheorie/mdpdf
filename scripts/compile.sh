#!/usr/bin/env bash
# -*- coding: utf-8 -*-

# Script for compiling frontend stylsheets as part of mdpdf assets
# Requires being run from npm or npm with purgecss and sass (dart) being installed
# WARNING: This requires GNU Coreutils Stat and GNU Grep being installed
# This script only compiles if one of the files changed
# You will need to remove your static/scss/.dir-changes file, if you're adding a new scss file
# under GPL v2.0 @ Cobalt <cobalt.rocks> (see mdpdf LICENSE)

# Constants
CHANGE_FILE="scss/.dir-changes"

# Compiling scss and removing unnecessary css
compile_scss() {
    echo "Compiling SCSS -> CSS"
    sass --style=compressed --update --no-source-map --load-path scss/ --load-path node_modules/ scss/de.scss css/de.css
    sass --style=compressed --update --no-source-map --load-path scss/ --load-path node_modules/ scss/de.scss css/en.css

    purgecss --config purgecss.config.js --css css/de.css --content "../../templates/**/*\.html" --output css/
    purgecss --config purgecss.config.js --css css/en.css --content "../../templates/**/*\.html" --output css/
    echo "Done"
}

# Builds change file
build_change_file() {
    # Ensure file is present && empty
    if [ -f "$CHANGE_FILE" ]; then
        rm "$CHANGE_FILE"
        touch "$CHANGE_FILE"
    fi

    for f in ./scss/*.scss; do
        FILE_MODIFY=$(stat "$f" | grep Modify)
        echo "$f:$FILE_MODIFY" >>"$CHANGE_FILE"
    done
}

# loads file and checks stats => 1: recompilation required 0: nothing todo
load_change_file() {
    INDEX=0
    FILES=(./scss/*.scss) # This is quite a handy way of indexing files

    # This just reads it line by line and compares the current stat to the saved stat
    # I need to extend this, when I have too much time, to support checking
    # for Filenames and have graceful handling of new files
    while IFS= read -r line; do
        CURRENT_CHANGE=$(stat "${FILES[INDEX]}" | grep Modify)
        if [ "${FILES[INDEX]}:$CURRENT_CHANGE" != "$line" ]; then
            echo 1
        fi
        INDEX=$((INDEX + 1))
    done <"$CHANGE_FILE"
    echo 0
}

# Get modified timestamp
if [ -f "$CHANGE_FILE" ]; then
    if [ "$(load_change_file)" != "0" ]; then
        compile_scss
        build_change_file
    fi
    # When no changes made -> no recompilation needed
else
    # When the file doesn't exists it should be built and the scss should be compiled
    build_change_file
    compile_scss
fi
