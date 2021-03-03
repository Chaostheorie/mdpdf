# Makefile for mdpdf

setup:
	bash scripts/check.sh
install:
	bash scripts/install.sh
deb:
	bash scripts/build.sh deb
build:
	bash scripts/build.sh
