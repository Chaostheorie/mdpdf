# Makefile for mdpdf

dev:
	bash scripts/build.sh dev
deb:
	bash scripts/build.sh deb
build:
	bash scripts/build.sh
clean:
	rm -rf target
	cd src/assets/ && npm run clean && cd ../../
