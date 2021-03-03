# mdpdf

mdpdf is a simple cli tool to convert markdown files to pdf files. It leverages [comrak]() and [wkhtmltopdf]() to enable you to either use the included css files or customize your pdfs.

> Only linux is supported at the moment though you may be able to cross compile.

## Building

The build process is at the moment only tested on debian. You will need to build wkhtmlpdf first. See their [packaging repository](https://github.com/wkhtmltopdf/packaging) for instructions.

You will also need GNU Grep, GNU Stat, GNU make and npm or yarn.

> Change the default name by editing src/name.txt. Please use UTF-8 for this file

Just run: `make build`

To build a debian package run `make deb` instead.

## Licensing

mdpdf is licensed under GPLv3.0 @ Cobalt <https://ocbalt.rocks>.

This uses [bootstrap 5](https://github.com/twbs/bootstrap) for styling and may embed a part of their code in the binary. The license for bootstrap is [found here](https://github.com/twbs/bootstrap/blob/main/LICENSE). It also relies on [pygments stylesheets](https://github.com/richleland/pygments-css) for code styling ([License](https://github.com/richleland/pygments-css/blob/master/UNLICENSE.txt)).

## TODO

-   Fix purgecss (will shrink binary size)
-   Fix styling
-   Windows and/ or Mac support
-   Fix tmp file messages
-   Add links to building instructions
