//! Background work and the messages it leaves behind.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    Running,
    Done(String),
    Failed(String),
}

/// One operation running off the UI thread, shown with its progress.
#[derive(Debug, Clone)]
pub struct Job {
    pub id: u64,
    pub label: String,
    pub status: JobStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Info,
    Error,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub id: u64,
    pub kind: ToastKind,
    pub text: String,
    /// Sliding out; dropped from the list once the exit has played.
    pub leaving: bool,
    /// The pointer is on it, so its timer waits.
    pub hovered: bool,
    /// Bumped on every hover change so an older timer finds itself stale.
    pub epoch: u32,
}
