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
use std::{fs::{self, File}, io::{BufWriter, Write}, process::{Command, Stdio}};
use tempfile;

use pulldown_cmark_to_cmark::cmark;

mod settings;

use crate::settings::{Lang, Settings};

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


fn detect_lang(lang_name: &String, settings: &Settings) -> Option<Lang> {
    let lang_name = lang_name.to_lowercase();
    let mut lang = None;

    for l in settings.fmt.values() {
        if l.contain_language_name(&lang_name) {
            lang = Some(l.clone())
        }
    }

    lang
}

fn fmtcommand(lang: Lang, file_path: &String) -> Result<()> {
    let mut args = vec![file_path.to_owned()];
    match lang.args() {
        Some(a) => args.extend_from_slice(&a),
        None => {}
    }
    let child = Command::new(lang.command()).args(&args).stderr(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    let output = child.wait_with_output()?;
    eprintln!("Run Command: {} {}", lang.command(), args.join(" "));
    eprintln!("stderr: {}", std::str::from_utf8(&output.stderr)?);
    eprintln!("stdout: {}\n", std::str::from_utf8(&output.stdout)?);
    Ok(())
}

fn fmt_code(code: &String, lang: Lang) -> Result<String> {
    let dir = tempfile::tempdir().expect("tmp dir error");
    let file_path = dir.path().join("tmp");
    let file = File::create(&file_path).expect("err create file");
    let mut w = BufWriter::new(&file);
    write!(w, "{}", code)?;
    w.flush()?;
    fmtcommand(lang, &file_path.as_path().to_str().unwrap().to_owned())?;

    let new_code = fs::read_to_string(&file_path)?;
    Ok(new_code)
}

fn main() -> Result<()> {
    let text = fs::read_to_string("test/raw_markdown.md")?;
    let (frontmatter, content) = split_frontmatter_and_content(&text);
    let options = Options::empty();
    let parser = Parser::new_ext(&content, options);

    let mut now_range = 0..0;
    let mut now_codetitle = "".to_string();

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
                    let code = s.to_string();
                    let (lang_name, _) = parse_codetitle(&now_codetitle);
                    let lang = match detect_lang(&lang_name, &Settings::default()) {
                        Some(l) => l,
                        None => {continue;}
                    };

                    let fmt_code = fmt_code(&code, lang)?;
                    Event::Text(fmt_code.into())
                } else {
                    Event::Text(s.to_owned())
                }
            }
            _ => e.clone(),
        };

        events.push(new_e);
    }

    let mut buf = match frontmatter {
        Some(s) => s.to_string(),
        None => String::new(),
    };

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
