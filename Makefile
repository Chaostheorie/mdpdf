# Makefile for mdpdf

install:
	bash scripts/install.sh
deb:
	cargo deb
build:
	cargo build --release
