use eoka::{Browser, Page, Result, StealthConfig};
use tokio::sync::OnceCell;

pub struct AutonomousWebBrowser {
    browser: Browser,
    page: Page,
}

pub static BROWSER: OnceCell<AutonomousWebBrowser> = OnceCell::const_new();

impl AutonomousWebBrowser {
    pub async fn instance() -> Result<&'static AutonomousWebBrowser> {
        BROWSER
            .get_or_try_init(|| async { AutonomousWebBrowser::new().await })
            .await
    }

    pub async fn browser() -> Result<&'static Browser> {
        Ok(&Self::instance().await?.browser)
    }

    pub async fn page() -> Result<&'static Page> {
        Ok(&Self::instance().await?.page)
    }

    pub async fn new() -> Result<AutonomousWebBrowser> {
        let config = StealthConfig {
            headless: false,
            ..Default::default()
        };
        let browser = Browser::launch_with_config(config).await?;
        let page = browser.new_blank_page().await?;
        Ok(AutonomousWebBrowser { browser, page })
    }

    pub async fn page_goto(url: &str) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.goto(url).await
    }

    pub async fn page_title() -> Result<String> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.title().await
    }

    pub async fn page_url() -> Result<String> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.url().await
    }

    pub async fn page_content() -> Result<String> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.content().await
    }

    pub async fn page_screenshot() -> Result<Vec<u8>> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.screenshot().await
    }

    // find

    pub async fn find(selector: &str) -> Result<String> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        let element: eoka::Element = page.find(selector).await?;
        element.value().await
    }

    pub async fn find_all(selector: &str) -> Result<Vec<eoka::Element>> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.find_all(selector).await
    }

    pub async fn find_text(text: &str) -> Result<eoka::Element> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.find_by_text(text).await
    }

    pub async fn find_all_text(selector: &str) -> Result<Vec<eoka::Element>> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.find_all_by_text(selector).await
    }

    pub async fn element_text(selector: &str) -> Result<String> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        let element: eoka::Element = page.find(selector).await?;
        element.text().await
    }

    pub async fn element_exists(selector: &str) -> Result<bool> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        Ok(page.find(selector).await.is_ok())
    }

    // input

    pub async fn click(selector: &str) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.human_click(selector).await
    }

    pub async fn click_text(text: &str) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.human_click_by_text(text).await
    }

    pub async fn fill(selector: &str, text: &str) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.human_fill(selector, text).await
    }

    pub async fn input_keys(selector: &str, keys: &str) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.human_type(selector, keys).await
    }

    pub async fn input_key(key: &str) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.press_key(key).await
    }

    pub async fn hover(selector: &str) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.hover(selector).await
    }

    // wait

    pub async fn wait_for(selector: &str, timeout_ms: u64) -> Result<eoka::Element<'_>> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.wait_for(selector, timeout_ms).await
    }
    pub async fn wait_for_hidden(selector: &str, timeout_ms: u64) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.wait_for_hidden(selector, timeout_ms).await
    }
    pub async fn wait_for_visible(selector: &str, timeout_ms: u64) -> Result<eoka::Element<'_>> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.wait_for_visible(selector, timeout_ms).await
    }
    pub async fn wait_for_text(text: &str, timeout_ms: u64) -> Result<eoka::Element<'_>> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.wait_for_text(text, timeout_ms).await
    }
    pub async fn wait_for_url_contains(pattern: &str, timeout_ms: u64) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.wait_for_url_contains(pattern, timeout_ms).await
    }
    pub async fn wait_for_url_change(timeout_ms: u64) -> Result<String> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.wait_for_url_change(timeout_ms).await
    }
    pub async fn wait_for_network_idle(idle_time_ms: u64, timeout_ms: u64) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.wait_for_network_idle(idle_time_ms, timeout_ms).await
    }
    pub async fn wait(timeout_ms: u64) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        Ok(page.wait(timeout_ms).await)
    }
}
