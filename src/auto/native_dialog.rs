// This file was generated by gir (0f1d1c1) from gir-files (77d1f70)
// DO NOT EDIT

#[cfg(any(feature = "v3_20", feature = "dox"))]
use Window;
use ffi;
use glib;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use libc;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct NativeDialog(Object<ffi::GtkNativeDialog, ffi::GtkNativeDialogClass>);

    match fn {
        get_type => || ffi::gtk_native_dialog_get_type(),
    }
}

pub trait NativeDialogExt {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn destroy(&self);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_modal(&self) -> bool;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_title(&self) -> Option<String>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_transient_for(&self) -> Option<Window>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_visible(&self) -> bool;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn hide(&self);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn run(&self) -> i32;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_modal(&self, modal: bool);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_title(&self, title: &str);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_transient_for<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn show(&self);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_visible(&self, visible: bool);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_response<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_transient_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NativeDialog> + IsA<glib::object::Object>> NativeDialogExt for O {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn destroy(&self) {
        unsafe {
            ffi::gtk_native_dialog_destroy(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_native_dialog_get_modal(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_native_dialog_get_title(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_transient_for(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_native_dialog_get_transient_for(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_native_dialog_get_visible(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn hide(&self) {
        unsafe {
            ffi::gtk_native_dialog_hide(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn run(&self) -> i32 {
        unsafe {
            ffi::gtk_native_dialog_run(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_native_dialog_set_modal(self.to_glib_none().0, modal.to_glib());
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_native_dialog_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_transient_for<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q) {
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            ffi::gtk_native_dialog_set_transient_for(self.to_glib_none().0, parent.0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn show(&self) {
        unsafe {
            ffi::gtk_native_dialog_show(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_visible(&self, visible: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "visible".to_glib_none().0, Value::from(&visible).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_response<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "response",
                transmute(response_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::modal",
                transmute(notify_modal_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::title",
                transmute(notify_title_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_transient_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::transient-for",
                transmute(notify_transient_for_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible",
                transmute(notify_visible_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn response_trampoline<P>(this: *mut ffi::GtkNativeDialog, response_id: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<NativeDialog> {
    callback_guard!();
    let f: &&(Fn(&P, i32) + 'static) = transmute(f);
    f(&NativeDialog::from_glib_borrow(this).downcast_unchecked(), response_id)
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_modal_trampoline<P>(this: *mut ffi::GtkNativeDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NativeDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NativeDialog::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_title_trampoline<P>(this: *mut ffi::GtkNativeDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NativeDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NativeDialog::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_transient_for_trampoline<P>(this: *mut ffi::GtkNativeDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NativeDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NativeDialog::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_visible_trampoline<P>(this: *mut ffi::GtkNativeDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NativeDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NativeDialog::from_glib_borrow(this).downcast_unchecked())
}
