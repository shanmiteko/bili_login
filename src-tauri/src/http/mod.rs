mod send;

use send::HttpRequest;

/// GET
pub async fn get<K, V>(url: &str, query: Option<Vec<(K, V)>>) -> HttpRequest
where
    K: Into<String>,
    V: Into<String>,
{
    HttpRequest::make("GET", url, query, None).await
}

/// POST
pub async fn post<K, V>(
    url: &str,
    query: Option<Vec<(K, V)>>,
    body: Option<Vec<(K, V)>>,
) -> HttpRequest
where
    K: Into<String>,
    V: Into<String>,
{
    HttpRequest::make("POST", url, query, body).await
}

#[cfg(test)]
mod tests {
    use super::{get, post};

    #[tokio::test]
    async fn get_test() {
        match get("https://httpbin.org/get", Some(vec![("foo", "bar")]))
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
            Some(vec![("foo".to_string(), "bar")]),
            Some(vec![("foo".to_string(), "bar")]),
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
