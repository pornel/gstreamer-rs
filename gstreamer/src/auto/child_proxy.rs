// This file was generated by gir (ef05cf1) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
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
    pub struct ChildProxy(Object<ffi::GstChildProxy>);

    match fn {
        get_type => || ffi::gst_child_proxy_get_type(),
    }
}

unsafe impl Send for ChildProxy {}
unsafe impl Sync for ChildProxy {}

pub trait ChildProxyExt {
    fn child_added<P: IsA<glib::Object>>(&self, child: &P, name: &str);

    fn child_removed<P: IsA<glib::Object>>(&self, child: &P, name: &str);

    //fn get(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_child_by_index(&self, index: u32) -> Option<glib::Object>;

    fn get_child_by_name(&self, name: &str) -> Option<glib::Object>;

    fn get_children_count(&self) -> u32;

    //fn get_property(&self, name: &str, value: /*Ignored*/glib::Value);

    //fn get_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn lookup(&self, name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<glib::Object>;

    //fn set(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn set_property(&self, name: &str, value: /*Ignored*/&glib::Value);

    //fn set_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn connect_child_added<F: Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>(&self, f: F) -> u64;

    fn connect_child_removed<F: Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ChildProxy> + IsA<glib::object::Object>> ChildProxyExt for O {
    fn child_added<P: IsA<glib::Object>>(&self, child: &P, name: &str) {
        unsafe {
            ffi::gst_child_proxy_child_added(self.to_glib_none().0, child.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn child_removed<P: IsA<glib::Object>>(&self, child: &P, name: &str) {
        unsafe {
            ffi::gst_child_proxy_child_removed(self.to_glib_none().0, child.to_glib_none().0, name.to_glib_none().0);
        }
    }

    //fn get(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_child_proxy_get() }
    //}

    fn get_child_by_index(&self, index: u32) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::gst_child_proxy_get_child_by_index(self.to_glib_none().0, index))
        }
    }

    fn get_child_by_name(&self, name: &str) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::gst_child_proxy_get_child_by_name(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_children_count(&self) -> u32 {
        unsafe {
            ffi::gst_child_proxy_get_children_count(self.to_glib_none().0)
        }
    }

    //fn get_property(&self, name: &str, value: /*Ignored*/glib::Value) {
    //    unsafe { TODO: call ffi::gst_child_proxy_get_property() }
    //}

    //fn get_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gst_child_proxy_get_valist() }
    //}

    //fn lookup(&self, name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<glib::Object> {
    //    unsafe { TODO: call ffi::gst_child_proxy_lookup() }
    //}

    //fn set(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_child_proxy_set() }
    //}

    //fn set_property(&self, name: &str, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call ffi::gst_child_proxy_set_property() }
    //}

    //fn set_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gst_child_proxy_set_valist() }
    //}

    fn connect_child_added<F: Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "child-added",
                transmute(child_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_child_removed<F: Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "child-removed",
                transmute(child_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn child_added_trampoline<P>(this: *mut ffi::GstChildProxy, object: *mut gobject_ffi::GObject, name: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<ChildProxy> {
    callback_guard!();
    let f: &Box_<Fn(&P, &glib::Object, &str) + Send + Sync + 'static> = transmute(f);
    f(&ChildProxy::from_glib_none(this).downcast_unchecked(), &from_glib_none(object), &String::from_glib_none(name))
}

unsafe extern "C" fn child_removed_trampoline<P>(this: *mut ffi::GstChildProxy, object: *mut gobject_ffi::GObject, name: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<ChildProxy> {
    callback_guard!();
    let f: &Box_<Fn(&P, &glib::Object, &str) + Send + Sync + 'static> = transmute(f);
    f(&ChildProxy::from_glib_none(this).downcast_unchecked(), &from_glib_none(object), &String::from_glib_none(name))
}