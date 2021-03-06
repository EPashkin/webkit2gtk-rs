// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CookieAcceptPolicy;
use CookiePersistentStorage;
use Error;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use gio;
use gio_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CookieManager(Object<ffi::WebKitCookieManager, ffi::WebKitCookieManagerClass>);

    match fn {
        get_type => || ffi::webkit_cookie_manager_get_type(),
    }
}

pub trait CookieManagerExt: Sized {
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn add_cookie<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cookie: /*Ignored*/&mut soup::Cookie, cancellable: P, callback: Q);

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn add_cookie_future(&self, cookie: /*Ignored*/&mut soup::Cookie) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    #[cfg_attr(feature = "v2_16", deprecated)]
    fn delete_all_cookies(&self);

    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn delete_cookie<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cookie: /*Ignored*/&mut soup::Cookie, cancellable: P, callback: Q);

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn delete_cookie_future(&self, cookie: /*Ignored*/&mut soup::Cookie) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    #[cfg_attr(feature = "v2_16", deprecated)]
    fn delete_cookies_for_domain(&self, domain: &str);

    fn get_accept_policy<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<CookieAcceptPolicy, Error>) + Send + 'static>(&self, cancellable: P, callback: Q);

    #[cfg(feature = "futures")]
    fn get_accept_policy_future(&self) -> Box_<futures_core::Future<Item = (Self, CookieAcceptPolicy), Error = (Self, Error)>>;

    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn get_cookies<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result</*Ignored*/Vec<soup::Cookie>, Error>) + Send + 'static>(&self, uri: &str, cancellable: P, callback: Q);

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn get_cookies_future(&self, uri: &str) -> Box_<futures_core::Future<Item = (Self, /*Ignored*/Vec<soup::Cookie>), Error = (Self, Error)>>;

    #[cfg_attr(feature = "v2_16", deprecated)]
    fn get_domains_with_cookies<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<Vec<String>, Error>) + Send + 'static>(&self, cancellable: P, callback: Q);

    #[cfg_attr(feature = "v2_16", deprecated)]
    #[cfg(feature = "futures")]
    fn get_domains_with_cookies_future(&self) -> Box_<futures_core::Future<Item = (Self, Vec<String>), Error = (Self, Error)>>;

    fn set_accept_policy(&self, policy: CookieAcceptPolicy);

    fn set_persistent_storage(&self, filename: &str, storage: CookiePersistentStorage);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CookieManager> + IsA<glib::object::Object> + Clone + 'static> CookieManagerExt for O {
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn add_cookie<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cookie: /*Ignored*/&mut soup::Cookie, cancellable: P, callback: Q) {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_add_cookie() }
    //}

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn add_cookie_future(&self, cookie: /*Ignored*/&mut soup::Cookie) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let cookie = cookie.clone();
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    let obj_clone = Fragile::new(obj.clone());
        //    obj.add_cookie(
        //         &cookie,
        //         Some(&cancellable),
        //         move |res| {
        //             let obj = obj_clone.into_inner();
        //             let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
        //             let _ = send.into_inner().send(res);
        //         },
        //    );

        //    cancellable
        //})
    //}

    fn delete_all_cookies(&self) {
        unsafe {
            ffi::webkit_cookie_manager_delete_all_cookies(self.to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn delete_cookie<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cookie: /*Ignored*/&mut soup::Cookie, cancellable: P, callback: Q) {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_delete_cookie() }
    //}

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn delete_cookie_future(&self, cookie: /*Ignored*/&mut soup::Cookie) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let cookie = cookie.clone();
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    let obj_clone = Fragile::new(obj.clone());
        //    obj.delete_cookie(
        //         &cookie,
        //         Some(&cancellable),
        //         move |res| {
        //             let obj = obj_clone.into_inner();
        //             let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
        //             let _ = send.into_inner().send(res);
        //         },
        //    );

        //    cancellable
        //})
    //}

    fn delete_cookies_for_domain(&self, domain: &str) {
        unsafe {
            ffi::webkit_cookie_manager_delete_cookies_for_domain(self.to_glib_none().0, domain.to_glib_none().0);
        }
    }

    fn get_accept_policy<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<CookieAcceptPolicy, Error>) + Send + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn get_accept_policy_trampoline<Q: FnOnce(Result<CookieAcceptPolicy, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_cookie_manager_get_accept_policy_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_accept_policy_trampoline::<Q>;
        unsafe {
            ffi::webkit_cookie_manager_get_accept_policy(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn get_accept_policy_future(&self) -> Box_<futures_core::Future<Item = (Self, CookieAcceptPolicy), Error = (Self, Error)>> {
        use gio::GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.get_accept_policy(
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn get_cookies<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result</*Ignored*/Vec<soup::Cookie>, Error>) + Send + 'static>(&self, uri: &str, cancellable: P, callback: Q) {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_get_cookies() }
    //}

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //fn get_cookies_future(&self, uri: &str) -> Box_<futures_core::Future<Item = (Self, /*Ignored*/Vec<soup::Cookie>), Error = (Self, Error)>> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let uri = String::from(uri);
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    let obj_clone = Fragile::new(obj.clone());
        //    obj.get_cookies(
        //         &uri,
        //         Some(&cancellable),
        //         move |res| {
        //             let obj = obj_clone.into_inner();
        //             let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
        //             let _ = send.into_inner().send(res);
        //         },
        //    );

        //    cancellable
        //})
    //}

    fn get_domains_with_cookies<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<Vec<String>, Error>) + Send + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn get_domains_with_cookies_trampoline<Q: FnOnce(Result<Vec<String>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_cookie_manager_get_domains_with_cookies_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_domains_with_cookies_trampoline::<Q>;
        unsafe {
            ffi::webkit_cookie_manager_get_domains_with_cookies(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn get_domains_with_cookies_future(&self) -> Box_<futures_core::Future<Item = (Self, Vec<String>), Error = (Self, Error)>> {
        use gio::GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.get_domains_with_cookies(
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn set_accept_policy(&self, policy: CookieAcceptPolicy) {
        unsafe {
            ffi::webkit_cookie_manager_set_accept_policy(self.to_glib_none().0, policy.to_glib());
        }
    }

    fn set_persistent_storage(&self, filename: &str, storage: CookiePersistentStorage) {
        unsafe {
            ffi::webkit_cookie_manager_set_persistent_storage(self.to_glib_none().0, filename.to_glib_none().0, storage.to_glib());
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::WebKitCookieManager, f: glib_ffi::gpointer)
where P: IsA<CookieManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CookieManager::from_glib_borrow(this).downcast_unchecked())
}
