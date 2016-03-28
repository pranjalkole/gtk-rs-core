// This file was generated by gir (becf3b4) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct CellEditable(Object<ffi::GtkCellEditable>): Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_cell_editable_get_type(),
    }
}

pub trait CellEditableExt {
    fn editing_done(&self);

    fn remove_widget(&self);

    //fn start_editing(&self, event: /*Unknown conversion*//*Unimplemented*/Event);
}

impl<O: IsA<CellEditable>> CellEditableExt for O {
    fn editing_done(&self) {
        unsafe {
            ffi::gtk_cell_editable_editing_done(self.to_glib_none().0);
        }
    }

    fn remove_widget(&self) {
        unsafe {
            ffi::gtk_cell_editable_remove_widget(self.to_glib_none().0);
        }
    }

    //fn start_editing(&self, event: /*Unknown conversion*//*Unimplemented*/Event) {
    //    unsafe { TODO: call ffi::gtk_cell_editable_start_editing() }
    //}
}
