use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

pub struct BrowserSingleton {
    browser: Browser,
    tab: Arc<Tab>,
}

pub static SINGLETON: OnceLock<BrowserSingleton> = OnceLock::new();

impl BrowserSingleton {
    pub fn browser() -> &'static Browser {
        &SINGLETON.get_or_init(BrowserSingleton::initialize).browser
    }

    pub fn tab() -> &'static Arc<Tab> {
        &SINGLETON.get_or_init(BrowserSingleton::initialize).tab
    }

    pub fn initialize() -> BrowserSingleton {
        let browser = Browser::new(
            LaunchOptionsBuilder::default()
                //.path(Some(PathBuf::from("/usr/bin/chromium")))
                .headless(true)
                .window_size(Some((1280, 800)))
                .port(Some(9222))
                //.extensions(vec![OsStr::new("/app/selenium-extensions/Vimium")])
                .sandbox(false)
                .build()
                .unwrap(),
        )
        .expect("Could Not Start Browser");

        let tab = browser.new_tab().expect("Could Not Open Tab");

        BrowserSingleton { browser, tab }
    }

    pub fn get_content() -> Result<String, anyhow::Error> {
        BrowserSingleton::tab().get_content()
    }

    pub fn get_element_content(selector: &str) -> Result<String, anyhow::Error> {
        BrowserSingleton::tab()
            .find_element(selector)?
            .get_content()
    }

    pub fn goto(url: &str) -> Result<(), anyhow::Error> {
        BrowserSingleton::tab().navigate_to(url)?;
        Ok(())
    }

    pub fn click(selector: &str) -> Result<(), anyhow::Error> {
        BrowserSingleton::tab()
            .wait_for_element(selector)?
            .click()?;
        Ok(())
    }

    pub fn wait_for(selector: &str) -> Result<headless_chrome::Element<'_>, anyhow::Error> {
        BrowserSingleton::tab().wait_for_element(selector)
    }

    pub fn send_keys(keys: &str) -> Result<(), anyhow::Error> {
        BrowserSingleton::tab().send_character(keys)?;
        Ok(())
    }

    pub fn screenshot() -> Result<Vec<u8>, anyhow::Error> {
        BrowserSingleton::tab().capture_screenshot(
            CaptureScreenshotFormatOption::Png,
            None,
            None,
            true,
        )
    }
}
