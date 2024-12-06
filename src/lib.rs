pub mod days;
mod error;

use error::Error;
use reqwest::{blocking::ClientBuilder, cookie::Jar, StatusCode, Url};

pub trait Solver {
    fn first_part(&self) -> String;
    fn second_part(&self) -> String;
}

pub trait InputReciever {
    fn recieve_input(day: u32, year: u32) -> Result<String, Error> {
        let aoc_session = std::env::var("AOC_SESSION").map_err(|_| Error::AocSession)?;
        let cookie = format!("session={}", aoc_session);
        let url = format!("https://adventofcode.com/{year}/day/{day}/input")
            .parse::<Url>()
            .unwrap();

        let jar = Jar::default();
        jar.add_cookie_str(&cookie, &url);

        let client = ClientBuilder::new()
            .cookie_provider(std::sync::Arc::new(jar))
            .build()
            .map_err(|_| Error::ClientBuild)?;

        let response = client.get(url).send().map_err(|_| Error::SendRequest)?;

        if response.status() == StatusCode::BAD_REQUEST {
            return Err(Error::BadRequest);
        } else if response.status() == StatusCode::NOT_FOUND {
            return Err(Error::NotFound);
        }

        response.text().map_err(|_| Error::SendRequest)
    }
}

#[macro_export]
macro_rules! day {
    ($name:ident, $day: expr, $year: expr) => {
        pub struct $name {
            input: String,
        }

        impl InputReciever for $name {}

        impl $name {
            pub fn new() -> Result<Self, Error> {
                use InputReciever;

                let input = Self::recieve_input($day, $year)?;
                Ok(Self { input })
            }
        }
    };
}
