use crate::settings::{Lang, Settings};
use anyhow::Result;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

pub fn write_string<P: AsRef<Path>>(p: &P, string: &String) -> Result<()> {
    let file = File::create(&p).expect("File create error!");
    let mut w = BufWriter::new(&file);
    write!(w, "{}", string)?;
    w.flush()?;
    Ok(())
}

pub fn read_string<P: AsRef<Path>>(p: &P) -> Result<String> {
    let mut buf = String::new();
    let mut file = File::open(p).map(|f| BufReader::new(f))?;

    file.read_to_string(&mut buf)?;
    Ok(buf)
}

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

pub fn parse_codetitle<S: ToString>(s: &S) -> (String, Option<String>) {
    let s = s.to_string();
    let v: Vec<&str> = s.split(":").collect();
    let language = v[0].to_string();
    let title = match v.get(1) {
        Some(s) => Some(s.to_string()),
        None => None,
    };
    (language, title)
}

pub fn detect_lang(lang_name: &String, settings: &Settings) -> Option<Lang> {
    let lang_name = lang_name.to_lowercase();
    let mut lang = None;

    for l in settings.fmt.values() {
        if l.contain_language_name(&lang_name) {
            lang = Some(l.clone())
        }
    }

    lang
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
