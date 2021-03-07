# mdpdf

mdpdf is a simple cli tool to convert commonmark files to pdf files. It leverages [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) syntect as well as ammonia and [wkhtmltopdf]() to provide a complete experience. This includes syntax highlighting and extensions such as tables, taskslists, strikethorugh or footnotes.

> Only linux is supported at the moment though you may be able to cross compile.

## Building

The build process is at the moment only tested on debian. You will need to build wkhtmlpdf first. See their [packaging repository](https://github.com/wkhtmltopdf/packaging) for instructions. It's required to have [GNU bash](https://www.gnu.org/software/bash/) installed though you may launch the make commands from another terminal emulator.

You will also need [GNU Grep](https://www.gnu.org/software/grep/), [GNU Coreutils (for stat)](https://www.gnu.org/software/coreutils/coreutils.html), [GNU make](https://www.gnu.org/software/make/) and npm.

> Change the default name by editing src/name.txt. Please use UTF-8 for this file

Just run: `make build`

To build a debian package run `make deb` instead.

## Licensing

mdpdf is licensed under GPLv3.0 @ Cobalt <https://ocbalt.rocks>.

This uses [bootstrap 5](https://github.com/twbs/bootstrap) for styling and may embed a part of their code in the binary. The license for bootstrap is [found here](https://github.com/twbs/bootstrap/blob/main/LICENSE).

## 

## TODO

### 0.1.1  (WIP)

- [ ] proper footnote styling
- [x] syntax highlighting

### 0.1.2

> This release will come with prebuilt static binaries for linux and prebuilt debian package

-   [x] Fix removal process
-   [ ] Custom stylesheet option (compile & runtime)
-   [ ] More dcumentation
-   [x] Fix tmp file messages
-   [ ] Add links to README.md

### 0.1.3 (WIP)

-   [ ] Fix purgecss (will shrink binary size)

### Planned

-   [ ] Windows and/ or Mac support
