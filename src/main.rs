use chrono::{DateTime, Duration, Utc};
use clap::{App, Arg};
use oauth2;
use reqwest::blocking::multipart;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs;
use std::io;
use toml;

#[derive(Clone, Deserialize)]
struct Config {
    author: String,
    auth_url: String,
    token_url: String,
    blog_id: u32,
    blog_url: String,
    client_id: u32,
    client_secret: String,
    token: String,
}

#[derive(Deserialize, Debug)]
struct UsersResponse {
    name: String,
}

#[derive(Deserialize, Debug)]
struct PostResponse {
    #[serde(rename = "short_URL")]
    short_url: String,
}

fn main() {
    let args = App::new("wprs")
        .version("0.1.0")
        .about("A command-line WordPress client")
        .author("Marcus Kazmierczak")
        .arg(
            Arg::new("config")
                .about("Configuration file")
                .short('c')
                .long("config")
                .takes_value(true),
        )
        .subcommand(App::new("auth").about("use to login"))
        .subcommand(App::new("test").about("use to test connection"))
        .subcommand(
            App::new("create")
                .about("use to create new post")
                .arg(
                    Arg::new("publish")
                        .about("Publish immediately (default: Draft)")
                        .long("publish"),
                )
                .arg(
                    Arg::new("file")
                        .about("HTML file to publish")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    // read in config
    let config_file = fs::read_to_string("wprs.conf").unwrap();
    let config: Config = toml::from_str(&config_file).unwrap();

    match args.subcommand_name() {
        Some("auth") => {
            auth(config.clone());
        }
        Some("test") => {
            connection_test(config.clone());
        }
        Some("create") => {
            if let Some(matches) = args.subcommand_matches("create") {
                let publish = matches.is_present("publish");
                let filename = matches.value_of("file").unwrap();
                create_post(config.clone(), filename, publish);
            }
        }
        _ => {
            println!("Command not found");
        }
    }
}

fn create_post(config: Config, filename: &str, publish: bool) {
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

fn auth(config: Config) {
    let mut oauth = oauth2::Config::new(
        format!("{}", config.client_id),
        config.client_secret,
        config.auth_url,
        config.token_url,
    );

    // Set the desired scopes.
    oauth = oauth.add_scope("read");
    oauth = oauth.add_scope("write");

    // Set the URL the user will be redirected to after the authorization process.
    oauth = oauth.set_redirect_url(config.blog_url);

    // Generate the full authorization URL.
    // The user needs to go to URL and authenticate.
    println!("Browse to: {}", oauth.authorize_url());

    // After authentication get ?code=XXXX
    // Prompt user to enter code to exchange for token
    println!("After authorization enter code: ");
    let mut code = String::new();
    io::stdin()
        .read_line(&mut code)
        .expect("error: unable to read user input");

    // remove whitespace
    let code = code.trim();

    // Use code to get auth token
    let token_result = oauth.exchange_code(code);

    // TODO: write token to wprs.conf
    match token_result {
        Ok(t) => {
            println!("Update wprs.conf with token");
            println!("Token: {}", t.access_token);
        }
        Err(_) => {
            println!("Error fetching token");
        }
    }
}

fn connection_test(config: Config) {
    // test connection
    let apiurl = format!(
        "https://public-api.wordpress.com/rest/v1.1/sites/{}",
        config.blog_id
    );

    let response = Client::new().get(&apiurl).bearer_auth(config.token).send();

    match response {
        Ok(res) => {
            // Note: res.json() is wrapped in a Result
            println!("Found: {:?}", res.json::<UsersResponse>());
        }
        Err(e) => panic!("Request error: {:?} ", e),
    };
}
