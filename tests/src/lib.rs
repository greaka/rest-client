#[cfg(test)]
mod tests {
    extern crate rest_client;

    use rest_client::*;
    use serde::Deserialize;

    #[rest("https://jsonplaceholder.typicode.com/comments?postId={}", vec)]
    #[rest("https://jsonplaceholder.typicode.com/comments/{}")]
    #[derive(Deserialize)]
    struct Comments {
        #[serde(rename = "postId")]
        post_id: u64,
        id: u64,
        name: String,
        email: String,
        body: String,
    }
}
