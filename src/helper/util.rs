use reqwest::Response;
use serde_json::Value;
use serde::Deserialize;

use rusty_ytdl::{Video, VideoQuality, VideoSearchOptions};
use std::io::{BufReader, Sink, Cursor};



use rusty_ytdl::{choose_format,VideoOptions};

#[derive(Debug, Deserialize)]
struct VideoDetails {
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    snippet: Snippet,
}

#[derive(Debug, Deserialize)]
struct Snippet {
    title: String,
}

pub async fn get_search_songs(song_name: &str, api_key: &str) -> Result<Value, Box<dyn std::error::Error>> {
    // Construct the API endpoint URL
    let search_query: &str = song_name;
    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?q={}&part=id&type=video&key={}",
        search_query, api_key
    );

    // Make the HTTP request
    let todos = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await?;
        let v: Value = serde_json::from_str(&todos)?;
        if let Some(items) = v["items"].as_array() {
            for item in items {
                if let Some(video_id) = item["id"]["videoId"].as_str() {
                    // Do something with the video_id
                    // println!("Video ID: {}", video_id);
                    let _ = get_video_title(video_id,api_key).await;
                }
            }
        }
        return Ok(v);
}


async fn get_video_title(video_id: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the API endpoint URL for video details
    let video_url = format!(
        "https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet&key={}",
        video_id, api_key
    );

    // Make the HTTP request for video details
    let response = reqwest::Client::new()
    .get(video_url)
    .send()
    .await?
    .text()
    .await?;

    // Deserialize JSON response
    // let video_details: VideoDetails = response.json().await?;
    let video_details: VideoDetails = serde_json::from_str(&response)?;
    println!("{:?} ---- ID {:?}",video_details.items[0].snippet.title,video_id);
    Ok(())
}

pub async fn play_song(video_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let video_options = VideoOptions {
        quality: VideoQuality::LowestAudio,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
      };

    let video_url = format!(
        "https://www.youtube.com/watch?v={}",
        video_id
    );
    
    let video: Video = Video::new_with_options(video_url, video_options).unwrap();
    
    let mut buf: Vec<u8> = Vec::new();
    let stream = video.stream().await.unwrap();

    // Create a BufReader from the accumulated bytes


    while let Some(chunk) = stream.chunk().await.unwrap() {
        // Do what you want with chunks
        
        buf.extend_from_slice(&chunk);
        
    }
    let buf_reader = BufReader::new(Cursor::new(buf));
    
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    sink.append(rodio::Decoder::new_mp3(BufReader::new(buf_reader)).unwrap());
    // sink.sleep_until_end();
    let handle = tokio::task::spawn_blocking(move || {
        sink.sleep_until_end();
    });
    handle.await?;
    
    Ok(())
}

