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
}
