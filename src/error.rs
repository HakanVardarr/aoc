use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to get env variable AOC_SESSION")]
    AocSession,
    #[error("Failed to build client")]
    ClientBuild,
    #[error("Failed to send request to the given url.")]
    SendRequest,
    #[error("Given aoc session is invalid please check it.")]
    BadRequest,
    #[error("Input not found check the day and year")]
    NotFound,
}
