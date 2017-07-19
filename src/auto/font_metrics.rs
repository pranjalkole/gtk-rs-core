// This file was generated by gir (a4dcfea) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FontMetrics(Shared<ffi::PangoFontMetrics>);

    match fn {
        ref => |ptr| ffi::pango_font_metrics_ref(ptr),
        unref => |ptr| ffi::pango_font_metrics_unref(ptr),
        get_type => || ffi::pango_font_metrics_get_type(),
    }
}

impl FontMetrics {
    pub fn new() -> FontMetrics {
        unsafe {
            from_glib_full(ffi::pango_font_metrics_new())
        }
    }

    pub fn get_approximate_char_width(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_approximate_char_width(self.to_glib_none().0)
        }
    }

    pub fn get_approximate_digit_width(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_approximate_digit_width(self.to_glib_none().0)
        }
    }

    pub fn get_ascent(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_ascent(self.to_glib_none().0)
        }
    }

    pub fn get_descent(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_descent(self.to_glib_none().0)
        }
    }

    pub fn get_strikethrough_position(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_strikethrough_position(self.to_glib_none().0)
        }
    }

    pub fn get_strikethrough_thickness(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_strikethrough_thickness(self.to_glib_none().0)
        }
    }

    pub fn get_underline_position(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_underline_position(self.to_glib_none().0)
        }
    }

    pub fn get_underline_thickness(&self) -> i32 {
        unsafe {
            ffi::pango_font_metrics_get_underline_thickness(self.to_glib_none().0)
        }
    }
}
