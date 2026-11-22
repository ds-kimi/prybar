//! Steam Workshop publish and update. The Steam SDK is only linked with
//! the `workshop` feature; without it every network call returns
//! [`WorkshopError::Disabled`] while dry runs still validate inputs.
//! Every Steam call goes through the [`Steam`] trait so the whole flow
//! runs against [`mock::MockSteam`] in tests.

pub mod addon;
mod avatar;
mod backend;
mod error;
pub mod preview;
mod publish;
mod report;
pub mod spec;
mod stage;

pub use avatar::Avatar;
pub use backend::{avatar as account_avatar, mock, steam, Steam};
pub use error::WorkshopError;
pub use publish::{list_items, publish, publish_with, status};
pub use report::{ListedItem, Progress, PublishReport, SteamStatus};
