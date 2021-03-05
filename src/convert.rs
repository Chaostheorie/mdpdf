use crate::document::{Document, Footer};
use crate::{error, info, warning};
use clap::ArgMatches;
use comrak::ComrakOptions;
use wkhtmltopdf::{Orientation, PageSize, PdfApplication, Size};

fn parse(wrapped: Option<&str>) -> PageSize {
    match wrapped {
        Some(size) => match size {
            "A3" => PageSize::A3,
            "A4" => PageSize::A4,
            "A5" => PageSize::A5,
            "A6" => PageSize::A6,
            _ => PageSize::A4,
        },
        None => PageSize::A4,
    }
}

pub fn convert(html: String, name: Option<String>, matches: &ArgMatches) {
    let mut app = match PdfApplication::new() {
        Ok(app) => app,
        Err(e) => error(format!("Failed to init PDF Application: {}", e)),
    };

    let title = match matches.value_of("title") {
        Some(title) => title,
        None => matches
            .value_of("INPUT")
            .unwrap()
            .strip_suffix(".md")
            .unwrap(),
    };

    let margin = match matches.value_of("margin") {
        Some(margin) => match margin.parse::<u32>() {
            Ok(margin) => Size::Millimeters(margin),
            Err(_) => {
                info("Invalid Margin supplied");
                Size::Millimeters(10)
            }
        },
        None => Size::Millimeters(10),
    };

    let orientation = match matches.value_of("orientation") {
        Some(orientation) => match orientation {
            "portrait" => Orientation::Portrait,
            "landscape" => Orientation::Landscape,
            _ => Orientation::Portrait,
        },
        None => Orientation::Portrait,
    };

    let mut builder = app.builder();

    // save has_name & footer_path here to make the available later
    let has_name = name.is_some();

    if has_name {
        let footer = Footer::new(name.unwrap(), matches);
        let footer_path = match footer.to_file() {
            Ok(path) => path,
            Err(e) => error(format!("Failed to render footer: {}", e)),
        };

        if matches.is_present("print") {
            unsafe { builder.object_setting("margin.left", "12cm") };
        }

        let out_result = unsafe {
            builder
                .orientation(orientation)
                .object_setting("footer.htmlUrl", footer_path) // pretty sure this isn't totally safe
                .object_setting("load.blockLocalFileAccess", "false")
                .margin(margin)
                .page_size(parse(matches.value_of("pagesize")))
                .title(&title)
                .build_from_html(&html)
        };

        let mut out = match out_result {
            Ok(pdf) => pdf,
            Err(e) => error(format!("Failed to generate PDF: {}", e)),
        };

        let path = matches.value_of("OUTPUT").unwrap();
        match out.save(path) {
            Ok(_) => info(format!("Generated PDF and saved to {}", path)),
            Err(e) => error(format!("Failed to save PDF to {}: {}", path, e)),
        };
    } else {
        if matches.is_present("print") {
            unsafe { builder.object_setting("margin.left", "12cm") };
        }

        let out_result = unsafe {
            builder
                .orientation(orientation)
                .margin(margin)
                .page_size(parse(matches.value_of("pagesize")))
                .object_setting("load.blockLocalFileAccess", "false")
                .title(&title)
                .build_from_html(&html)
        };

        let mut out = match out_result {
            Ok(pdf) => pdf,
            Err(e) => error(format!("Failed to generate PDF: {}", e)),
        };

        let path = matches.value_of("OUTPUT").unwrap();
        match out.save(path) {
            Ok(_) => info(format!("Generated PDF and saved to {}", path)),
            Err(e) => error(format!("Failed to save PDF to {}: {}", path, e)),
        };
    }

    if !matches.is_present("keep") {
        match Document::remove_artifacts() {
            Ok(_) => (),
            Err(e) => warning(format!("Failed to remove old document artifact: {}", e)),
        };
    } else {
        match Document::to_file(html) {
            Ok(path) => info(format!("Kept document body under: {}", path)),
            Err(e) => error(format!(
                "Failed to render tmp document (try without -k): {}",
                e
            )),
        };
    }
}

pub fn build_options(matches: &ArgMatches) -> ComrakOptions {
    let mut options = ComrakOptions::default();

    if matches.is_present("extensions") {
        // extract data and prepare new options obj
        let activated_extensions: Vec<&str> =
            matches.value_of("extensions").unwrap().split(",").collect();
        // check for extensions
        options.extension.strikethrough = activated_extensions.contains(&"strikethrough");
        options.extension.superscript = activated_extensions.contains(&"superscript");
        options.extension.footnotes = activated_extensions.contains(&"footnotes");
        options.extension.autolink = activated_extensions.contains(&"autolink");
        options.extension.table = activated_extensions.contains(&"table");
        options.extension.tagfilter = activated_extensions.contains(&"tagfilter");
        options.extension.tasklist = activated_extensions.contains(&"tasklist");
        options.extension.description_lists = activated_extensions.contains(&"description_lists");
        if activated_extensions.contains(&"header_ids") {
            options.extension.header_ids = Some("".to_owned());
        }
    } else {
        options.extension.strikethrough = true;
        options.extension.superscript = true;
        options.extension.footnotes = true;
        options.extension.autolink = true;
        options.extension.table = true;
        options.extension.tagfilter = true;
        options.extension.tasklist = true;
        options.extension.description_lists = true;
        options.extension.header_ids = Some("".to_owned());
    };

    options
}
