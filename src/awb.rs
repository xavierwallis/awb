use eoka::{Browser, Page, Result };
use tokio::sync::OnceCell;

pub struct AutonomousWebBrowser {
    browser: Browser,
    page: Page,
}

pub static BROWSER: OnceCell<AutonomousWebBrowser> = OnceCell::const_new();

impl AutonomousWebBrowser {
    pub async fn instance() -> Result<&'static AutonomousWebBrowser> {
        BROWSER.get_or_try_init(|| async { AutonomousWebBrowser::new().await }).await
    }

    pub async fn browser() -> Result<&'static Browser> {
        Ok(&Self::instance().await?.browser)
    }

    pub async fn page() -> Result<&'static Page> {
        Ok(&Self::instance().await?.page)
    }

    pub async fn new() -> Result<AutonomousWebBrowser> {
        let browser = Browser::launch().await?;
        let page = browser.new_blank_page().await?;
        Ok(AutonomousWebBrowser { browser, page })
    }

    pub async fn get_page_metadata() -> Result<String> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.title().await
    }

    pub async fn get_element_content(selector: &str) -> Result<String> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        let element: eoka::Element = page.find(selector).await?;
        element.value().await 
    }

    pub async fn goto(url: &str) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.goto( url ).await
    }

    pub async fn click(selector: &str) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.click( selector ).await
    }

    pub async fn wait_for(selector: &str) -> Result<eoka::Element<'_>> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.wait_for(selector, 3000 ).await
    }

    pub async fn send_keys(selector: &str, keys: &str) -> Result<()> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.human_fill( selector, keys ).await 
    }

    pub async fn screenshot() -> Result<Vec<u8>> {
        let page: &Page = AutonomousWebBrowser::page().await?;
        page.screenshot().await
    }
}
