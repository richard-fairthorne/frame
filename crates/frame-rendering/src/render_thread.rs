use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use vello::Scene;

pub enum RenderCommand {
    Render {
        scene: Box<Scene>,
        width: u32,
        height: u32,
    },
    Resize {
        width: u32,
        height: u32,
    },
    Shutdown,
}

pub struct RenderThreadHandle {
    tx: Sender<RenderCommand>,
}

impl RenderThreadHandle {
    pub fn submit(&self, scene: Scene, width: u32, height: u32) {
        let _ = self.tx.send(RenderCommand::Render {
            scene: Box::new(scene),
            width,
            height,
        });
    }

    pub fn resize(&self, width: u32, height: u32) {
        let _ = self.tx.send(RenderCommand::Resize { width, height });
    }

    pub fn shutdown(&self) {
        let _ = self.tx.send(RenderCommand::Shutdown);
    }
}

pub struct SceneReceiver {
    rx: Receiver<RenderCommand>,
}

impl SceneReceiver {
    pub fn drain_latest(&self) -> Option<RenderCommand> {
        let mut latest = None;
        loop {
            match self.rx.try_recv() {
                Ok(cmd) => {
                    if let Some(prev) = latest.replace(cmd) {
                        drop(prev);
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
        latest
    }

    pub fn has_pending(&self) -> bool {
        match self.rx.try_recv() {
            Ok(cmd) => {
                drop(cmd);
                true
            }
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => false,
        }
    }
}

pub fn spawn_render_thread() -> (RenderThreadHandle, SceneReceiver) {
    let (tx, rx) = channel();
    (RenderThreadHandle { tx }, SceneReceiver { rx })
}
