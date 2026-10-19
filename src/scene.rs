//! UIScene life cycle, which apps built against the iOS 27 SDK must adopt or they fail to launch.
//!
//! winit 0.30 creates its `UIWindow` without a scene (rust-windowing/winit#4224), so we hand UIKit
//! our own scene delegate and move winit's windows onto the scene it connects.

use std::{cell::RefCell, mem};

use objc2::{
    ClassType, MainThreadOnly, Message, define_class, msg_send,
    rc::Retained,
    runtime::{Imp, NSObjectProtocol, Sel},
    sel,
};
use objc2_foundation::{NSBundle, NSSet, ns_string};
use objc2_ui_kit::{
    UIOpenURLContext, UIResponder, UIScene, UISceneConfiguration, UISceneConnectionOptions,
    UISceneDelegate, UISceneSession, UIWindow, UIWindowScene, UIWindowSceneDelegate,
};

use crate::swizzle::open_url;

thread_local! {
    static SCENE: RefCell<Option<Retained<UIWindowScene>>> = const { RefCell::new(None) };
    // Windows made visible before the scene connected.
    static PENDING: RefCell<Vec<Retained<UIWindow>>> = const { RefCell::new(Vec::new()) };
}

define_class!(
    #[unsafe(super(UIResponder))]
    #[thread_kind = MainThreadOnly]
    struct SceneDelegate;

    unsafe impl NSObjectProtocol for SceneDelegate {}

    unsafe impl UISceneDelegate for SceneDelegate {
        #[allow(non_snake_case)]
        #[unsafe(method(scene:willConnectToSession:options:))]
        unsafe fn scene_willConnectToSession_options(
            &self,
            scene: &UIScene,
            _session: &UISceneSession,
            options: &UISceneConnectionOptions,
        ) {
            if let Some(scene) = scene.downcast_ref::<UIWindowScene>() {
                connect(scene);
            }

            // A URL that cold-launches the app only arrives here, not in `scene:openURLContexts:`.
            // Declared nonnull, but nil when there is none.
            let url_contexts: Option<Retained<NSSet<UIOpenURLContext>>> =
                unsafe { msg_send![options, URLContexts] };
            if let Some(url_contexts) = url_contexts {
                open_url_contexts(&url_contexts);
            }
        }

        #[allow(non_snake_case)]
        #[unsafe(method(scene:openURLContexts:))]
        unsafe fn scene_openURLContexts(
            &self,
            _scene: &UIScene,
            url_contexts: &NSSet<UIOpenURLContext>,
        ) {
            open_url_contexts(url_contexts);
        }
    }

    unsafe impl UIWindowSceneDelegate for SceneDelegate {}
);

/// Whether the app's `Info.plist` opts into scenes; UIKit ignores our scene delegate otherwise.
pub(crate) fn enabled() -> bool {
    unsafe {
        NSBundle::mainBundle()
            .objectForInfoDictionaryKey(ns_string!("UIApplicationSceneManifest"))
            .is_some()
    }
}

pub(crate) fn configuration(session: &UISceneSession) -> Retained<UISceneConfiguration> {
    unsafe {
        // No name, so UIKit does not look for a matching configuration in the `Info.plist`.
        let config = UISceneConfiguration::configurationWithName_sessionRole(
            None,
            &session.role(),
            session.mtm(),
        );
        config.setDelegateClass(Some(SceneDelegate::class()));
        config
    }
}

fn connect(scene: &UIWindowScene) {
    SCENE.set(Some(scene.retain()));

    for window in PENDING.take() {
        unsafe { window.setWindowScene(Some(scene)) };
        window.makeKeyAndVisible();
    }

    bevy_log::debug!("scene connected");
}

/// winit's `initWithFrame:` window lands on UIKit's delegate-less, screen-based fallback scene,
/// which is never displayed.
fn needs_scene(window: &UIWindow) -> bool {
    unsafe { window.windowScene() }.is_none_or(|scene| unsafe { scene.delegate() }.is_none())
}

fn open_url_contexts(url_contexts: &NSSet<UIOpenURLContext>) {
    for context in url_contexts.iter() {
        open_url(&*unsafe { context.URL() });
    }
}

/// Swizzle `-[UIWindow makeKeyAndVisible]` to move windows onto our scene: winit calls it right
/// after creating its window.
pub(crate) fn swizzle() {
    type MakeKeyAndVisible = extern "C-unwind" fn(*mut UIWindow, Sel);

    static mut ORIGINAL: Option<MakeKeyAndVisible> = None;

    extern "C-unwind" fn make_key_and_visible(this: *mut UIWindow, sel: Sel) {
        let window = unsafe { &*this };

        if needs_scene(window) {
            match SCENE.with_borrow(Clone::clone) {
                Some(scene) => unsafe { window.setWindowScene(Some(&scene)) },
                None => PENDING.with_borrow_mut(|pending| {
                    if !pending.iter().any(|w| std::ptr::eq(&**w, window)) {
                        pending.push(window.retain());
                    }
                }),
            }
        }

        let original = unsafe { ORIGINAL.unwrap() };
        original(this, sel);
    }

    let method = UIWindow::class()
        .instance_method(sel!(makeKeyAndVisible))
        .expect("UIWindow must have makeKeyAndVisible method");

    let overridden = unsafe { mem::transmute::<MakeKeyAndVisible, Imp>(make_key_and_visible) };

    let original = unsafe { method.set_implementation(overridden) };

    let original = unsafe { mem::transmute::<Imp, MakeKeyAndVisible>(original) };

    unsafe { ORIGINAL = Some(original) };
}
