use std::sync::{Arc, OnceLock};

use eoka::{Browser, Page, Result, StealthConfig};
use tokio::sync::Mutex;

pub struct AutonomousWebBrowser {
    _browser: Browser,
    page: Page,
}

pub static BROWSER: OnceLock<Mutex<Option<Arc<AutonomousWebBrowser>>>> = OnceLock::new();

fn get_lock() -> &'static Mutex<Option<Arc<AutonomousWebBrowser>>> {
    BROWSER.get_or_init(|| Mutex::new(None))
}

fn is_fatal(err: &eoka::Error) -> bool {
    matches!(err, eoka::Error::Transport { .. } | eoka::Error::Launch(_))
}

impl AutonomousWebBrowser {
    pub async fn new() -> Result<AutonomousWebBrowser> {
        let config = StealthConfig {
            headless: true,
            ..Default::default()
        };
        let browser = Browser::launch_with_config(config).await?;
        let page = browser.new_blank_page().await?;
        Ok(AutonomousWebBrowser {
            _browser: browser,
            page,
        })
    }

    pub async fn instance() -> Result<Arc<AutonomousWebBrowser>> {
        let mut guard = get_lock().lock().await;
        if let Some(ref awb) = *guard {
            return Ok(Arc::clone(awb));
        }
        tracing::info!("Browser not initialised — starting Chrome...");
        let awb = Arc::new(AutonomousWebBrowser::new().await?);
        *guard = Some(Arc::clone(&awb));
        tracing::info!("Browser ready");
        Ok(awb)
    }

    async fn reset_and_reinit() -> Result<Arc<AutonomousWebBrowser>> {
        let mut guard = get_lock().lock().await;
        tracing::warn!("Dropping dead browser instance and restarting Chrome...");
        *guard = None;
        let awb = Arc::new(AutonomousWebBrowser::new().await.map_err(|e| {
            tracing::error!("Browser reinit failed: {}", e);
            e
        })?);
        *guard = Some(Arc::clone(&awb));
        tracing::info!("Browser restarted successfully");
        Ok(awb)
    }

    // page

