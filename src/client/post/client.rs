use super::dto::*;
use reqwest;

#[derive(Clone)]
pub struct PostClient {
    base_url: String,
    client: reqwest::Client,
}

impl PostClient {
    const LIST_URL: &str = "/v1/posts";
    const DETAIL_URL: &str = "/v1/posts/:id";

    pub fn new(addrs: Vec<String>) -> Self {
        Self {
            base_url: addrs.first().unwrap().clone(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn lists(&self, page: u32, page_size: u32) -> Result<ListResponse, reqwest::Error> {
        let query = &[("page", page), ("page_size", page_size)];
        let response = self
            .client
            .get(format!("{}{}", self.base_url, Self::LIST_URL))
            .query(query)
            .send()
            .await?;
        let resp = response.json::<ListResponse>().await?;
        Ok(resp)
    }

    pub async fn detail(&self, id: u64) -> Result<DetailResponse, reqwest::Error> {
        let path = Self::DETAIL_URL.replace(":id", id.to_string().as_str());
        let response = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?;
        let resp = response.json::<DetailResponse>().await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_client() -> PostClient {
        let addrs = vec!["http://127.0.0.1:8080".to_string()];
        PostClient::new(addrs)
    }

    #[tokio::test]
    async fn test_post_lists() {
        let client = new_client();
        let resp = client.lists(1, 10).await.unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_post_detail() {
        let client = new_client();
        let resp = client.detail(1).await.unwrap();
        println!("{:?}", resp);
    }
}
