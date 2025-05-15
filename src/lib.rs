use bevy_app::{App, Plugin};
use bevy_channel_trigger::ChannelTriggerApp;
use bevy_ecs::event::Event;
use channel::set_sender;

mod channel;
#[cfg(any(target_os = "ios"))]
mod swizzle;

#[derive(Event, Clone, Debug)]
pub enum AppDelegateCall {
    OpenURL(String),
}

pub struct IosAppDelegatePlugin;

impl Plugin for IosAppDelegatePlugin {
    fn build(&self, app: &mut App) {
        let sender = app.add_channel_trigger::<AppDelegateCall>();

        set_sender(sender);

        #[cfg(any(target_os = "ios"))]
        swizzle::swizzle();
    }
}
