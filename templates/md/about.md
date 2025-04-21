# :warning: About Page is Work-in-progress :warning:

---

```bash
echo "hello!"
printf "hello!\n"
```

---

# This webpage was built with Rust! :crab:
### an example of how resume is formated to the website.

```rust
async fn resume_page() -> Html<String> {
    let md = &RESUME_MD;
    let mut options = ComrakOptions::default();
    crate::mdext::enable_extensions(&mut options);
    let html = markdown_to_html(md, &options);
    let template = ContentTemplate { content: html };
    Html(template.render().unwrap())
}
```
---
