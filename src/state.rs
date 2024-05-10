use std::sync::Arc;

use futures::lock::Mutex;

pub struct AudioState {
    pub status: String,
}

pub async fn set_status(status: String, audio_state: Arc<Mutex<AudioState>>) {
    let mut state = audio_state.lock().await;
    state.status = status;
}
