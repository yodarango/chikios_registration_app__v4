use serde::Deserialize;
use serde_json::json;
use warp::{Filter, Rejection, Reply};

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
pub async  fn authorize(credentials: Credentials) -> Result<impl Reply, Rejection>{
    println!("Credentials: {:?}", credentials);
    if credentials.username == "admin" && credentials.password == "k1dzqu35t2023" {
        let token = json!({
            "token": "3Wq94yEsdn_93_394JjkVh_o2"
        });

        return Ok(warp::reply::json(&token))
    }

    let token = json!({
        "error": "Invalid credentials"});
        
    return Ok(warp::reply::json(&token))

}