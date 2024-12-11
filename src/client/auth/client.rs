use super::dto::*;
use reqwest;

#[derive(Clone)]
pub struct AuthClient {
    base_url: String,
    client: reqwest::Client,
}

impl AuthClient {
    const REGISTER_URL: &str = "/v1/auth/register";
    const LOGIN_URL: &str = "/v1/auth/login";

    pub fn new(addrs: Vec<String>) -> Self {
        Self {
            base_url: addrs.first().unwrap().clone(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<RegisterResponse, reqwest::Error> {
        let response = self
            .client
            .post(format!("{}{}", self.base_url, Self::REGISTER_URL))
            .json(&req)
            .send()
            .await?;
        let resp = response.json::<RegisterResponse>().await?;
        Ok(resp)
    }

    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, reqwest::Error> {
        let response = self
            .client
            .post(format!("{}{}", self.base_url, Self::LOGIN_URL))
            .json(&req)
            .send()
            .await?;
        let resp = response.json::<LoginResponse>().await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    fn new_client() -> AuthClient {
        let addrs = vec!["http://127.0.0.1:8080".to_string()];
        AuthClient::new(addrs)
    }

    #[tokio::test]
    async fn test_auth_register() {
        let client = new_client();
        let rand_num = rand::thread_rng().gen_range(1..1000000);
        let name = format!("test{}", rand_num);
        let req = RegisterRequest {
            name: name.clone(),
            email: format!("{}@test.com", name),
            password: "123456".to_string(),
            password_confirmation: "123456".to_string(),
        };
        let resp = client.register(req).await.unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_auth_login() {
        let client = new_client();
        let req = LoginRequest {
            name: "test".to_string(),
            password: "123456".to_string(),
        };
        let resp = client.login(req).await.unwrap();
        println!("{:?}", resp);
    }
}
