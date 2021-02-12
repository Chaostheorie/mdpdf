# mdpdf

mdpdf is a simple cli tool to convert markdown files to pdf files. It leverages [comrak]() and [wkhtmltopdf]() to enable you to either use the included css files or customize your pdfs.

## Building

The build process is at the moment only tested on debian. You will need to build wkhtmlpdf first. See their [packaging repository](https://github.com/wkhtmltopdf/packaging) for instructions.

> Change the default name by editing src/name.txt. Please use UTF-8 for this file

Just run: `make setup`

## Licensing

mdpdf is licensed under GPLv3.0 @ Cobalt <https://ocbalt.rocks>.

The source code is accompanied by a copy of [mdb-ui-kit](https://github.com/mdbootstrap/mdb-ui-kit) that is embedded on compile time. Please refer to their [license](https://github.com/mdbootstrap/mdb-ui-kit/blob/master/License.pdf) for their code.