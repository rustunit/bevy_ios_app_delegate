# bevy_ios_app_delegate

[![crates.io][sh_crates]][lk_crates]
[![docs.rs][sh_docs]][lk_docs]
[![discord][sh_discord]][lk_discord]

[sh_crates]: https://img.shields.io/crates/v/bevy_ios_app_delegate.svg
[lk_crates]: https://crates.io/crates/bevy_ios_app_delegate
[sh_docs]: https://img.shields.io/docsrs/bevy_ios_app_delegate
[lk_docs]: https://docs.rs/bevy_ios_app_delegate/latest/bevy_ios_app_delegate/
[sh_discord]: https://img.shields.io/discord/1176858176897953872?label=discord&color=5561E6
[lk_discord]: https://discord.gg/rQNeEnMhus

Read more about usage and examples in this [blog post](https://rustunit.com/blog/2025/05-18-bevy-ios-deep-linking/).

## Features
* trigger `AppDelegateCall::OpenURL` event if app was opened or forgrounded based on a click on a [URL schema](https://developer.apple.com/documentation/xcode/defining-a-custom-url-scheme-for-your-app)

## Usage

1. Make sure your `winit` dependency is at least `0.30.10` (where winit stopped implementing `AppDelegate` itself)
2. Add dependency `cargo add bevy_ios_app_delegate`
3. Register plugin `app.add_plugins((IosAppDelegatePlugin));`
4. Add observer: `app.add_observer(|trigger: On<AppDelegateCall>| {});`

## Our Other Crates

- [bevy_ios_iap](https://github.com/rustunit/bevy_ios_iap)
- [bevy_ios_review](https://github.com/rustunit/bevy_ios_review)
- [bevy_ios_gamecenter](https://github.com/rustunit/bevy_ios_gamecenter)
- [bevy_ios_alerts](https://github.com/rustunit/bevy_ios_alerts)
- [bevy_ios_notifications](https://github.com/rustunit/bevy_ios_notifications)
- [bevy_ios_impact](https://github.com/rustunit/bevy_ios_impact)
- [bevy_ios_safearea](https://github.com/rustunit/bevy_ios_safearea)
- [bevy_debug_log](https://github.com/rustunit/bevy_debug_log)
- [bevy_device_lang](https://github.com/rustunit/bevy_device_lang)
- [bevy_web_popups](https://github.com/rustunit/bevy_web_popups)
- [bevy_libgdx_atlas](https://github.com/rustunit/bevy_libgdx_atlas)

## Bevy version support

|bevy|crate|
|---|---|
|0.18|0.4,main|
|0.17|0.3|
|0.16|0.2|
|0.15|0.1|

# License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

## Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
