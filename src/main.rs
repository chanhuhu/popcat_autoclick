use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use thirtyfour_sync::prelude::*;

fn main() -> Result<()> {
    let mut caps = DesiredCapabilities::chrome();
    //caps.set_headless()?;
    caps.add_chrome_arg("--window-size=300,300")?;

    let driver = WebDriver::new("http://localhost:4444", &caps)?;
    driver.get("https://popcat.click")?;

    let cat_img: WebElement = driver.find_element(By::ClassName("cat-img"))?;
    cat_img.wait_until();

    loop {
        cat_img.click()?;
        cat_img.click()?;
        cat_img.click()?;
        cat_img.click()?;
        cat_img.click()?;
        sleep(Duration::from_millis(250));
    }
}
