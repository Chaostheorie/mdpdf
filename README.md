[![Build status](https://img.shields.io/github/workflow/status/Chaostheorie/mdpdf/CI?style=for-the-badge)](https://github.com/Chaostheorie/mdpdf/actions) [![License](https://img.shields.io/github/license/Chaostheorie/mdpdf?style=for-the-badge)](https://github.com/Chaostheorie/mdpdf/blob/main/LICENSE) [![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FChaostheorie%2Fmdpdf.svg?type=small)](https://app.fossa.com/projects/git%2Bgithub.com%2FChaostheorie%2Fmdpdf?ref=badge_small)

# mdpdf

mdpdf is a simple CLI tool to convert [commonmark](https://commonmark.org/) files to PDF files. It leverages [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark), [syntect](https://github.com/trishume/syntect) as well as [ammonia](https://github.com/rust-ammonia/ammonia) and [wkhtmltopdf](https://wkhtmltopdf.org/) to provide a complete experience. This includes syntax highlighting and extensions such as tables, taskslists, strikethrough or footnotes.

It features support footers with amongst other things license and name options that may help you when publishing PDFs frequently. It also features the option to use german or english translations for static texts such as a table of contents (WIP) and the footer.

> Only linux is supported for building ATM though you may be able to cross compile for windows etc with e.g., WSL

## Building

The build process is at the moment only tested on debian. You will need to build wkhtmltopdf first. See their [packaging repository](https://github.com/wkhtmltopdf/packaging/releases/) for instructions. It's required to have [GNU bash](https://www.gnu.org/software/bash/) installed though you may launch the make commands from another terminal emulator.

You will also need [GNU Grep](https://www.gnu.org/software/grep/), [GNU Coreutils (for stat)](https://www.gnu.org/software/coreutils/coreutils.html), [GNU make](https://www.gnu.org/software/make/) and [npm](https://www.npmjs.com/).

The builds are at the moment not target at a specific architecture and will use the cargo default though in the future the builds will default compile for `musl` to improve portability. I'm trying to not use platform specific libraries. When issues arise, or you are aware of any shortcomings don't hesitate to [open an issue](https://github.com/Chaostheorie/mdpdf/issues).

> Change the default name by editing src/name.txt. Please use UTF-8 for this file

Just run: `make build`

To build a debian package run `make deb` instead.

## Examples

Build with `make create-examples`. The `example.md` will be built for each theme without `-d`. All converted files as well as the source will be in `examples/`.

> This example is only partially functional due to broken taskslists

## Licensing

mdpdf is licensed under GPLv2.0 @ Cobalt <https://cobalt.rocks>. The same applies for all distributed binaries.

This uses [bootstrap 5](https://github.com/twbs/bootstrap) for styling and may embed a part of their code in the binary. The license for bootstrap can be [found here](https://github.com/twbs/bootstrap/blob/main/LICENSE).
