use std::collections::HashMap;

use lazy_static::lazy_static;
use tauri::{
    api::{
        http::{
            header::HeaderMap, Body, Client, ClientBuilder, FormBody, FormPart, HttpRequestBuilder,
            ResponseData, ResponseType,
        },
        Error,
    },
    http::header::USER_AGENT,
};

lazy_static! {
    static ref CLIENT: Client = ClientBuilder::new().build().unwrap();
    static ref HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0"
                .parse()
                .unwrap(),
        );
        headers
    };
}

pub struct HttpRequest(HttpRequestBuilder);

impl HttpRequest {
    /// 生成Http(s)请求
    pub async fn make(
        method: &str,
        url: &str,
        query: Option<Vec<(String, String)>>,
        body: Option<Vec<(String, String)>>,
    ) -> Self {
        let mut request = HttpRequestBuilder::new(method, url)
            .unwrap()
            .headers(HEADERS.clone());

        if let Some(query_vec) = query {
            request = request.query(query_vec.into_iter().collect::<HashMap<String, String>>())
        }

        if let Some(body_vec) = body {
            request = request.body(Body::Form(FormBody::new(
                body_vec
                    .into_iter()
                    .map(|(k, v)| (k, FormPart::Text(v)))
                    .collect::<HashMap<String, FormPart>>(),
            )))
        }

        Self(request)
    }

    /// 接收JSON格式数据
    pub async fn json(self) -> Result<ResponseData, Error> {
        CLIENT
            .send(self.0.response_type(ResponseType::Json))
            .await?
            .read()
            .await
    }
}
