use reqwest::Client;
use serde_json::Value;
#[tokio::main]
async fn main() {
    let host = "http://127.0.0.1:8090";

    let values = ("your_bucket_name", "your_key_image_name");
    let client = Client::new();
    let post_detect_text_url = format!("{host}/get_text_given_image_stored_on_bucket");
    let detected_text = client
        .post(post_detect_text_url)
        .json(&values)
        .send()
        .await
        .unwrap();
    println!(
        "\n\nDetected Text Given image stored in the bucket: \n{}\n",
        detected_text.text().await.unwrap()
    );
    let count = 2;
    if count == 1 {
        let new_bucket_name = "Prevent calling multiple times";
        let client = Client::new();
        let post_create_bucket_url = format!("{host}/create_bucket");
        let create_bucket = client
            .post(post_create_bucket_url)
            .json(new_bucket_name)
            .send()
            .await
            .expect("Error Communicating with server or bad request")
            .text()
            .await
            .unwrap();
        println!("Create Bucket Status: {}\n", create_bucket);
    }

    //Returns array of string: [lists: ['','']]
    let get_bucket_list_url = format!("{host}/get_buckets");
    let bucket_lists = Client::new()
        .get(get_bucket_list_url)
        .send()
        .await
        .expect("Error while connecting to server")
        .json::<serde_json::Value>()
        .await
        .unwrap();
    println!("Available Buckets\n");
    bucket_lists
        .as_array()
        .expect("This is an array of string values")
        .into_iter()
        .for_each(|bucket| {
            println!("{}", bucket.as_str().expect("This is a type of string"));
        });
    println!("");
    println!("Available Keys/objects in the given Bucket\n");
    let bucket_name = "your_bucket_name";
    let get_keys_url = format!("{host}/get_bucket_keys/{}", bucket_name);
    let get_keys_in_a_bucket = Client::new()
        .get(get_keys_url)
        .send()
        .await
        .expect("Error while communicating")
        .json::<Value>()
        .await
        .expect("Error Parsing JSON Array");
    get_keys_in_a_bucket
        .as_array()
        .expect("This is an array")
        .into_iter()
        .for_each(|key| {
            println!("{}", key.as_str().unwrap());
        });
    println!("");
    let engine_name = "neural";
    //let engine_name ="standard";
    let language_code = "en-US";
    let ssml_text = include_str!("./../example.ssml");
    let post_audio_generation_url =
        format!("{host}/generate_polly_audios/{engine_name}/{language_code}");
    let response = Client::new()
        .post(post_audio_generation_url)
        .json(ssml_text)
        .send()
        .await
        .expect("Error while connecting\n")
        .text().await.unwrap();
    println!("Status of the Response: {}\n",response);
}
