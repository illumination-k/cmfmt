---
name: test
---
# Test

## TL;DR

**other**

```rust:title
fn parse_options<'a>(options: &'a ArgMatches) -> CoolToolOptions<'a> {
    let a = 1;
    CoolToolOptionsBuilder::default()
        .expect(options.values_of("expect").map(|x| x.collect::<Vec<_>>().join(",")),)
        .cmd_prompt(options.values_of("cmd-prompt").and_then(|vals| vals.last()))
        .multi(if options.is_present("no-multi") {
            false
        } else {
            options.is_present("multi")
        })
        .bind(options.values_of("bind").map(|x| x.collect::<Vec<_>>()).unwrap_or_default(),)
        .build()
        .unwrap()
}
```

````python
python code!
````

~iii~

`code`

[url](example)