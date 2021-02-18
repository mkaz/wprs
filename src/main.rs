use clap::{App, Arg};
use oauth2;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::io;

mod config;
mod posts;

#[derive(Deserialize, Debug)]
struct UsersResponse {
    name: String,
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
                    Arg::new("title")
                        .about("Title for post")
                        .long("title")
                        .required(true)
                        .takes_value(true),
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
    let config = config::get_config("wprs.conf");

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
                let title = matches.value_of("title").unwrap();
                let filename = matches.value_of("file").unwrap();
                posts::create(config.clone(), title.to_string(), filename, publish);
            }
        }
        _ => {
            println!("Command not found");
        }
    }
}

fn auth(config: config::Config) {
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

fn connection_test(config: config::Config) {
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
