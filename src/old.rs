#[macro_use]
extern crate clap;
use std::fs;
use std::process::Command;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Cobalt <cobalt.rocks>")
        (about: "Converts md to pdf with wkhtlmtopdf and pandoc")
        (@arg INPUT: +required +takes_value "Sets the input file to use")
        (@arg OUTPUT: +required +takes_value "Sets the output file to use")
        (@arg name: -n --name "Add name and date as footer")
        (@arg print: -p --print "Add offset on left side for printer")
        (@arg shrink: -s --shrink "Shrink output with ghostscript")
        (@arg toc: -t --toc "Add toc")
        (@arg de: -d --german "Static Headings in german")
        (@arg book: -b --book "Make format as book")
        (@arg html: --keep -k "Keep generated html file")
    )
    .get_matches();

    // evaluate script args
    let left = if matches.is_present("print") {
        "20mm"
    } else {
        "10mm"
    }
    .to_string();

    let inp = matches.value_of("INPUT").unwrap();
    let tmp = format!("{}.html", inp);
    let book = matches.is_present("book");
    let out = matches.value_of("OUTPUT").unwrap();

    // create out file
    let _ = fs::File::create(out);

    // prepare pandoc and wkhtmltopdf args
    let extension = if matches.is_present("de") {
        "-de.css"
    } else {
        ".css"
    };
    let css_file = if book {
        format!(
            "/home/cobalt/Documents/Schule/mdpdf/assets/css/markdown-8{}",
            extension
        )
    } else {
        format!(
            "/home/cobalt/Documents/Schule/mdpdf/assets/css/markdown7{}",
            extension
        )
    };

    let mut pandoc_args = vec![
        inp,
        "-f",
        "markdown+footnotes+autolink_bare_uris+inline_code_attributes",
        "-t",
        "html5",
        "--css",
        &css_file,
        "-s",
        "--wrap=preserve",
        "-o",
        &tmp,
    ];
    let mut wkhtmltopdf_args = vec![
        "--page-size",
        "A4",
        "-L",
        &left,
        "-B",
        "10mm",
        "-T",
        "10mm",
        "-R",
        "10mm",
        "--dpi",
        "1200",
        "--enable-javascript",
        "--enable-local-file-access",
        "--encoding",
        "UTF-8",
        "--print-media-type",
        &tmp,
        &out,
    ];

    // check if name is set
    if matches.is_present("name") {
        wkhtmltopdf_args.insert(0, "--footer-html");
        wkhtmltopdf_args.insert(1, "/home/cobalt/Documents/Schule/mdpdf/assets/name.html");
    };

    if matches.is_present("toc") {
        pandoc_args.insert(3, "--toc");
    }

    // use pandoc to create html file
    Command::new("/usr/bin/pandoc")
        .args(&pandoc_args)
        .output()
        .expect("OK");

    // add header
    let contents = fs::read_to_string(&tmp).expect("Something went wrong reading the file");
    let mut header = fs::read_to_string("/home/cobalt/Documents/Schule/mdpdf/assets/header.html")
        .expect("Something went wrong reading the file");
    header.push_str(&contents);

    // call wkhtmltopdf
    Command::new("/usr/local/bin/wkhtmltopdf")
        .args(&wkhtmltopdf_args)
        .output()
        .expect("OK");

    // remove tmp file if no --html arg
    if !matches.is_present("html") {
        Command::new("rm").args(&["-f", &tmp]).output().expect("OK");
    }

    // shrink
    if matches.is_present("shrink") {
        Command::new("/usr/bin/gs")
            .args(&[
                "-sDEVICE=pdfwrite",
                "-dCompatibilityLevel=1.4",
                "-dPDFSETTINGS=/default",
                "-dNOPAUSE",
                "-dQUIET",
                "-dBATCH",
                "-dDetectDuplicateImages",
                "-dCompressFonts=true",
                "-r150",
                &format!("-sOutputFile={}", &out),
                &out,
            ])
            .output()
            .expect("OK");
    }
}
