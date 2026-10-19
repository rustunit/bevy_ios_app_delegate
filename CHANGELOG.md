# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added
* UIScene life cycle support, which apps built with the iOS 27 SDK need or they fail to launch
  ("UIScene life cycle is required for apps built with this SDK"). winit does not adopt it yet
  ([winit#4224](https://github.com/rust-windowing/winit/issues/4224)), so the plugin provides the
  scene delegate and moves winit's window onto the scene. Opt in by adding a
  `UIApplicationSceneManifest` to the app's `Info.plist` (see README); apps without one behave as
  before
* with scenes enabled, `AppDelegateCall::OpenURL` is triggered from the scene delegate, both when a
  URL launches the app and while it is running

## [0.6.0] - 2026-10-19

### Changed
* upgrade to bevy `0.20`

## [0.5.0] - 2026-08-09

### Changed
* upgrade to bevy `0.19`

## [0.4.0] - 2026-01-15

### Changed
* upgrade to bevy `0.18`

## [0.3.0] - 2025-10-18

### Changed
* upgrade to bevy `0.17`

## [0.2.0] - 2025-05-18

### Changed
* upgrade to bevy `0.16`

## [0.1.1] - 2025-05-16

### Changed
* usage guide and docs

## [0.1.0] - 2025-05-16

### Added
* `IosAppDelegatePlugin` installing a custom `UIApplicationDelegate` that triggers
  `AppDelegateCall::OpenURL` when the app is opened via a URL scheme
