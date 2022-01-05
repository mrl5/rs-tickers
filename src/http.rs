use std::time::Duration;

pub fn get_client() -> Result<reqwest::blocking::Client, reqwest::Error> {
    let timeout = Duration::from_secs(10);
    let is_verbose = true;
    let tcp_keepalive = Duration::from_secs(180);
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36";

    reqwest::blocking::Client::builder()
        .timeout(timeout)
        .connection_verbose(is_verbose)
        .tcp_keepalive(tcp_keepalive)
        .user_agent(user_agent)
        .build()
}
