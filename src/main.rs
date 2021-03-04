extern crate anyhow;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

extern crate toml;

#[cfg(test)]
#[macro_use]
extern crate maplit;

#[macro_use]
extern crate serde;

use anyhow::Result;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    process::Command,
};
use tempfile;

use pulldown_cmark_to_cmark::cmark;

mod settings;

pub fn find_frontmatter_block(text: &str) -> Option<(usize, usize)> {
    match text.starts_with("---\n") {
        true => {
            let slice_after_marker = &text[4..];
            let fm_end = match slice_after_marker.find("---\n") {
                Some(f) => f,
                None => return None,
            };

            Some((0, fm_end + 2 * 4))
        }
        false => None,
    }
}

pub fn split_frontmatter_and_content(text: &str) -> (Option<&str>, &str) {
    match find_frontmatter_block(text) {
        Some((fm_start, fm_end)) => (Some(&text[fm_start..fm_end]), &text[fm_end..]),
        None => (None, text),
    }
}

fn parse_codetitle<S: ToString>(s: &S) -> (String, String) {
    let s = s.to_string();
    let v: Vec<&str> = s.split(":").collect();
    let language = v[0].to_string();
    let title = match v.get(1) {
        Some(s) => s.to_string(),
        None => "".to_string(),
    };
    (language, title)
}

#[derive(Debug, Clone)]
struct PreCode {
    language: String,
    title: String,
    text: String,
}

impl PreCode {
    fn new<S: ToString>(code_title: &S, text: &S) -> Self {
        let (language, title) = parse_codetitle(code_title);
        Self {
            language,
            title,
            text: text.to_string(),
        }
    }
}

fn fmtcommand_build(language: &str) -> Command {
    let language = language.to_lowercase();
    if language == "rust" {
        Command::new("rustfmt")
    } else if language == "python" {
        Command::new("black")
    } else {
        Command::new("cat")
    }
}

fn main() -> Result<()> {
    let text = fs::read_to_string("test/raw_markdown.md")?;
    let (frontmatter, content) = split_frontmatter_and_content(&text);
    let options = Options::empty();
    let parser = Parser::new_ext(&content, options);

    let mut now_range = 0..0;
    let mut now_codetitle = "".to_string();
    let mut now_text = "".to_string();
    let mut codes = vec![];

    let mut events = vec![];
    for (e, r) in parser.into_offset_iter() {
        // dbg!(&e, &r);
        let new_e = match &e {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(s))) => {
                now_range = r;
                now_codetitle = s.to_string();
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(s.to_owned())))
            }
            Event::Text(s) => {
                let start = r.start;
                let end = r.end;
                if now_range.start <= start && end <= now_range.end {
                    let mut text = s.to_string();
                    now_text = text.clone();
                    let (language, _) = parse_codetitle(&now_codetitle);
                    dbg!(&language);
                    if language.to_lowercase() == "rust" {
                        let dir = tempfile::tempdir().expect("tmpdir error");
                        let file_path = dir.path().join("tmp.rs");
                        let file = File::create(&file_path).expect("err create file");
                        let mut w = BufWriter::new(&file);
                        dbg!(&text);
                        write!(w, "{}", text)?;
                        w.flush()?;
                        let status = Command::new("rustfmt")
                            .arg(file_path.to_str().unwrap())
                            .status()?;
                        println!("{}", status);
                        let new_text = fs::read_to_string(&file_path)?;
                        dbg!(&new_text);
                        text = new_text;
                    }
                    // make tmpfile and fmt by rustfmt, black and prelitter
                    Event::Text(text.into())
                } else {
                    Event::Text(s.to_owned())
                }
            }
            Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(s))) => {
                if r == now_range {
                    let code = PreCode::new(&now_codetitle, &now_text);
                    codes.push(code);
                }
                Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(s.to_owned())))
            }
            _ => e.clone(),
        };

        events.push(new_e);
    }

    let mut buf = match frontmatter {
        Some(s) => s.to_string(),
        None => String::new(),
    };

    // test codes by cargo test, pytest and node.js
    // run code selected by title

    cmark(events.iter(), &mut buf, None).unwrap();
    println!("{}", buf.replace("````", "```"));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_frontmatter() {
        let test_string = "---\ntitle: Valid Yaml Test\n---\nsomething that's not yaml";
        assert_eq!(find_frontmatter_block(test_string), Some((0, 31)));

        let (frontmatter, content) = split_frontmatter_and_content(test_string);
        assert_eq!(frontmatter.unwrap(), "---\ntitle: Valid Yaml Test\n---\n");
        assert_eq!(content, "something that's not yaml")
    }
}
