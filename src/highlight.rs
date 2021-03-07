use ammonia::Builder;
use maplit::{hashmap, hashset};
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

// add code highlighting
// this function is only applied to fenced code blocks with a *language token*
// otherwise there is no way to evaluate the syntax required for
fn highlight(source: &String, language: &String) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];
    let sr = ss
        .find_syntax_by_token(&language)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    highlighted_html_for_string(&source, &ss, sr, theme)
}

// parse html
pub fn parse_html(markdown: String, options: Options) -> String {
    // indicator if next block needs to syntax highlighted
    let mut code_inidicator = false;
    let mut code = String::new(); // contain all code for one block in one string to only highlight once per block
    let mut language = String::new(); // container for language token in fenced code block

    // Create a new vector of events since we can only consume the parser once
    let mut highlighted_html = Vec::new();

    // Set up options and parser
    Parser::new_ext(&markdown, options).for_each(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            // set values to catch following text blocks
            language = lang.clone().into_string(); // this is required to find the language for syntax highlighting later
            code_inidicator = true;
            highlighted_html.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))));
        }
        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            highlighted_html.push(Event::Html(CowStr::from(highlight(&code, &language))));
            highlighted_html.push(Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(lang))));

            // reset values
            code_inidicator = false;
            code = String::new();
        }
        Event::Text(text) => {
            if code_inidicator {
                code.push_str(&text)
            } else {
                highlighted_html.push(Event::Text(text));
            }
        }
        Event::TaskListMarker(status) => {
            // use boostrap 5 checkboxes instead of the ugly default ones
            if status {
                highlighted_html.push(Event::Html(CowStr::from(CHECKBOX_TOGGLED)));
            } else {
                highlighted_html.push(Event::Html(CowStr::from(CHECKBOX)));
            }
        }
        event => highlighted_html.push(event),
    });

    // Write to String buffer.
    let mut html_output: String = String::new();
    html::push_html(&mut html_output, highlighted_html.into_iter());

    // clean html
    Builder::default()
        .add_generic_attributes(&["style", "type", "checked"])
        .add_tags(&["input"])
        .allowed_classes(
            hashmap!["input" => hashset!["form-check-input"], "div" => hashset!["form-check"]],
        )
        .clean(&html_output)
        .to_string()
}

// checkbox varaints
// kept here for readability
static CHECKBOX: &'static str =
    "<div class='form-check'><input class='form-check-input' type='checkbox' value=''></div>";
static CHECKBOX_TOGGLED: &'static str = "<div class='form-check'><input class='form-check-input' type='checkbox' value='' checked></div>";
