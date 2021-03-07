# Makefile for mdpdf

setup:
	bash scripts/install.sh
dev:
	bash scripts/build.sh dev
deb:
	bash scripts/build.sh deb
build:
	bash scripts/build.sh
