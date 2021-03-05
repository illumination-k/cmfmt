# cmfmt

**C**ode in **M**arkdown **F**or**M**a**T**ter

A tool to automatically format the code in the markdown by external tools such as rustfmt, black, and prettier. The mechanism is simple: parse the markdown once, generate the code as a temporary file, and format it using an external tool. The end result is a markdown with the code formatted.
## Usage

```bash
USAGE:
    cmfmt [FLAGS] [OPTIONS] <markdown>

FLAGS:
    -h, --help       Prints help information
        --stdout     Output formatted markdown to stdout instead of overwrite the input markdown
    -V, --version    Prints version information

OPTIONS:
        --config <config>    Path of the config file. default: ${home}/.config/cmfmt.toml

ARGS:
    <markdown>    Path of the input markdown you would like to format
```

## Settings

You can set the command and args for each language.

This tool detects what language is used in the pre-code block by fence. For example, following code is recoganaized as Python. 

````
```python
print("Hello World")
```
````

You can set which words are recognized as which language by setting the name.

Also, this tool can set extension used when generating a tmp file. This is useful for the tool that work with specific extensions such as prettier.


### Default Settings

When you first run this tool, following toml file is generated in `~/.config/cmfmt.toml`. You can set your own settings by rewriting this file.

```toml
[fmt.python]
command = "black"
name = ["py", "python", "python3"]
extention = "py"

[fmt.rust]
command = "rustfmt"
name = ["rs", "rust"]
extention = "rs"

[fmt.js]
command = "prettier"
args = ["--write"]
name = ["js", "ts", "javascript", "typescript"]
extention = "js"
```

## Other Features

- Allow frontmatter

## LICENSE

MIT