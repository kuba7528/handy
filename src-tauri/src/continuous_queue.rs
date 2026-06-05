use crate::actions::process_continuous_segment;
use log::warn;
use std::sync::mpsc::{self, Sender};
use std::thread;
use tauri::AppHandle;

enum QueueCommand {
    Segment(Vec<f32>),
}

/// Serialises continuous-listening segment processing through a single worker
/// thread so rapid utterances never race on ASR model loading.
pub struct ContinuousSegmentQueue {
    tx: Sender<QueueCommand>,
}

impl ContinuousSegmentQueue {
    pub fn new(app: AppHandle) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            while let Ok(QueueCommand::Segment(samples)) = rx.recv() {
                tauri::async_runtime::block_on(process_continuous_segment(&app, samples));
            }
        });

        Self { tx }
    }

    pub fn enqueue(&self, samples: Vec<f32>) {
        if self.tx.send(QueueCommand::Segment(samples)).is_err() {
            warn!("Continuous segment queue channel closed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_command_enum_exists() {
        let _ = QueueCommand::Segment(vec![0.0]);
    }
}
