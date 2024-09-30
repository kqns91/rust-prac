use std::panic;
use thirtyfour::{DesiredCapabilities, WebDriver};

#[cfg(test)]
#[allow(non_snake_case)]
mod E2Eテスト {
    use super::*;

    #[tokio::test]
    async fn projectsに対してpathを指定するとそのページの情報を取得できる() {
        let url = "http://server:8080/technologies/aws";

        // 1
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://selenium-chrome:4444", caps)
            .await
            .expect("failed to create driver");

        // 2
        if let Err(e) = driver.goto(url).await {
            driver.quit().await.expect("quit error due to goto failure");
            panic!("goto error: {:?}", e);
        }

        // 3
        let html = match driver.source().await {
            Ok(content) => content,
            Err(e) => {
                driver
                    .quit()
                    .await
                    .expect("quit error due to source failure");
                panic!("source error: {:?}", e);
            }
        };

        // 4
        driver.quit().await.expect("quit error");

        // 5
        assert!(html.contains("AWS"));
        assert!(html.contains("ProjectA"));
    }
}
