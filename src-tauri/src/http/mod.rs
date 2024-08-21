mod send;

use send::HttpRequest;

/// GET
pub async fn get(url: &str, query: Option<Vec<(String, String)>>) -> HttpRequest {
    HttpRequest::make("GET", url, query, None).await
}

/// POST
pub async fn post(
    url: &str,
    query: Option<Vec<(String, String)>>,
    body: Option<Vec<(String, String)>>,
) -> HttpRequest {
    HttpRequest::make("POST", url, query, body).await
}

#[cfg(test)]
mod tests {
    use super::{get, post};

    #[tokio::test]
    async fn get_test() {
        match get(
            "https://httpbin.org/get",
            Some(vec![("foo".to_string(), "bar".to_string())]),
        )
        .await
        .json()
        .await
        {
            Ok(resp) => {
                let resp_data = resp.data;
                let pdata = resp_data
                    .get("args")
                    .unwrap()
                    .get("foo")
                    .unwrap()
                    .as_str()
                    .unwrap();
                assert_eq!("bar", pdata)
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }

    #[tokio::test]
    async fn post_test() {
        match post(
            "https://httpbin.org/post",
            Some(vec![("foo".to_string(), "bar".to_string())]),
            Some(vec![("foo".to_string(), "bar".to_string())]),
        )
        .await
        .json()
        .await
        {
            Ok(resp) => {
                let resp_data = resp.data;
                let pdata = resp_data
                    .get("args")
                    .unwrap()
                    .get("foo")
                    .unwrap()
                    .as_str()
                    .unwrap();
                let bdata = resp_data
                    .get("form")
                    .unwrap()
                    .get("foo")
                    .unwrap()
                    .as_str()
                    .unwrap();
                assert_eq!("bar", pdata);
                assert_eq!("bar", bdata);
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
}
