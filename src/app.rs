use crate::document::{CC4Licenses, ClapOption};
use clap::{App, AppSettings, Arg, SubCommand};

/* default name - can be included by having a name.txt file in src at compilation time */
static NAME: &'static str = include_str!("name.txt");

fn name_arg() -> Arg<'static, 'static> {
    let arg = Arg::with_name("name")
        .short("-n")
        .long("--name")
        .takes_value(true)
        .help("Add name and date to pdf footer");

    if NAME != "" && NAME != "\n" {
        arg.default_value(NAME.trim())
    } else {
        arg
    }
}

pub fn app() -> App<'static, 'static> {
    App::new("mdpdf")
        .version("0.1.0 (WIP)")
        .author("Cobalt <https://cobalt.rocks>")
        .about("Converts md to pdf with wkhtlmtopdf and pulldown-cmark")
        .setting(AppSettings::AllowMissingPositional)
        .arg(
            Arg::with_name("INPUT")
                .takes_value(true)
                .required(true)
                .hide_default_value(true)
                .default_value("EMPTY")
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .takes_value(true)
                .required(true)
                .hide_default_value(true)
                .default_value("EMPTY")
                .help("Sets the output path to write pdf to"),
        )
        .arg(
            name_arg()
        )
        .arg(
            Arg::with_name("extensions")
            .help("Commonmark Extensions to be used. By default all are activated. Commas are supported as separators when specifying multiple.")
            .takes_value(true)
            .possible_values(&["footnotes", "table", "tasklist", "smart-punctuation", "strikethrough"])
            .long("--extensions")
        )
        .arg(
            Arg::with_name("pagesize")
            .long("--pagesize")
            .takes_value(true)
            .help("PDF pagesize")
            .default_value("A4")
            .possible_values(&["A3", "A4", "A5", "A6"])
        )
        .arg(
            Arg::with_name("date")
            .long("--date")
            .requires("name")
            .takes_value(true)
            .help("Defined date for footer specifically (e.g. 2014-11-28) [Default: today]")
            .long_help("Defined date for footer specifically following the syntax %Y-%m-%d (e.g. 2014-11-28) [Default: today]")
        )
        .arg(
            Arg::with_name("keep")
            .short("-k")
            .takes_value(false)
            .help("Keep temporary files (including body and footer")
        )
        .arg(
            Arg::with_name("orientation")
            .long("--orientation")        
            .help("PDF document orientation")
            .possible_values(&["landscape", "portrait"])
            .default_value("portrait")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("stylesheet")
            .short("-s")
            .long("--stylesheet")
            .takes_value(true)
            .help("Custom css stylesheet in addition to theme")
        )
        .arg(
            Arg::with_name("theme")
            .long("--theme")
            .help("Theme for document")
            .takes_value(true)
            .default_value("light")
            .possible_values(&["lime", "light", "night"])
        )
        .arg(
            Arg::with_name("de")
            .short("-d")
            .long("--german")
            .help("Static content in german") // ATM only affecting footer
        )
        .arg(
            Arg::with_name("safe")
            .long("--unsafe")
            .help("Don't clean html before converting with wkhtmltopdf") // ATM only affecting footer
        )
        .arg(
            Arg::with_name("toc")
            .help("Add table of contents (not implemented ATM)")
            .long("--toc")
        )
        .arg(
            Arg::with_name("title")
            .long("--title")
            .short("-t")
            .takes_value(true)
            .help("PDF document title")
        )
        .arg(
            Arg::with_name("license")
            .short("-l")
            .long("--license")
            .help("Add CC 4.0 license to footer")
            .requires("name")
            .takes_value(true)
            .possible_values(CC4Licenses::options())
        )
        .subcommand(
            SubCommand::with_name("changelog")
            .about("Print changelog for current build")
        )
}
