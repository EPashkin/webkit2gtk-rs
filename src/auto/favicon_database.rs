// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use cairo;
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
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FaviconDatabase(Object<ffi::WebKitFaviconDatabase, ffi::WebKitFaviconDatabaseClass>);

    match fn {
        get_type => || ffi::webkit_favicon_database_get_type(),
    }
}

pub trait FaviconDatabaseExt: Sized {
    fn clear(&self);

    fn get_favicon<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<cairo::Surface, Error>) + Send + 'static>(&self, page_uri: &str, cancellable: P, callback: Q);

    #[cfg(feature = "futures")]
    fn get_favicon_future(&self, page_uri: &str) -> Box_<futures_core::Future<Item = (Self, cairo::Surface), Error = (Self, Error)>>;

    fn get_favicon_uri(&self, page_uri: &str) -> Option<String>;

    fn connect_favicon_changed<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FaviconDatabase> + IsA<glib::object::Object> + Clone + 'static> FaviconDatabaseExt for O {
    fn clear(&self) {
        unsafe {
            ffi::webkit_favicon_database_clear(self.to_glib_none().0);
        }
    }

    fn get_favicon<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<cairo::Surface, Error>) + Send + 'static>(&self, page_uri: &str, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn get_favicon_trampoline<Q: FnOnce(Result<cairo::Surface, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_favicon_database_get_favicon_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_favicon_trampoline::<Q>;
        unsafe {
            ffi::webkit_favicon_database_get_favicon(self.to_glib_none().0, page_uri.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn get_favicon_future(&self, page_uri: &str) -> Box_<futures_core::Future<Item = (Self, cairo::Surface), Error = (Self, Error)>> {
        use gio::GioFuture;
        use fragile::Fragile;

        let page_uri = String::from(page_uri);
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.get_favicon(
                 &page_uri,
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

    fn get_favicon_uri(&self, page_uri: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_favicon_database_get_favicon_uri(self.to_glib_none().0, page_uri.to_glib_none().0))
        }
    }

    fn connect_favicon_changed<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "favicon-changed",
                transmute(favicon_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn favicon_changed_trampoline<P>(this: *mut ffi::WebKitFaviconDatabase, page_uri: *mut libc::c_char, favicon_uri: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<FaviconDatabase> {
    let f: &&(Fn(&P, &str, &str) + 'static) = transmute(f);
    f(&FaviconDatabase::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(page_uri), &String::from_glib_none(favicon_uri))
}
