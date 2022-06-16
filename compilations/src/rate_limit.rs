///////////////////////////////////////////////////////////////////////////////
// NAME:            rate_limit.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Unit that enforces a 0.5 req/sec rate limit on all clients.
//
// CREATED:         06/16/2022
//
// LAST EDITED:     06/16/2022
////

use reqwest_middleware::{Error, RequestBuilder};
use tokio::{
    sync::{mpsc, oneshot}, task::JoinError, time::{sleep, Duration},
};

type ResponseResult = Result<reqwest::Response, Error>;

#[derive(Clone)]
pub struct RateLimiter {
    tx: mpsc::Sender<(RequestBuilder, oneshot::Sender<ResponseResult>)>,
}

pub struct ResponderTask {
    rx: mpsc::Receiver<(RequestBuilder, oneshot::Sender<ResponseResult>)>,
}

///////////////////////////////////////////////////////////////////////////////
// ResponderTask
////

impl ResponderTask {
    pub async fn spawn(mut self) -> Result<(), JoinError> {
        let task = tokio::spawn(async move {
            while let Some((request, channel)) = self.rx.recv().await {
                let delay = sleep(Duration::from_secs(2));
                let response = request.send().await;
                channel.send(response).unwrap();
                delay.await;
            }
        });

        task.await
    }
}

///////////////////////////////////////////////////////////////////////////////
// RateLimiter
////

impl RateLimiter {
    pub fn new() -> (Self, ResponderTask) {
        let (tx, rx) = mpsc::channel(32);
        let rate_limiter = Self {tx};
        let responder = ResponderTask {rx};

        (rate_limiter, responder)
    }

    pub async fn send(&mut self, request: RequestBuilder) -> ResponseResult {
        let (response_tx, response_rx) = oneshot::channel();
        if let Err(_) = self.tx.send((request, response_tx)).await {
            panic!("Rate Limiting channel receiver dropped!");
        }
        response_rx.await.unwrap()
    }
}

///////////////////////////////////////////////////////////////////////////////
