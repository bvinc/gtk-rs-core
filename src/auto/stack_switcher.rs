// This file was generated by gir (https://github.com/gtk-rs/gir @ ea993ed)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Box;
use Buildable;
use Container;
use Orientable;
use Stack;
use Widget;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
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
    pub struct StackSwitcher(Object<ffi::GtkStackSwitcher, ffi::GtkStackSwitcherClass>): Box, Container, Widget, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_stack_switcher_get_type(),
    }
}

impl StackSwitcher {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn new() -> StackSwitcher {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_stack_switcher_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
impl Default for StackSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

pub trait StackSwitcherExt {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_stack(&self) -> Option<Stack>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_stack<'a, P: Into<Option<&'a Stack>>>(&self, stack: P);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_icon_size(&self) -> i32;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_icon_size(&self, icon_size: i32);

    fn get_property_stack(&self) -> Option<Stack>;

    fn set_property_stack(&self, stack: Option<&Stack>);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StackSwitcher> + IsA<glib::object::Object>> StackSwitcherExt for O {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_stack(&self) -> Option<Stack> {
        unsafe {
            from_glib_none(ffi::gtk_stack_switcher_get_stack(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_stack<'a, P: Into<Option<&'a Stack>>>(&self, stack: P) {
        let stack = stack.into();
        let stack = stack.to_glib_none();
        unsafe {
            ffi::gtk_stack_switcher_set_stack(self.to_glib_none().0, stack.0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_icon_size(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "icon-size".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_icon_size(&self, icon_size: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-size".to_glib_none().0, Value::from(&icon_size).to_glib_none().0);
        }
    }

    fn get_property_stack(&self) -> Option<Stack> {
        unsafe {
            let mut value = Value::from_type(<Stack as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stack".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_stack(&self, stack: Option<&Stack>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stack".to_glib_none().0, Value::from(stack).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-size",
                transmute(notify_icon_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stack",
                transmute(notify_stack_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_icon_size_trampoline<P>(this: *mut ffi::GtkStackSwitcher, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StackSwitcher> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StackSwitcher::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stack_trampoline<P>(this: *mut ffi::GtkStackSwitcher, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StackSwitcher> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StackSwitcher::from_glib_borrow(this).downcast_unchecked())
}
