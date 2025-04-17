use comrak::ComrakOptions;

/// use in functions that call ComrakOptions::default()
/// keeps extensions same, less repeated code.
/// example:
/// ```rust
/// let mut options = ComrakOptions::default(); 
/// enable_extensions(&mut options);
/// ```
pub fn enable_extensions(options: &mut ComrakOptions) {
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.shortcodes = true;
    options.extension.tasklist = true;
}


