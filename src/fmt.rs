use anyhow::Result;

use crate::settings::{Lang, Settings};
use crate::utils::{
    detect_lang, parse_codetitle, read_string, split_frontmatter_and_content, write_string,
};

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use std::process::{Command, Stdio};

fn fmtcommand(lang: Lang, file_path: &String) -> Result<()> {
    let mut args = vec![file_path.to_owned()];
    match lang.args() {
        Some(a) => args.extend_from_slice(&a),
        None => {}
    }
    let child = Command::new(lang.command())
        .args(&args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let output = child.wait_with_output()?;
    eprintln!("Run Command: {} {}", lang.command(), args.join(" "));
    eprintln!("stderr: {}", std::str::from_utf8(&output.stderr)?);
    eprintln!("stdout: {}\n", std::str::from_utf8(&output.stdout)?);
    Ok(())
}

fn fmt_code(code: &String, lang: Lang, title: Option<String>) -> Result<String> {
    let filename = match title {
        Some(t) => t,
        None => match &lang.ext() {
            Some(ext) => format!("tmp_code{}", ext),
            None => "tmp_code".to_string(),
        },
    };

    let dir = tempfile::tempdir().expect("tmp dir error");
    let file_path = dir.path().join(filename);
    write_string(&file_path, code)?;
    fmtcommand(lang, &file_path.as_path().to_str().unwrap().to_owned())?;

    let new_code = read_string(&file_path)?;
    Ok(new_code)
}

pub fn fmt(text: &String, settings: &Settings) -> Result<String> {
    let (frontmatter, content) = split_frontmatter_and_content(text);
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
                    let (lang_name, title) = parse_codetitle(&now_codetitle);
                    let lang = detect_lang(&lang_name, settings);

                    let fmt_code = match lang {
                        Some(l) => fmt_code(&code, l, title)?,
                        None => code,
                    };
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

    cmark(events.iter(), &mut buf, None)?;
    Ok(buf.replace("````", "```"))
}
