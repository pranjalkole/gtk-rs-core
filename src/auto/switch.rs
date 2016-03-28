// This file was generated by gir (becf3b4) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Buildable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Switch(Object<ffi::GtkSwitch>): Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_switch_get_type(),
    }
}

impl Switch {
    pub fn new() -> Switch {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_switch_new()).downcast_unchecked()
        }
    }

    pub fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_switch_get_active(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_state(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_switch_get_state(self.to_glib_none().0))
        }
    }

    pub fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_switch_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_state(&self, state: bool) {
        unsafe {
            ffi::gtk_switch_set_state(self.to_glib_none().0, state.to_glib());
        }
    }
}
