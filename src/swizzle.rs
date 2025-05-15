use crate::AppDelegateCall;
use crate::channel::send_event;

use objc2::{
    ClassType, MainThreadMarker, MainThreadOnly, define_class, msg_send,
    rc::{Allocated, Retained},
    runtime::{AnyObject, Imp, NSObjectProtocol, ProtocolObject, Sel},
    sel,
};
use objc2_foundation::{NSDictionary, NSURL};
use objc2_ui_kit::{
    UIApplication, UIApplicationDelegate, UIApplicationOpenURLOptionsKey, UIResponder,
};
use std::mem;

define_class!(
    #[unsafe(super(UIResponder))]
    #[thread_kind = MainThreadOnly]
    struct AppDelegate;

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl UIApplicationDelegate for AppDelegate {
        #[allow(non_snake_case)]
        #[unsafe(method(application:openURL:options:))]
        unsafe fn application_openURL_options(
            &self,
            _app: &UIApplication,
            url: &NSURL,
            _options: &NSDictionary<UIApplicationOpenURLOptionsKey, AnyObject>,
        ) -> bool {
            let url = unsafe { url.absoluteString().unwrap().to_string() };

            bevy_log::debug!("url open: {url}");

            send_event(AppDelegateCall::OpenURL(url));

            true
        }
    }
);

define_class!(
    #[unsafe(super(UIApplication))]
    #[name = "SpecialAppNameThatGoesInPlist"]
    struct IosApp;

    impl IosApp {
        #[unsafe(method_id(init))]
        fn init(this: Allocated<Self>) -> Retained<Self> {
            let this = this.set_ivars(());
            let app: Retained<Self> = unsafe { objc2::msg_send![super(this), init] };
            install_global_app_delegate(&app);
            app
        }
    }
);

impl AppDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        unsafe { msg_send![super(mtm.alloc().set_ivars(())), init] }
    }
}

fn install_global_app_delegate(app: &UIApplication) {
    let mtm = MainThreadMarker::new()
        .expect("install_global_app_delegate must be called on the main thread");

    let app_delegate = AppDelegate::new(mtm);

    unsafe {
        app.setDelegate(Some(ProtocolObject::from_ref(&*app_delegate)));
    }

    // We need the app delegate not to be deallocated.
    // It will live for the lifetime of the application.
    std::mem::forget(app_delegate);

    bevy_log::debug!("AppDelegate registererd");
}

/// Swizzle `-[UIApplication init]` to install our application delegate in there.
pub(crate) fn swizzle() {
    type SetDelegate = extern "C-unwind" fn(*mut UIApplication, Sel) -> *mut UIApplication;

    static mut ORIGINAL: Option<SetDelegate> = None;

    extern "C-unwind" fn init(this: *mut UIApplication, sel: Sel) -> *mut UIApplication {
        let original = unsafe { ORIGINAL.unwrap() };
        let this = original(this, sel);
        let app = unsafe { &*this };
        install_global_app_delegate(&app);
        this
    }

    let _mtm = MainThreadMarker::new().unwrap();
    let class = UIApplication::class();

    let method = class
        .instance_method(sel!(init))
        .expect("UIApplication must have init method");

    let overridden = unsafe { mem::transmute::<SetDelegate, Imp>(init) };

    let original = unsafe { method.set_implementation(overridden) };

    let original = unsafe { mem::transmute::<Imp, SetDelegate>(original) };

    unsafe { ORIGINAL = Some(original) };
}
