# Makefile for mdpdf

# Build binary/ and or package
dev:
	bash scripts/build.sh dev
deb:
	bash scripts/build.sh deb
build:
	bash scripts/build.sh
musl:
	bash scripts/build.sh musl
# Clean all artifacts from building
clean:
	rm -rf target .footer* .document*
	cd src/assets/ && npm run clean && cd ../../
# Creating examples for all themes in examples/
create-examples:
	cd examples && bash ../scripts/create.sh
