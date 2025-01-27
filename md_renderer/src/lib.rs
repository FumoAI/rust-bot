use std::{error::Error, sync::Arc};

use base64::Engine;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, Tab};

static HTML: &'static str = include_str!("../assets/index.html");

pub struct Renderer {
    browser: Browser,
    tab: Arc<Tab>,
}

impl Renderer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;
        tab.set_default_timeout(std::time::Duration::from_secs(10));
        tab.navigate_to(&("data:text/html,".to_string() + HTML))?;
        tab.wait_until_navigated()?;
        tab.wait_for_element("body")?;
        tab.set_default_timeout(std::time::Duration::from_secs(10));
        Ok(Self { browser, tab })
    }

    pub fn render(&self, md: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let b64 = base64::engine::general_purpose::STANDARD.encode(md);
        let js = format!(
            r#"
            window.renderMarkdown(atob("{}"));
            "#,
            b64
        );
        self.tab.evaluate(&js, true)?;
        let content = self.tab.wait_for_element("#content.rendering-finished")?;
        let data = content.capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let renderer = Renderer::new().unwrap();
        let data = renderer
            .render(
                "
# Hello, World!

This is a **feature-rich** multi-line markdown example.

## Features

- **Bold text**
- *Italic text*
- [Links](https://www.example.com)
- Inline `code`
- Code blocks:
```rust
fn main() {
    println!(\"Hello, world!\");
}
```
- Blockquotes:
> This is a blockquote.

- Lists:
    1. First item
    2. Second item
    3. Third item

- Tables:

| Syntax | Description |
|--------|-------------|
| Header | Title       |
| Paragraph | Text     |

- Images:
![Alt text](https://www.example.com/image.jpg)

- Horizontal rules:

---

Enjoy rendering your markdown!
        ",
            )
            .unwrap();
        assert!(!data.is_empty());
        std::fs::write("test.png", &data).unwrap();
    }
}
