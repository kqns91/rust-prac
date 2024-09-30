use webapi_mvp; // tests ディレクトリ内から src ディレクトリ内のモジュールを参照するために必要

#[cfg(test)]
#[allow(non_snake_case)]
mod integration_API_DBテスト {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn get_technology_page関数は指定したpathを元にDBから情報を取得する() {
        let path = "/technologies/aws";
        let tech_name = "AWS";

        // テスト用サーバーの立ち上げ
        let service = App::new().service(webapi_mvp::apis::get_technology_page);
        let app = test::init_service(service).await;

        // テスト用のリクエストを生成してレスポンスを取得
        let req = test::TestRequest::get().uri(path).to_request();
        let resp = test::call_service(&app, req).await;

        // 取得した値をパースして確認
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.contains(tech_name));
    }
}
