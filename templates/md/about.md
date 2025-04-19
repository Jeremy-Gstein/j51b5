# `echo $WHOAMI` 
## This is a h2

>>>
idk what this does?
`#here is 1 line`
```bash
echo "hello!"
printf "hello!\n"
```
>>>

# /usr/bin/bash
`echo this is a codeblock!`
# /usr/bin/bash
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

