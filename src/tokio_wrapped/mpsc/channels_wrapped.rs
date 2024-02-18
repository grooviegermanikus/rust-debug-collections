use std::time::Duration;
use log::{info, warn};
use tokio::sync::mpsc::error::{SendError, SendTimeoutError};
use tokio::sync::mpsc::Sender;


pub async fn send_timed<T>(value: T, sender: &Sender<T>) -> Result<(), SendError<T>> {
    send_timebound(value, sender, Duration::from_millis(5)).await
}

pub async fn send_timebound<T>(value: T, sender: &Sender<T>, timeout: Duration) -> Result<(), SendError<T>> {
    match sender.send_timeout(value, timeout).await {
        Ok(()) => {
            return Ok(());
        }
        Err(SendTimeoutError::Timeout(value)) => {
            warn!("Timout sending value after {:?}", timeout);
            sender.send(value).await
        }
        Err(SendTimeoutError::Closed(value)) => {
            warn!("Receiver closed");
            return Err(SendError(value));
        }
    }
}




