use bevy_channel_trigger::ChannelSender;
use std::sync::OnceLock;

use crate::AppDelegateCall;

static SENDER: OnceLock<Option<ChannelSender<AppDelegateCall>>> = OnceLock::new();

#[allow(dead_code)]
pub fn send_event(e: AppDelegateCall) {
    let Some(sender) = SENDER.get().and_then(Option::as_ref) else {
        return bevy_log::error!(
            "`BevyIosAppDelegatePlugin` not installed correctly (no sender found)"
        );
    };
    sender.send(e);
}

pub fn set_sender(sender: ChannelSender<AppDelegateCall>) {
    while SENDER.set(Some(sender.clone())).is_err() {}
}
