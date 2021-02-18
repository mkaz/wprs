use chrono::{DateTime, Duration, Utc};
use reqwest::blocking::multipart;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct PostResponse {
    #[serde(rename = "short_URL")]
    short_url: String,
}

pub fn create(config: super::config::Config, filename: &str, publish: bool) {
    // Create new post
    let apiurl = format!(
        "https://public-api.wordpress.com/rest/v1.1/sites/{}/posts/new",
        config.blog_id
    );

    let status = if publish { "publish" } else { "draft" };

    // TODO: improve post template to allow dynamic content
    // - set title from file
    let post_content = fs::read_to_string(filename).unwrap();

    // scheduled to run Monday morning
    // so plus two days is the Wednesday date
    let dt_wed: DateTime<Utc> = Utc::now() + Duration::days(2);
    let title = format!("Tinker Weekly: {}", dt_wed.format("%b %e, %Y"));
    let form = multipart::Form::new()
        .text("author", config.author)
        .text("title", title)
        .text("content", post_content)
        // TODO: pull from post-template?
        .text("tags", "team-meeting")
        // TODO: add flag for create subcommand
        .text("status", status);

    let response = Client::new()
        .post(&apiurl)
        .bearer_auth(config.token.clone())
        .multipart(form)
        .send();

    match response {
        Ok(res) => {
            // Note: res.json() is wrapped in an option
            match res.json::<PostResponse>() {
                Ok(post) => {
                    println!("New post created: {}", post.short_url);
                }
                Err(e) => {
                    println!("Error parsing response: {:?}", e);
                }
            }
        }
        Err(e) => panic!("Request error: {:?} ", e),
    };
}
