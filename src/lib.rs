use bevy_app::{App, Plugin};
use bevy_channel_trigger::ChannelTriggerApp;
use bevy_ecs::event::Event;
use channel::set_sender;

mod channel;
#[cfg(target_os = "ios")]
mod swizzle;

/// Event being triggered based on AppDelegate calls
#[derive(Event, Clone, Debug)]
pub enum AppDelegateCall {
    /// triggered after app being opened or foregrounded based on a click on a URL schema
    /// see <https://developer.apple.com/documentation/xcode/defining-a-custom-url-scheme-for-your-app>
    OpenURL(String),
}

/// Plugin to hook into iOS app delegate calls
/// It will use swizzling to hook into UIApplication init to register our custom AppDelegate as soon as winit starts the UIKit initialization procedure
/// This only takes effect on ios builds
pub struct IosAppDelegatePlugin;

impl Plugin for IosAppDelegatePlugin {
    fn build(&self, app: &mut App) {
        let sender = app.add_channel_trigger::<AppDelegateCall>();

        set_sender(sender);

        #[cfg(target_os = "ios")]
        swizzle::swizzle();
    }
}
