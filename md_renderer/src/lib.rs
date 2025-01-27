use std::{error::Error, sync::Arc};

use base64::Engine;
use headless_chrome::{protocol::cdp::Page, LaunchOptions};
use headless_chrome::{Browser, Tab};

static HTML: &'static str = include_str!("../assets/index.html");

struct Renderer {
    browser: Browser,
    tab: Arc<Tab>,
}

impl Renderer {
    fn new() -> Result<Self, Box<dyn Error>> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;
        tab.set_default_timeout(std::time::Duration::from_secs(10));
        tab.navigate_to(&("data:text/html,".to_string() + HTML))?;
        tab.wait_until_navigated()?;
        tab.wait_for_element("body")?;
        tab.set_default_timeout(std::time::Duration::from_secs(10));
        Ok(Self { browser, tab })
    }

    fn render(&self, md: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let b64 = base64::engine::general_purpose::STANDARD.encode(md);
        let js = format!(
            r#"
            window.renderMarkdown(atob("{}"));
            "#,
            b64
        );
        self.tab.evaluate(&js, true)?;
        let content = self.tab.wait_for_element("#content.rendering-finished")?;
        println!("content: {:?}", content.get_content());
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
        let data = renderer.render("# Hello, world!").unwrap();
        assert!(!data.is_empty());
        std::fs::write("test.png", &data).unwrap();
    }
}
