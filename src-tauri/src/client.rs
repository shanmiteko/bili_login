use std::fmt::Display;

use crate::http::{get, post};

pub enum Error {
    TauriError(String),
    KeyNotFind(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TauriError(msg) => write!(f, "{}", msg),
            Error::KeyNotFind(msg) => write!(f, "key {} not found", msg),
        }
    }
}

/// 验证码参数
pub struct CaptchaCombine {
    pub gt: String,
    pub challenge: String,
    pub key: String,
}

impl CaptchaCombine {
    /// ### 申请验证码参数
    ///
    /// 获取并初始化 `CaptchaCombine`
    pub async fn fetch() -> Result<Self, Error> {
        let captcha_combine_data = get(
            "https://passport.bilibili.com/web/captcha/combine",
            Some(vec![("plat", "6")]),
        )
        .await
        .json()
        .await
        .or_else(|error| Err(Error::TauriError(error.to_string())))?
        .data;

        let captcha_combine_result = captcha_combine_data
            .get("data")
            .ok_or(Error::KeyNotFind("data".into()))?
            .get("result")
            .ok_or(Error::KeyNotFind("result".into()))?;

        Ok(Self {
            gt: captcha_combine_result
                .get("gt")
                .ok_or(Error::KeyNotFind("gt".into()))?
                .as_str()
                .unwrap()
                .into(),
            challenge: captcha_combine_result
                .get("challenge")
                .ok_or(Error::KeyNotFind("challenge".into()))?
                .as_str()
                .unwrap()
                .into(),
            key: captcha_combine_result
                .get("key")
                .ok_or(Error::KeyNotFind("key".into()))?
                .as_str()
                .unwrap()
                .into(),
        })
    }
}

/// 加密公钥及密码盐值
pub struct LoginKeys {
    pub hash: String,
    pub key: String,
}

impl LoginKeys {
    /// ### 获取加密公钥及密码盐值
    ///
    /// 获取并初始化 `LoginKeys`
    pub async fn fetch() -> Result<Self, Error> {
        let login_keys_data = get(
            "https://passport.bilibili.com/login",
            Some(vec![("act", "getkey")]),
        )
        .await
        .json()
        .await
        .or_else(|error| Err(Error::TauriError(error.to_string())))?
        .data;

        Ok(Self {
            hash: login_keys_data
                .get("hash")
                .ok_or(Error::KeyNotFind("hash".into()))?
                .as_str()
                .unwrap()
                .into(),
            key: login_keys_data
                .get("key")
                .ok_or(Error::KeyNotFind("key".into()))?
                .as_str()
                .unwrap()
                .into(),
        })
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct LoginInfo {
    captchaType: String,
    username: String,
    pub password: String,
    keep: String,
    key: String,
    pub challenge: String,
    validate: String,
    seccode: String,
}

impl LoginInfo {
    pub fn new() -> Self {
        LoginInfo {
            captchaType: "6".into(),
            username: String::new(),
            password: String::new(), //codec::rsa_encode(&pem, &hash.add(&password)),
            keep: "true".into(),
            key: String::new(),
            challenge: String::new(),
            validate: String::new(),
            seccode: String::new(),
        }
    }

    pub fn username(&mut self, username: String) {
        self.username = username;
    }

    pub fn password(&mut self, password: String) {
        self.password = password;
    }

    pub fn key(&mut self, key: String) {
        self.key = key;
    }

    pub fn challenge(&mut self, challenge: String) {
        self.challenge = challenge;
    }

    pub fn validate(&mut self, validate: String) {
        self.validate = validate;
    }

    pub fn seccode(&mut self, seccode: String) {
        self.seccode = seccode;
    }

    fn as_vec<'a>(&self) -> Vec<(&'a str, String)> {
        vec![
            ("captchaType", self.captchaType.clone()),
            ("username", self.username.clone()),
            ("password", self.password.clone()),
            ("keep", self.keep.clone()),
            ("key", self.key.clone()),
            ("challenge", self.challenge.clone()),
            ("validate", self.validate.clone()),
            ("seccode", self.seccode.clone()),
        ]
    }

    pub async fn fetch(&self) -> Result<String, Error> {
        let data = post(
            "https://passport.bilibili.com/web/login/v2",
            None,
            Some(self.as_vec()),
        )
        .await
        .json()
        .await
        .or_else(|error| Err(Error::TauriError(error.to_string())))?
        .data;

        Ok(data.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{CaptchaCombine, LoginInfo, LoginKeys};

    #[tokio::test]
    async fn fetch_captcha_combine_test() {
        match CaptchaCombine::fetch().await {
            Ok(CaptchaCombine { gt, challenge, key }) => {
                assert!(!gt.is_empty());
                assert!(!challenge.is_empty());
                assert!(!key.is_empty());
            }
            Err(error) => {
                panic!("{}", error)
            }
        }
    }

    #[tokio::test]
    async fn fetch_login_keys_test() {
        match LoginKeys::fetch().await {
            Ok(LoginKeys { hash, key }) => {
                assert!(!hash.is_empty());
                assert!(!key.is_empty());
            }
            Err(error) => {
                panic!("{}", error)
            }
        }
    }

    #[tokio::test]
    async fn fetch_login_info_test() {
        let mut login_info = LoginInfo::new();
        login_info.username("username".to_string());
        login_info.password("password".to_string());
        login_info.key(
            "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAo4QBxWqrnzFAkCLBZ/+z
UGZPrbV267z/2fItMD91nZa79TqAmM0SjHCe+ESq9YbRAnQXTXDOXJc34Z9a2m9y
ZaBWexHPprIygKm1PIi9UrVa58EV/AbiBRc53ExvRDVZDjG6OPZfceTJS4nA+hRR
idT9ZlACtXid++lw2/Y32woJRj40Mjaxa0Hi7C0A+vyVL8SvDh1AvFOW5/dGnKkf
WMelpsyjmnJ0Ub1zr46aDT1m9Rb/lBijLjOqeEt0FgvpXJM5mb8N0oWdLoxir4MX
Z+MVhfGZtKOu533fwCvYD35Br/LbBLxnTwPolrvLZKOS6wEktWVqx/bJMc20h87G
8wIDAQAB
-----END PUBLIC KEY-----"
                .to_string(),
        );
        login_info.challenge("challenge".to_string());
        login_info.seccode("seccode".to_string());
        login_info.validate("validate".to_string());

        match login_info.fetch().await {
            Ok(s) => {
                assert!(!s.is_empty())
            }
            Err(error) => {
                panic!("{}", error)
            }
        }
    }
}
