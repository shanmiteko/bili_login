use std::fmt::Display;

use crate::http::{get, post};

pub enum Error {
    TauriError(String),
    KeyNotFound(String),
    LoginError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TauriError(msg) => write!(f, "{}", msg),
            Error::KeyNotFound(msg) => write!(f, "key {} not found", msg),
            Error::LoginError(msg) => write!(f, "登录失败: {}", msg),
        }
    }
}

/// 验证码参数
pub struct Captcha {
    pub gt: String,
    pub challenge: String,
    pub token: String,
}

impl Captcha {
    /// ### 申请验证码参数
    ///
    /// 获取并初始化 `Captcha`
    pub async fn fetch() -> Result<Self, Error> {
        let captcha_res = get(
            "https://passport.bilibili.com/x/passport-login/captcha",
            Some(vec![("source".to_string(), "main_web".to_string())]),
        )
        .await
        .json()
        .await
        .map_err(|error| Error::TauriError(error.to_string()))?
        .data;

        let captcha_data = captcha_res
            .get("data")
            .ok_or(Error::KeyNotFound("Captcha data".into()))?;

        let captcha_data_geetest = captcha_data
            .get("geetest")
            .ok_or(Error::KeyNotFound("Captcha data geetest".into()))?;

        Ok(Self {
            gt: captcha_data_geetest
                .get("gt")
                .ok_or(Error::KeyNotFound("Captcha data geetest gt".into()))?
                .as_str()
                .unwrap()
                .into(),
            challenge: captcha_data_geetest
                .get("challenge")
                .ok_or(Error::KeyNotFound("Captcha data geetest challenge".into()))?
                .as_str()
                .unwrap()
                .into(),
            token: captcha_data
                .get("token")
                .ok_or(Error::KeyNotFound("Captcha data token".into()))?
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
        let login_keys_res = get(
            "https://passport.bilibili.com/x/passport-login/web/key",
            None,
        )
        .await
        .json()
        .await
        .map_err(|error| Error::TauriError(error.to_string()))?
        .data;

        let login_keys_data = login_keys_res
            .get("data")
            .ok_or(Error::KeyNotFound("LoginKeys data".into()))?;

        Ok(Self {
            hash: login_keys_data
                .get("hash")
                .ok_or(Error::KeyNotFound("LoginKeys data hash".into()))?
                .as_str()
                .unwrap()
                .into(),
            key: login_keys_data
                .get("key")
                .ok_or(Error::KeyNotFound("LoginKeys data key".into()))?
                .as_str()
                .unwrap()
                .into(),
        })
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct LoginInfo {
    username: String,
    pub password: String,
    keep: String,
    token: String,
    pub challenge: String,
    validate: String,
    seccode: String,
}

impl LoginInfo {
    pub fn new() -> Self {
        LoginInfo {
            username: String::new(),
            password: String::new(), //codec::rsa_encode(&pem, &hash.add(&password)),
            keep: "0".into(),
            token: String::new(),
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

    pub fn token(&mut self, token: String) {
        self.token = token;
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

    fn to_vec(&self) -> Vec<(String, String)> {
        vec![
            ("username".to_string(), self.username.clone()),
            ("password".to_string(), self.password.clone()),
            ("keep".to_string(), self.keep.clone()),
            ("token".to_string(), self.token.clone()),
            ("challenge".to_string(), self.challenge.clone()),
            ("validate".to_string(), self.validate.clone()),
            ("seccode".to_string(), self.seccode.clone()),
        ]
    }

    pub async fn fetch(&self) -> Result<String, Error> {
        let login_res = post(
            "https://passport.bilibili.com/x/passport-login/web/login",
            None,
            Some(self.to_vec()),
        )
        .await
        .json()
        .await
        .map_err(|error| Error::TauriError(error.to_string()))?
        .data;

        match login_res
            .get("code")
            .ok_or(Error::KeyNotFound("code".into()))?
            .as_i64()
            .unwrap()
        {
            0 => {
                dbg!(&login_res)
            }
            _ => {
                return Err(Error::LoginError(
                    login_res
                        .get("message")
                        .ok_or(Error::KeyNotFound("message".into()))?
                        .as_str()
                        .unwrap()
                        .into(),
                ));
            }
        };

        Ok(login_res
            .get("data")
            .ok_or(Error::KeyNotFound("data".into()))?
            .get("url")
            .ok_or(Error::KeyNotFound("url".into()))?
            .as_str()
            .unwrap()
            .into())
    }
}

#[cfg(test)]
mod tests {
    use super::{Captcha, Error, LoginInfo, LoginKeys};

    #[tokio::test]
    async fn fetch_captcha_combine_test() {
        match Captcha::fetch().await {
            Ok(Captcha {
                gt,
                challenge,
                token,
            }) => {
                assert!(!gt.is_empty());
                assert!(!challenge.is_empty());
                assert!(!token.is_empty());
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
        login_info.token(
            "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDjb4V7EidX/ym28t2ybo0U6t0n
6p4ej8VjqKHg100va6jkNbNTrLQqMCQCAYtXMXXp2Fwkk6WR+12N9zknLjf+C9sx
/+l48mjUU8RqahiFD1XT/u2e0m2EN029OhCgkHx3Fc/KlFSIbak93EH/XlYis0w+
Xl69GV6klzgxW6d2xQIDAQAB
-----END PUBLIC KEY-----
"
            .to_string(),
        );
        login_info.challenge("challenge".to_string());
        login_info.seccode("seccode".to_string());
        login_info.validate("validate".to_string());

        match login_info.fetch().await {
            Ok(s) => {
                assert!(!s.is_empty())
            }
            Err(error) => match error {
                Error::LoginError(_) => (),
                _ => panic!(),
            },
        }
    }
}
