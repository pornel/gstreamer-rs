// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PlayerVideoRenderer(Object<ffi::GstPlayerVideoRenderer, ffi::GstPlayerVideoRendererInterface>);

    match fn {
        get_type => || ffi::gst_player_video_renderer_get_type(),
    }
}

unsafe impl Send for PlayerVideoRenderer {}
unsafe impl Sync for PlayerVideoRenderer {}

pub trait PlayerVideoRendererExt {}

impl<O: IsA<PlayerVideoRenderer>> PlayerVideoRendererExt for O {}
