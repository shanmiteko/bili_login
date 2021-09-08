use std::collections::HashMap;

use lazy_static::lazy_static;
use tauri::api::{
    http::{
        Body, Client, ClientBuilder, FormBody, FormPart, HttpRequestBuilder, ResponseData,
        ResponseType,
    },
    Error,
};

lazy_static! {
    static ref CLIENT: Client = ClientBuilder::new().build().unwrap();
    static ref HEADERS: HashMap<String, String> = {
        let mut headers = HashMap::new();
        headers.insert(
            "user-agent".into(),
            "Mozilla/5.0 (X11; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0".into(),
        );
        headers
    };
}

pub struct HttpRequest(HttpRequestBuilder);

impl HttpRequest {
    /// 生成Http(s)请求
    pub async fn make<K, V>(
        method: &str,
        url: &str,
        query: Option<Vec<(K, V)>>,
        body: Option<Vec<(K, V)>>,
    ) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        let mut request = HttpRequestBuilder::new(method, url).headers(HEADERS.clone());

        if let Some(query_vec) = query {
            request = request.query(
                query_vec
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect::<HashMap<String, String>>(),
            )
        }

        if let Some(body_vec) = body {
            request = request.body(Body::Form(FormBody::new(
                body_vec
                    .into_iter()
                    .map(|(k, v)| (k.into(), FormPart::Text(v.into())))
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
