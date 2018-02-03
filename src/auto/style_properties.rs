// This file was generated by gir (0f1d1c1) from gir-files (77d1f70)
// DO NOT EDIT

use StateFlags;
use StyleProvider;
use SymbolicColor;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct StyleProperties(Object<ffi::GtkStyleProperties, ffi::GtkStylePropertiesClass>): StyleProvider;

    match fn {
        get_type => || ffi::gtk_style_properties_get_type(),
    }
}

impl StyleProperties {
    #[cfg_attr(feature = "v3_16", deprecated)]
    pub fn new() -> StyleProperties {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_style_properties_new())
        }
    }

    //#[cfg_attr(feature = "v3_8", deprecated)]
    //pub fn lookup_property(property_name: &str, parse_func: /*Unknown conversion*//*Unimplemented*/StylePropertyParser, pspec: /*Ignored*/glib::ParamSpec) -> bool {
    //    unsafe { TODO: call ffi::gtk_style_properties_lookup_property() }
    //}

    //#[cfg_attr(feature = "v3_8", deprecated)]
    //pub fn register_property<'a, P: Into<Option<&'a /*Unimplemented*/StylePropertyParser>>, Q: IsA</*Ignored*/glib::ParamSpec>>(parse_func: P, pspec: &Q) {
    //    unsafe { TODO: call ffi::gtk_style_properties_register_property() }
    //}
}

#[cfg_attr(feature = "v3_16", deprecated)]
impl Default for StyleProperties {
    fn default() -> Self {
        Self::new()
    }
}

pub trait StylePropertiesExt {
    #[cfg_attr(feature = "v3_16", deprecated)]
    fn clear(&self);

    //#[cfg_attr(feature = "v3_16", deprecated)]
    //fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn get_property(&self, property: &str, state: StateFlags) -> Option<glib::Value>;

    //#[cfg_attr(feature = "v3_16", deprecated)]
    //fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    #[cfg_attr(feature = "v3_8", deprecated)]
    fn lookup_color(&self, name: &str) -> Option<SymbolicColor>;

    #[cfg_attr(feature = "v3_8", deprecated)]
    fn map_color(&self, name: &str, color: &SymbolicColor);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn merge(&self, props_to_merge: &StyleProperties, replace: bool);

    //#[cfg_attr(feature = "v3_16", deprecated)]
    //fn set(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn set_property(&self, property: &str, state: StateFlags, value: &glib::Value);

    //#[cfg_attr(feature = "v3_16", deprecated)]
    //fn set_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn unset_property(&self, property: &str, state: StateFlags);
}

impl<O: IsA<StyleProperties>> StylePropertiesExt for O {
    fn clear(&self) {
        unsafe {
            ffi::gtk_style_properties_clear(self.to_glib_none().0);
        }
    }

    //fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_style_properties_get() }
    //}

    fn get_property(&self, property: &str, state: StateFlags) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret = from_glib(ffi::gtk_style_properties_get_property(self.to_glib_none().0, property.to_glib_none().0, state.to_glib(), value.to_glib_none_mut().0));
            if ret { Some(value) } else { None }
        }
    }

    //fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_style_properties_get_valist() }
    //}

    fn lookup_color(&self, name: &str) -> Option<SymbolicColor> {
        unsafe {
            from_glib_none(ffi::gtk_style_properties_lookup_color(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn map_color(&self, name: &str, color: &SymbolicColor) {
        unsafe {
            ffi::gtk_style_properties_map_color(self.to_glib_none().0, name.to_glib_none().0, color.to_glib_none().0);
        }
    }

    fn merge(&self, props_to_merge: &StyleProperties, replace: bool) {
        unsafe {
            ffi::gtk_style_properties_merge(self.to_glib_none().0, props_to_merge.to_glib_none().0, replace.to_glib());
        }
    }

    //fn set(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_style_properties_set() }
    //}

    fn set_property(&self, property: &str, state: StateFlags, value: &glib::Value) {
        unsafe {
            ffi::gtk_style_properties_set_property(self.to_glib_none().0, property.to_glib_none().0, state.to_glib(), value.to_glib_none().0);
        }
    }

    //fn set_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_style_properties_set_valist() }
    //}

    fn unset_property(&self, property: &str, state: StateFlags) {
        unsafe {
            ffi::gtk_style_properties_unset_property(self.to_glib_none().0, property.to_glib_none().0, state.to_glib());
        }
    }
}