    pub async fn page_goto(url: &str) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.goto(url).await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on page_goto: {}", err);
                drop(awb);
                Self::reset_and_reinit().await?.page.goto(url).await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn page_title() -> Result<String> {
        let awb = Self::instance().await?;
        match awb.page.title().await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on page_title: {}", err);
                drop(awb);
                Self::reset_and_reinit().await?.page.title().await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn page_url() -> Result<String> {
        let awb = Self::instance().await?;
        match awb.page.url().await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on page_url: {}", err);
                drop(awb);
                Self::reset_and_reinit().await?.page.url().await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn page_content() -> Result<String> {
        let awb = Self::instance().await?;
        match awb.page.content().await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on page_content: {}", err);
                drop(awb);
                Self::reset_and_reinit().await?.page.content().await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn page_screenshot() -> Result<Vec<u8>> {
        let awb = Self::instance().await?;
        match awb.page.screenshot().await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on page_screenshot: {}", err);
                drop(awb);
                Self::reset_and_reinit().await?.page.screenshot().await
            }
            Err(err) => Err(err),
        }
    }

    // find

    pub async fn find(selector: &str) -> Result<String> {
        let awb = Self::instance().await?;
        match awb.page.find(selector).await {
            Ok(element) => element.value().await,
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on find: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .find(selector)
                    .await?
                    .value()
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn find_all(selector: &str) -> Result<Vec<String>> {
        let awb = Self::instance().await?;
        match awb.page.find_all(selector).await {
            Ok(elements) => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(element.value().await?);
                }
                Ok(values)
            }
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on find_all: {}", err);
                drop(awb);
                let awb = Self::reset_and_reinit().await?;
                let elements = awb.page.find_all(selector).await?;
                let mut values = Vec::new();
                for element in elements {
                    values.push(element.value().await?);
                }
                Ok(values)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn find_text(text: &str) -> Result<String> {
        let awb = Self::instance().await?;
        match awb.page.find_by_text(text).await {
            Ok(element) => element.text().await,
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on find_text: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .find_by_text(text)
                    .await?
                    .text()
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn find_all_text(selector: &str) -> Result<Vec<String>> {
        let awb = Self::instance().await?;
        match awb.page.find_all_by_text(selector).await {
            Ok(elements) => {
                let mut texts = Vec::new();
                for element in elements {
                    texts.push(element.text().await?);
                }
                Ok(texts)
            }
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on find_all_text: {}", err);
                drop(awb);
                let awb = Self::reset_and_reinit().await?;
                let elements = awb.page.find_all_by_text(selector).await?;
                let mut texts = Vec::new();
                for element in elements {
                    texts.push(element.text().await?);
                }
                Ok(texts)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn element_text(selector: &str) -> Result<String> {
        let awb = Self::instance().await?;
        match awb.page.find(selector).await {
            Ok(element) => element.text().await,
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on element_text: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .find(selector)
                    .await?
                    .text()
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn element_exists(selector: &str) -> Result<bool> {
        let awb = Self::instance().await?;
        match awb.page.find(selector).await {
            Ok(_) => Ok(true),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on element_exists: {}", err);
                drop(awb);
                match Self::reset_and_reinit().await?.page.find(selector).await {
                    Ok(_) => Ok(true),
                    Err(eoka::Error::ElementNotFound(_)) => Ok(false),
                    Err(err) => Err(err),
                }
            }
            Err(eoka::Error::ElementNotFound(_)) => Ok(false),
            Err(err) => Err(err),
        }
    }

    // input

    pub async fn click(selector: &str) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.human_click(selector).await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on click: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .human_click(selector)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn click_text(text: &str) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.human_click_by_text(text).await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on click_text: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .human_click_by_text(text)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn fill(selector: &str, text: &str) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.human_fill(selector, text).await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on fill: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .human_fill(selector, text)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn input_keys(selector: &str, keys: &str) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.human_type(selector, keys).await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on input_keys: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .human_type(selector, keys)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn input_key(key: &str) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.press_key(key).await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on input_key: {}", err);
                drop(awb);
                Self::reset_and_reinit().await?.page.press_key(key).await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn hover(selector: &str) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.hover(selector).await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on hover: {}", err);
                drop(awb);
                Self::reset_and_reinit().await?.page.hover(selector).await
            }
            Err(err) => Err(err),
        }
    }

    // wait

    #[allow(dead_code)]
    pub async fn wait_for(selector: &str, timeout_ms: u64) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.wait_for(selector, timeout_ms).await {
            Ok(_) => Ok(()),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on wait_for: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .wait_for(selector, timeout_ms)
                    .await
                    .map(|_| ())
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn wait_for_hidden(selector: &str, timeout_ms: u64) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.wait_for_hidden(selector, timeout_ms).await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on wait_for_hidden: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .wait_for_hidden(selector, timeout_ms)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn wait_for_visible(selector: &str, timeout_ms: u64) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.wait_for_visible(selector, timeout_ms).await {
            Ok(_) => Ok(()),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on wait_for_visible: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .wait_for_visible(selector, timeout_ms)
                    .await
                    .map(|_| ())
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn wait_for_text(text: &str, timeout_ms: u64) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.wait_for_text(text, timeout_ms).await {
            Ok(_) => Ok(()),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on wait_for_text: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .wait_for_text(text, timeout_ms)
                    .await
                    .map(|_| ())
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn wait_for_url_contains(pattern: &str, timeout_ms: u64) -> Result<()> {
        let awb = Self::instance().await?;
        match awb.page.wait_for_url_contains(pattern, timeout_ms).await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on wait_for_url_contains: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .wait_for_url_contains(pattern, timeout_ms)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn wait_for_url_change(timeout_ms: u64) -> Result<String> {
        let awb = Self::instance().await?;
        match awb.page.wait_for_url_change(timeout_ms).await {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on wait_for_url_change: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .wait_for_url_change(timeout_ms)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn wait_for_network_idle(idle_time_ms: u64, timeout_ms: u64) -> Result<()> {
        let awb = Self::instance().await?;
        match awb
            .page
            .wait_for_network_idle(idle_time_ms, timeout_ms)
            .await
        {
            Ok(v) => Ok(v),
            Err(err) if is_fatal(&err) => {
                tracing::warn!("Fatal browser error on wait_for_network_idle: {}", err);
                drop(awb);
                Self::reset_and_reinit()
                    .await?
                    .page
                    .wait_for_network_idle(idle_time_ms, timeout_ms)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn wait(timeout_ms: u64) -> Result<()> {
        let awb = Self::instance().await?;
        Ok(awb.page.wait(timeout_ms).await)
    }
}
