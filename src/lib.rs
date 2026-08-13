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
    /// triggered after app being opened or foregrounded based on a click on a
    /// [universal link](https://developer.apple.com/documentation/xcode/supporting-universal-links-in-your-app):
    /// an ordinary `https://` address of a domain the app claims via its
    /// `associated-domains` entitlement. Carries the web page URL that was
    /// clicked, so the app can route the visitor to the same place its website
    /// would have.
    ///
    /// Only browsing-web activities produce this. Handoff and Spotlight arrive at
    /// the same delegate call with no web page URL and are dropped.
    UniversalLink(String),
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
