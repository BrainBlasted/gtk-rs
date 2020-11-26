// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Icon;
use crate::MenuModel;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct MenuItem(Object<ffi::GMenuItem>);

    match fn {
        get_type => || ffi::g_menu_item_get_type(),
    }
}

impl MenuItem {
    pub fn new(label: Option<&str>, detailed_action: Option<&str>) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new(
                label.to_glib_none().0,
                detailed_action.to_glib_none().0,
            ))
        }
    }

    pub fn from_model<P: IsA<MenuModel>>(model: &P, item_index: i32) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new_from_model(
                model.as_ref().to_glib_none().0,
                item_index,
            ))
        }
    }

    pub fn new_section<P: IsA<MenuModel>>(label: Option<&str>, section: &P) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new_section(
                label.to_glib_none().0,
                section.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn new_submenu<P: IsA<MenuModel>>(label: Option<&str>, submenu: &P) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new_submenu(
                label.to_glib_none().0,
                submenu.as_ref().to_glib_none().0,
            ))
        }
    }

    //pub fn get_attribute(&self, attribute: &str, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi:g_menu_item_get_attribute() }
    //}

    pub fn get_attribute_value(
        &self,
        attribute: &str,
        expected_type: Option<&glib::VariantTy>,
    ) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_menu_item_get_attribute_value(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                expected_type.to_glib_none().0,
            ))
        }
    }

    pub fn get_link(&self, link: &str) -> Option<MenuModel> {
        unsafe {
            from_glib_full(ffi::g_menu_item_get_link(
                self.to_glib_none().0,
                link.to_glib_none().0,
            ))
        }
    }

    //pub fn set_action_and_target(&self, action: Option<&str>, format_string: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:g_menu_item_set_action_and_target() }
    //}

    pub fn set_action_and_target_value(
        &self,
        action: Option<&str>,
        target_value: Option<&glib::Variant>,
    ) {
        unsafe {
            ffi::g_menu_item_set_action_and_target_value(
                self.to_glib_none().0,
                action.to_glib_none().0,
                target_value.to_glib_none().0,
            );
        }
    }

    //pub fn set_attribute(&self, attribute: &str, format_string: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:g_menu_item_set_attribute() }
    //}

    pub fn set_attribute_value(&self, attribute: &str, value: Option<&glib::Variant>) {
        unsafe {
            ffi::g_menu_item_set_attribute_value(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    pub fn set_detailed_action(&self, detailed_action: &str) {
        unsafe {
            ffi::g_menu_item_set_detailed_action(
                self.to_glib_none().0,
                detailed_action.to_glib_none().0,
            );
        }
    }

    pub fn set_icon<P: IsA<Icon>>(&self, icon: &P) {
        unsafe {
            ffi::g_menu_item_set_icon(self.to_glib_none().0, icon.as_ref().to_glib_none().0);
        }
    }

    pub fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::g_menu_item_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    pub fn set_link<P: IsA<MenuModel>>(&self, link: &str, model: Option<&P>) {
        unsafe {
            ffi::g_menu_item_set_link(
                self.to_glib_none().0,
                link.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_section<P: IsA<MenuModel>>(&self, section: Option<&P>) {
        unsafe {
            ffi::g_menu_item_set_section(
                self.to_glib_none().0,
                section.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_submenu<P: IsA<MenuModel>>(&self, submenu: Option<&P>) {
        unsafe {
            ffi::g_menu_item_set_submenu(
                self.to_glib_none().0,
                submenu.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MenuItem")
    }
}