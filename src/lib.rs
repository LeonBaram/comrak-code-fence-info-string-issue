use comrak::{Arena, format_commonmark, parse_document};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn transform(input: String) -> Result<String> {
    let arena = Arena::new();
    let root = parse_document(&arena, &input, &comrak::Options::default());

    let mut output = String::new();
    format_commonmark(root, &comrak::Options::default(), &mut output)?;
    Ok(output)
}

#[test]
fn regular_input() {
    let input = r#"
just some regular input
"#
    .trim_start();
    let output = r#"
just some regular input
"#
    .trim_start();

    let actual = transform(input.to_string()).unwrap();
    assert_eq!(actual, output);
}

#[test]
fn code_fence_spacing() {
    let input = r#"
```lua calcmd
function myFunc(x, y)
    local ans = 0
    for i=x,y,1 do
        ans = ans + i
    end
    return ans
end
```
"#
    .trim_start();
    let output = r#"
```lua calcmd
function myFunc(x, y)
    local ans = 0
    for i=x,y,1 do
        ans = ans + i
    end
    return ans
end
```
"#
    .trim_start();

    let actual = transform(input.to_string()).unwrap();
    assert_eq!(actual, output);
}
