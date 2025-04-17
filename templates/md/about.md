# Hello from markdown!

## This is a h2


### This is a h3!


`echo this is a codeblock!`
`echo idk?`

----------

```rust
fn rename_dungeon(id: i32, mapping: &HashMap<i32, String>) -> String {
    mapping.get(&id).cloned().unwrap_or_else(|| format!("Unknown Dungeon (ID: {})", id))
}

// let user type Coolname instead of Cöölname (no need to type alt codes)
fn check_alt_codes(name: &str) -> String {
    name.to_lowercase()
        .replace("ö", "o")
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}
```
----------

