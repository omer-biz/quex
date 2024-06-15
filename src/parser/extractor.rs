use pulldown_cmark::CodeBlockKind::Fenced;
use pulldown_cmark::CowStr::Borrowed;
use pulldown_cmark::Event::{Start, Text};
use pulldown_cmark::Parser;
use pulldown_cmark::Tag::CodeBlock;

pub type RawQuex<'a> = Vec<&'a str>;

pub fn extract_quex(markdown_input: &str) -> RawQuex {
    let mut raw_quex = vec![];

    let mut events = Parser::new(markdown_input);

    while let Some(event) = events.next() {
        if let Start(CodeBlock(Fenced(code_block))) = event {
            if code_block.as_bytes() == [113, 117, 101, 120] {
                if let Some(Text(Borrowed(text))) = events.next() {
                    raw_quex.push(text);
                }
            }
        }
    }

    raw_quex
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extract_quex() {
        let markdown_input = r#"
# Sample Input

This is a sample input.

```quex
2024 mar 1, sample description.
d=5, recurring monthly
1992* feb 29, named recurring yearly
* jan 1, recurring yearly
```

This should be ignored as well.
"#;
        let quex_input = super::extract_quex(markdown_input);
        assert_eq!(quex_input, vec!["2024 mar 1, sample description.\nd=5, recurring monthly\n1992* feb 29, recurring yearly\n"]);
    }
}
