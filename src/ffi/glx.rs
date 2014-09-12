/*
 * This file generated automatically from glx.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(non_camel_case_types)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi;
use ffi::xproto;

pub static GLX_MAJOR_VERSION : c_uint = 1;
pub static GLX_MINOR_VERSION : c_uint = 3;

pub type pixmap = u32;
/**
 * @brief pixmap_iterator
 **/
pub struct pixmap_iterator {
    data : *mut pixmap,
    rem  : c_int,
    index: c_int
}


pub type context = u32;
/**
 * @brief context_iterator
 **/
pub struct context_iterator {
    data : *mut context,
    rem  : c_int,
    index: c_int
}


pub type pbuffer = u32;
/**
 * @brief pbuffer_iterator
 **/
pub struct pbuffer_iterator {
    data : *mut pbuffer,
    rem  : c_int,
    index: c_int
}


pub type window = u32;
/**
 * @brief window_iterator
 **/
pub struct window_iterator {
    data : *mut window,
    rem  : c_int,
    index: c_int
}


pub type fbconfig = u32;
/**
 * @brief fbconfig_iterator
 **/
pub struct fbconfig_iterator {
    data : *mut fbconfig,
    rem  : c_int,
    index: c_int
}


pub type drawable = u32;
/**
 * @brief drawable_iterator
 **/
pub struct drawable_iterator {
    data : *mut drawable,
    rem  : c_int,
    index: c_int
}


pub type float32 = f32;
/**
 * @brief float32_iterator
 **/
pub struct float32_iterator {
    data : *mut float32,
    rem  : c_int,
    index: c_int
}


pub type float64 = f64;
/**
 * @brief float64_iterator
 **/
pub struct float64_iterator {
    data : *mut float64,
    rem  : c_int,
    index: c_int
}


pub type bool32 = u32;
/**
 * @brief bool32_iterator
 **/
pub struct bool32_iterator {
    data : *mut bool32,
    rem  : c_int,
    index: c_int
}


pub type context_tag = u32;
/**
 * @brief context_tag_iterator
 **/
pub struct context_tag_iterator {
    data : *mut context_tag,
    rem  : c_int,
    index: c_int
}



pub struct generic_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16,
    bad_value :       u32,
    minor_opcode :    u16,
    major_opcode :    u8,
    pad0 :            [u8,..21]
}



pub type bad_context_error  = generic_error;


pub type bad_context_state_error  = generic_error;


pub type bad_drawable_error  = generic_error;


pub type bad_pixmap_error  = generic_error;


pub type bad_context_tag_error  = generic_error;


pub type bad_current_window_error  = generic_error;


pub type bad_render_request_error  = generic_error;


pub type bad_large_request_error  = generic_error;


pub type unsupported_private_request_error  = generic_error;


pub type bad_fb_config_error  = generic_error;


pub type bad_pbuffer_error  = generic_error;


pub type bad_current_drawable_error  = generic_error;


pub type bad_window_error  = generic_error;


pub type glx_bad_profile_arb_error  = generic_error;


pub struct pbuffer_clobber_event {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    event_type :      u16,
    draw_type :       u16,
    drawable :        drawable,
    b_mask :          u32,
    aux_buffer :      u16,
    x :               u16,
    y :               u16,
    width :           u16,
    height :          u16,
    count :           u16,
    pad1 :            [u8,..4]
}



pub struct render_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}



pub struct render_large_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    context_tag :     context_tag,
    request_num :     u16,
    request_total :   u16,
    data_len :        u32
}



pub struct create_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context,
    visual :         ffi::xproto::visualid,
    screen :         u32,
    share_list :     context,
    is_direct :      u8,
    pad0 :           [u8,..3]
}



pub struct destroy_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context
}


pub struct make_current_cookie {
    sequence : c_uint
}


pub struct make_current_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    drawable :          drawable,
    context :           context,
    old_context_tag :   context_tag
}


pub struct make_current_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_tag :     context_tag,
    pad1 :            [u8,..20]
}


pub struct is_direct_cookie {
    sequence : c_uint
}


pub struct is_direct_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context
}


pub struct is_direct_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    is_direct :       u8,
    pad1 :            [u8,..23]
}


pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u32,
    minor_version :   u32,
    pad1 :            [u8,..16]
}



pub struct wait_gl_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}



pub struct wait_x_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}



pub struct copy_context_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    src :               context,
    dest :              context,
    mask :              u32,
    src_context_tag :   context_tag
}



pub struct swap_buffers_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    drawable :       drawable
}



pub struct use_x_font_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    font :           ffi::xproto::font,
    first :          u32,
    count :          u32,
    list_base :      u32
}



pub struct create_glx_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    visual :         ffi::xproto::visualid,
    pixmap :         ffi::xproto::pixmap,
    glx_pixmap :     pixmap
}


pub struct get_visual_configs_cookie {
    sequence : c_uint
}


pub struct get_visual_configs_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}


pub struct get_visual_configs_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    num_visuals :      u32,
    num_properties :   u32,
    pad1 :             [u8,..16]
}



pub struct destroy_glx_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glx_pixmap :     pixmap
}



pub struct vendor_private_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    vendor_code :    u32,
    context_tag :    context_tag
}


pub struct vendor_private_with_reply_cookie {
    sequence : c_uint
}


pub struct vendor_private_with_reply_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    vendor_code :    u32,
    context_tag :    context_tag
}


pub struct vendor_private_with_reply_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    retval :          u32,
    data1 :           [u8,..24]
}


pub struct query_extensions_string_cookie {
    sequence : c_uint
}


pub struct query_extensions_string_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}


pub struct query_extensions_string_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    pad2 :            [u8,..16]
}


pub struct query_server_string_cookie {
    sequence : c_uint
}


pub struct query_server_string_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    name :           u32
}


pub struct query_server_string_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    str_len :         u32,
    pad2 :            [u8,..16]
}



pub struct client_info_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32,
    str_len :         u32
}


pub struct get_fb_configs_cookie {
    sequence : c_uint
}


pub struct get_fb_configs_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32
}


pub struct get_fb_configs_reply {
    response_type :    u8,
    pad0 :             u8,
    sequence :         u16,
    length :           u32,
    num_FB_configs :   u32,
    num_properties :   u32,
    pad1 :             [u8,..16]
}



pub struct create_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    fbconfig :       fbconfig,
    pixmap :         ffi::xproto::pixmap,
    glx_pixmap :     pixmap,
    num_attribs :    u32
}



pub struct destroy_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glx_pixmap :     pixmap
}



pub struct create_new_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context,
    fbconfig :       fbconfig,
    screen :         u32,
    render_type :    u32,
    share_list :     context,
    is_direct :      u8,
    pad0 :           [u8,..3]
}


pub struct query_context_cookie {
    sequence : c_uint
}


pub struct query_context_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context
}


pub struct query_context_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_attribs :     u32,
    pad1 :            [u8,..20]
}


pub struct make_context_current_cookie {
    sequence : c_uint
}


pub struct make_context_current_request {
    major_opcode :      u8,
    minor_opcode :      u8,
    length :            u16,
    old_context_tag :   context_tag,
    drawable :          drawable,
    read_drawable :     drawable,
    context :           context
}


pub struct make_context_current_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    context_tag :     context_tag,
    pad1 :            [u8,..20]
}



pub struct create_pbuffer_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    fbconfig :       fbconfig,
    pbuffer :        pbuffer,
    num_attribs :    u32
}



pub struct destroy_pbuffer_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    pbuffer :        pbuffer
}


pub struct get_drawable_attributes_cookie {
    sequence : c_uint
}


pub struct get_drawable_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       drawable
}


pub struct get_drawable_attributes_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    num_attribs :     u32,
    pad1 :            [u8,..20]
}



pub struct change_drawable_attributes_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    drawable :       drawable,
    num_attribs :    u32
}



pub struct create_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    screen :         u32,
    fbconfig :       fbconfig,
    window :         ffi::xproto::window,
    glx_window :     window,
    num_attribs :    u32
}



pub struct delete_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    glxwindow :      window
}



pub struct set_client_info_arb_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32,
    num_versions :    u32,
    gl_str_len :      u32,
    glx_str_len :     u32
}



pub struct create_context_attribs_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context :        context,
    fbconfig :       fbconfig,
    screen :         u32,
    share_list :     context,
    is_direct :      u8,
    pad0 :           [u8,..3],
    num_attribs :    u32
}



pub struct set_client_info_2arb_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    major_version :   u32,
    minor_version :   u32,
    num_versions :    u32,
    gl_str_len :      u32,
    glx_str_len :     u32
}



pub struct new_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    list :           u32,
    mode :           u32
}



pub struct end_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}



pub struct delete_lists_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    list :           u32,
    range :          i32
}


pub struct gen_lists_cookie {
    sequence : c_uint
}


pub struct gen_lists_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    range :          i32
}


pub struct gen_lists_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         u32
}



pub struct feedback_buffer_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    size :           i32,
    type_ :          i32
}



pub struct select_buffer_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    size :           i32
}


pub struct render_mode_cookie {
    sequence : c_uint
}


pub struct render_mode_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    mode :           u32
}


pub struct render_mode_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         u32,
    n :               u32,
    new_mode :        u32,
    pad1 :            [u8,..12]
}


pub struct finish_cookie {
    sequence : c_uint
}


pub struct finish_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}


pub struct finish_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32
}



pub struct pixel_storef_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          u32,
    datum :          float32
}



pub struct pixel_storei_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          u32,
    datum :          i32
}


pub struct read_pixels_cookie {
    sequence : c_uint
}


pub struct read_pixels_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    x :              i32,
    y :              i32,
    width :          i32,
    height :         i32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8,
    lsb_first :      u8
}


pub struct read_pixels_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}


pub struct get_booleanv_cookie {
    sequence : c_uint
}


pub struct get_booleanv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          i32
}


pub struct get_booleanv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           u8,
    pad2 :            [u8,..15]
}


pub struct get_clip_plane_cookie {
    sequence : c_uint
}


pub struct get_clip_plane_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    plane :          i32
}


pub struct get_clip_plane_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}


pub struct get_doublev_cookie {
    sequence : c_uint
}


pub struct get_doublev_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          u32
}


pub struct get_doublev_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float64,
    pad2 :            [u8,..8]
}


pub struct get_error_cookie {
    sequence : c_uint
}


pub struct get_error_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}


pub struct get_error_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    error :           i32
}


pub struct get_floatv_cookie {
    sequence : c_uint
}


pub struct get_floatv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          u32
}


pub struct get_floatv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_integerv_cookie {
    sequence : c_uint
}


pub struct get_integerv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    pname :          u32
}


pub struct get_integerv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_lightfv_cookie {
    sequence : c_uint
}


pub struct get_lightfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    light :          u32,
    pname :          u32
}


pub struct get_lightfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_lightiv_cookie {
    sequence : c_uint
}


pub struct get_lightiv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    light :          u32,
    pname :          u32
}


pub struct get_lightiv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_mapdv_cookie {
    sequence : c_uint
}


pub struct get_mapdv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    query :          u32
}


pub struct get_mapdv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float64,
    pad2 :            [u8,..8]
}


pub struct get_mapfv_cookie {
    sequence : c_uint
}


pub struct get_mapfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    query :          u32
}


pub struct get_mapfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_mapiv_cookie {
    sequence : c_uint
}


pub struct get_mapiv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    query :          u32
}


pub struct get_mapiv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_materialfv_cookie {
    sequence : c_uint
}


pub struct get_materialfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    face :           u32,
    pname :          u32
}


pub struct get_materialfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_materialiv_cookie {
    sequence : c_uint
}


pub struct get_materialiv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    face :           u32,
    pname :          u32
}


pub struct get_materialiv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_pixel_mapfv_cookie {
    sequence : c_uint
}


pub struct get_pixel_mapfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    map :            u32
}


pub struct get_pixel_mapfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_pixel_mapuiv_cookie {
    sequence : c_uint
}


pub struct get_pixel_mapuiv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    map :            u32
}


pub struct get_pixel_mapuiv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           u32,
    pad2 :            [u8,..12]
}


pub struct get_pixel_mapusv_cookie {
    sequence : c_uint
}


pub struct get_pixel_mapusv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    map :            u32
}


pub struct get_pixel_mapusv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           u16,
    pad2 :            [u8,..16]
}


pub struct get_polygon_stipple_cookie {
    sequence : c_uint
}


pub struct get_polygon_stipple_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    lsb_first :      u8
}


pub struct get_polygon_stipple_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}


pub struct get_string_cookie {
    sequence : c_uint
}


pub struct get_string_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    name :           u32
}


pub struct get_string_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    pad2 :            [u8,..16]
}


pub struct get_tex_envfv_cookie {
    sequence : c_uint
}


pub struct get_tex_envfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_tex_envfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_tex_enviv_cookie {
    sequence : c_uint
}


pub struct get_tex_enviv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_tex_enviv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_tex_gendv_cookie {
    sequence : c_uint
}


pub struct get_tex_gendv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    coord :          u32,
    pname :          u32
}


pub struct get_tex_gendv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float64,
    pad2 :            [u8,..8]
}


pub struct get_tex_genfv_cookie {
    sequence : c_uint
}


pub struct get_tex_genfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    coord :          u32,
    pname :          u32
}


pub struct get_tex_genfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_tex_geniv_cookie {
    sequence : c_uint
}


pub struct get_tex_geniv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    coord :          u32,
    pname :          u32
}


pub struct get_tex_geniv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_tex_image_cookie {
    sequence : c_uint
}


pub struct get_tex_image_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    level :          i32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8
}


pub struct get_tex_image_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    width :           i32,
    height :          i32,
    depth :           i32,
    pad2 :            [u8,..4]
}


pub struct get_tex_parameterfv_cookie {
    sequence : c_uint
}


pub struct get_tex_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_tex_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_tex_parameteriv_cookie {
    sequence : c_uint
}


pub struct get_tex_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_tex_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_tex_level_parameterfv_cookie {
    sequence : c_uint
}


pub struct get_tex_level_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    level :          i32,
    pname :          u32
}


pub struct get_tex_level_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_tex_level_parameteriv_cookie {
    sequence : c_uint
}


pub struct get_tex_level_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    level :          i32,
    pname :          u32
}


pub struct get_tex_level_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct is_list_cookie {
    sequence : c_uint
}


pub struct is_list_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    list :           u32
}


pub struct is_list_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         bool32
}



pub struct flush_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag
}


pub struct are_textures_resident_cookie {
    sequence : c_uint
}


pub struct are_textures_resident_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    n :              i32
}


pub struct are_textures_resident_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         bool32,
    pad1 :            [u8,..20]
}



pub struct delete_textures_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    n :              i32
}


pub struct gen_textures_cookie {
    sequence : c_uint
}


pub struct gen_textures_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    n :              i32
}


pub struct gen_textures_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}


pub struct is_texture_cookie {
    sequence : c_uint
}


pub struct is_texture_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    texture :        u32
}


pub struct is_texture_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         bool32
}


pub struct get_color_table_cookie {
    sequence : c_uint
}


pub struct get_color_table_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8
}


pub struct get_color_table_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    width :           i32,
    pad2 :            [u8,..12]
}


pub struct get_color_table_parameterfv_cookie {
    sequence : c_uint
}


pub struct get_color_table_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_color_table_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_color_table_parameteriv_cookie {
    sequence : c_uint
}


pub struct get_color_table_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_color_table_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_convolution_filter_cookie {
    sequence : c_uint
}


pub struct get_convolution_filter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8
}


pub struct get_convolution_filter_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    width :           i32,
    height :          i32,
    pad2 :            [u8,..8]
}


pub struct get_convolution_parameterfv_cookie {
    sequence : c_uint
}


pub struct get_convolution_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_convolution_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_convolution_parameteriv_cookie {
    sequence : c_uint
}


pub struct get_convolution_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_convolution_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_separable_filter_cookie {
    sequence : c_uint
}


pub struct get_separable_filter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8
}


pub struct get_separable_filter_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    row_w :           i32,
    col_h :           i32,
    pad2 :            [u8,..8]
}


pub struct get_histogram_cookie {
    sequence : c_uint
}


pub struct get_histogram_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8,
    reset :          u8
}


pub struct get_histogram_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    width :           i32,
    pad2 :            [u8,..12]
}


pub struct get_histogram_parameterfv_cookie {
    sequence : c_uint
}


pub struct get_histogram_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_histogram_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_histogram_parameteriv_cookie {
    sequence : c_uint
}


pub struct get_histogram_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_histogram_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_minmax_cookie {
    sequence : c_uint
}


pub struct get_minmax_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    format :         u32,
    type_ :          u32,
    swap_bytes :     u8,
    reset :          u8
}


pub struct get_minmax_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}


pub struct get_minmax_parameterfv_cookie {
    sequence : c_uint
}


pub struct get_minmax_parameterfv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_minmax_parameterfv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           float32,
    pad2 :            [u8,..12]
}


pub struct get_minmax_parameteriv_cookie {
    sequence : c_uint
}


pub struct get_minmax_parameteriv_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_minmax_parameteriv_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_compressed_tex_image_arb_cookie {
    sequence : c_uint
}


pub struct get_compressed_tex_image_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    level :          i32
}


pub struct get_compressed_tex_image_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..8],
    size :            i32,
    pad2 :            [u8,..12]
}



pub struct delete_queries_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    n :              i32
}


pub struct gen_queries_arb_cookie {
    sequence : c_uint
}


pub struct gen_queries_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    n :              i32
}


pub struct gen_queries_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..24]
}


pub struct is_query_arb_cookie {
    sequence : c_uint
}


pub struct is_query_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    id :             u32
}


pub struct is_query_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    ret_val :         bool32
}


pub struct get_queryiv_arb_cookie {
    sequence : c_uint
}


pub struct get_queryiv_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    target :         u32,
    pname :          u32
}


pub struct get_queryiv_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_query_objectiv_arb_cookie {
    sequence : c_uint
}


pub struct get_query_objectiv_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    id :             u32,
    pname :          u32
}


pub struct get_query_objectiv_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           i32,
    pad2 :            [u8,..12]
}


pub struct get_query_objectuiv_arb_cookie {
    sequence : c_uint
}


pub struct get_query_objectuiv_arb_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    context_tag :    context_tag,
    id :             u32,
    pname :          u32
}


pub struct get_query_objectuiv_arb_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    pad1 :            [u8,..4],
    n :               u32,
    datum :           u32,
    pad2 :            [u8,..12]
}

#[link(name="lxcb-glx")]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a pixmap_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pixmap)
 *
 *
 */
pub fn xcb_glx_pixmap_next (i:*mut pixmap_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pixmap_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_glx_pixmap_end (i:pixmap_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a context_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(context)
 *
 *
 */
pub fn xcb_glx_context_next (i:*mut context_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An context_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_glx_context_end (i:context_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a pbuffer_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(pbuffer)
 *
 *
 */
pub fn xcb_glx_pbuffer_next (i:*mut pbuffer_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An pbuffer_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_glx_pbuffer_end (i:pbuffer_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a window_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(window)
 *
 *
 */
pub fn xcb_glx_window_next (i:*mut window_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An window_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_glx_window_end (i:window_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a fbconfig_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(fbconfig)
 *
 *
 */
pub fn xcb_glx_fbconfig_next (i:*mut fbconfig_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An fbconfig_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_glx_fbconfig_end (i:fbconfig_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a drawable_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(drawable)
 *
 *
 */
pub fn xcb_glx_drawable_next (i:*mut drawable_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An drawable_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_glx_drawable_end (i:drawable_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a float32_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(float32)
 *
 *
 */
pub fn xcb_glx_float32_next (i:*mut float32_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An float32_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_glx_float32_end (i:float32_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a float64_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(float64)
 *
 *
 */
pub fn xcb_glx_float64_next (i:*mut float64_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An float64_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_glx_float64_end (i:float64_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a bool32_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(bool32)
 *
 *
 */
pub fn xcb_glx_bool32_next (i:*mut bool32_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An bool32_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_glx_bool32_end (i:bool32_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a context_tag_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(context_tag)
 *
 *
 */
pub fn xcb_glx_context_tag_next (i:*mut context_tag_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An context_tag_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_glx_context_tag_end (i:context_tag_iterator) -> generic_iterator;

pub fn xcb_glx_render_sizeof (_buffer :  *mut c_void,
                       data_len :     u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_render_checked (c : *mut connection,
                                  context_tag :  context_tag,
                                  data_len :  u32,
                                  data : *mut u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_render (c : *mut connection,
                          context_tag :  context_tag,
                          data_len :  u32,
                          data : *mut u8) -> void_cookie;

pub fn xcb_glx_render_large_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_render_large_checked (c : *mut connection,
                                        context_tag :  context_tag,
                                        request_num :  u16,
                                        request_total :  u16,
                                        data_len :  u32,
                                        data : *mut u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_render_large (c : *mut connection,
                                context_tag :  context_tag,
                                request_num :  u16,
                                request_total :  u16,
                                data_len :  u32,
                                data : *mut u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_create_context_checked (c : *mut connection,
                                          context :  context,
                                          visual :  ffi::xproto::visualid,
                                          screen :  u32,
                                          share_list :  context,
                                          is_direct :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_create_context (c : *mut connection,
                                  context :  context,
                                  visual :  ffi::xproto::visualid,
                                  screen :  u32,
                                  share_list :  context,
                                  is_direct :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_destroy_context_checked (c : *mut connection,
                                           context :  context) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_destroy_context (c : *mut connection,
                                   context :  context) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_make_current (c : *mut connection,
                                drawable :  drawable,
                                context :  context,
                                old_context_tag :  context_tag) -> make_current_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_make_current_unchecked (c : *mut connection,
                                          drawable :  drawable,
                                          context :  context,
                                          old_context_tag :  context_tag) -> make_current_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_make_current_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_make_current_reply (c : *mut connection,
                                      cookie : make_current_cookie,
                                      e : *mut *mut generic_error) -> *mut make_current_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_is_direct (c : *mut connection,
                             context :  context) -> is_direct_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_is_direct_unchecked (c : *mut connection,
                                       context :  context) -> is_direct_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_direct_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_is_direct_reply (c : *mut connection,
                                   cookie : is_direct_cookie,
                                   e : *mut *mut generic_error) -> *mut is_direct_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_query_version (c : *mut connection,
                                 major_version :  u32,
                                 minor_version :  u32) -> query_version_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_query_version_unchecked (c : *mut connection,
                                           major_version :  u32,
                                           minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_query_version_reply (c : *mut connection,
                                       cookie : query_version_cookie,
                                       e : *mut *mut generic_error) -> *mut query_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_wait_gl_checked (c : *mut connection,
                                   context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_wait_gl (c : *mut connection,
                           context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_wait_x_checked (c : *mut connection,
                                  context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_wait_x (c : *mut connection,
                          context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_copy_context_checked (c : *mut connection,
                                        src :  context,
                                        dest :  context,
                                        mask :  u32,
                                        src_context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_copy_context (c : *mut connection,
                                src :  context,
                                dest :  context,
                                mask :  u32,
                                src_context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_swap_buffers_checked (c : *mut connection,
                                        context_tag :  context_tag,
                                        drawable :  drawable) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_swap_buffers (c : *mut connection,
                                context_tag :  context_tag,
                                drawable :  drawable) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_use_x_font_checked (c : *mut connection,
                                      context_tag :  context_tag,
                                      font :  ffi::xproto::font,
                                      first :  u32,
                                      count :  u32,
                                      list_base :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_use_x_font (c : *mut connection,
                              context_tag :  context_tag,
                              font :  ffi::xproto::font,
                              first :  u32,
                              count :  u32,
                              list_base :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_create_glx_pixmap_checked (c : *mut connection,
                                             screen :  u32,
                                             visual :  ffi::xproto::visualid,
                                             pixmap :  ffi::xproto::pixmap,
                                             glx_pixmap :  pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_create_glx_pixmap (c : *mut connection,
                                     screen :  u32,
                                     visual :  ffi::xproto::visualid,
                                     pixmap :  ffi::xproto::pixmap,
                                     glx_pixmap :  pixmap) -> void_cookie;

pub fn xcb_glx_get_visual_configs_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_visual_configs (c : *mut connection,
                                      screen :  u32) -> get_visual_configs_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_visual_configs_unchecked (c : *mut connection,
                                                screen :  u32) -> get_visual_configs_cookie;

pub fn xcb_glx_get_visual_configs_property_list (R : *mut get_visual_configs_reply) -> *mut u32;


pub fn xcb_glx_get_visual_configs_property_list_length (R : *mut get_visual_configs_reply) -> c_int;


pub fn xcb_glx_get_visual_configs_property_list_end (R : *mut get_visual_configs_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_visual_configs_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_visual_configs_reply (c : *mut connection,
                                            cookie : get_visual_configs_cookie,
                                            e : *mut *mut generic_error) -> *mut get_visual_configs_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_destroy_glx_pixmap_checked (c : *mut connection,
                                              glx_pixmap :  pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_destroy_glx_pixmap (c : *mut connection,
                                      glx_pixmap :  pixmap) -> void_cookie;

pub fn xcb_glx_vendor_private_sizeof (_buffer :  *mut c_void,
                               data_len :     u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_vendor_private_checked (c : *mut connection,
                                          vendor_code :  u32,
                                          context_tag :  context_tag,
                                          data_len :  u32,
                                          data : *mut u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_vendor_private (c : *mut connection,
                                  vendor_code :  u32,
                                  context_tag :  context_tag,
                                  data_len :  u32,
                                  data : *mut u8) -> void_cookie;

pub fn xcb_glx_vendor_private_with_reply_sizeof (_buffer :  *mut c_void,
                                          data_len :     u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_vendor_private_with_reply (c : *mut connection,
                                             vendor_code :  u32,
                                             context_tag :  context_tag,
                                             data_len :  u32,
                                             data : *mut u8) -> vendor_private_with_reply_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_vendor_private_with_reply_unchecked (c : *mut connection,
                                                       vendor_code :  u32,
                                                       context_tag :  context_tag,
                                                       data_len :  u32,
                                                       data : *mut u8) -> vendor_private_with_reply_cookie;

pub fn xcb_glx_vendor_private_with_reply_data_2 (R : *mut vendor_private_with_reply_reply) -> *mut u8;


pub fn xcb_glx_vendor_private_with_reply_data_2_length (R : *mut vendor_private_with_reply_reply) -> c_int;


pub fn xcb_glx_vendor_private_with_reply_data_2_end (R : *mut vendor_private_with_reply_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_vendor_private_with_reply_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_vendor_private_with_reply_reply (c : *mut connection,
                                                   cookie : vendor_private_with_reply_cookie,
                                                   e : *mut *mut generic_error) -> *mut vendor_private_with_reply_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_query_extensions_string (c : *mut connection,
                                           screen :  u32) -> query_extensions_string_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_query_extensions_string_unchecked (c : *mut connection,
                                                     screen :  u32) -> query_extensions_string_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_extensions_string_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_query_extensions_string_reply (c : *mut connection,
                                                 cookie : query_extensions_string_cookie,
                                                 e : *mut *mut generic_error) -> *mut query_extensions_string_reply;

pub fn xcb_glx_query_server_string_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_query_server_string (c : *mut connection,
                                       screen :  u32,
                                       name :  u32) -> query_server_string_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_query_server_string_unchecked (c : *mut connection,
                                                 screen :  u32,
                                                 name :  u32) -> query_server_string_cookie;

pub fn xcb_glx_query_server_string_string (R : *mut query_server_string_reply) -> *mut c_char;


pub fn xcb_glx_query_server_string_string_length (R : *mut query_server_string_reply) -> c_int;


pub fn xcb_glx_query_server_string_string_end (R : *mut query_server_string_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_server_string_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_query_server_string_reply (c : *mut connection,
                                             cookie : query_server_string_cookie,
                                             e : *mut *mut generic_error) -> *mut query_server_string_reply;

pub fn xcb_glx_client_info_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_client_info_checked (c : *mut connection,
                                       major_version :  u32,
                                       minor_version :  u32,
                                       str_len :  u32,
                                       string : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_client_info (c : *mut connection,
                               major_version :  u32,
                               minor_version :  u32,
                               str_len :  u32,
                               string : *mut c_char) -> void_cookie;

pub fn xcb_glx_get_fb_configs_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_fb_configs (c : *mut connection,
                                  screen :  u32) -> get_fb_configs_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_fb_configs_unchecked (c : *mut connection,
                                            screen :  u32) -> get_fb_configs_cookie;

pub fn xcb_glx_get_fb_configs_property_list (R : *mut get_fb_configs_reply) -> *mut u32;


pub fn xcb_glx_get_fb_configs_property_list_length (R : *mut get_fb_configs_reply) -> c_int;


pub fn xcb_glx_get_fb_configs_property_list_end (R : *mut get_fb_configs_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_fb_configs_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_fb_configs_reply (c : *mut connection,
                                        cookie : get_fb_configs_cookie,
                                        e : *mut *mut generic_error) -> *mut get_fb_configs_reply;

pub fn xcb_glx_create_pixmap_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_create_pixmap_checked (c : *mut connection,
                                         screen :  u32,
                                         fbconfig :  fbconfig,
                                         pixmap :  ffi::xproto::pixmap,
                                         glx_pixmap :  pixmap,
                                         num_attribs :  u32,
                                         attribs : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_create_pixmap (c : *mut connection,
                                 screen :  u32,
                                 fbconfig :  fbconfig,
                                 pixmap :  ffi::xproto::pixmap,
                                 glx_pixmap :  pixmap,
                                 num_attribs :  u32,
                                 attribs : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_destroy_pixmap_checked (c : *mut connection,
                                          glx_pixmap :  pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_destroy_pixmap (c : *mut connection,
                                  glx_pixmap :  pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_create_new_context_checked (c : *mut connection,
                                              context :  context,
                                              fbconfig :  fbconfig,
                                              screen :  u32,
                                              render_type :  u32,
                                              share_list :  context,
                                              is_direct :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_create_new_context (c : *mut connection,
                                      context :  context,
                                      fbconfig :  fbconfig,
                                      screen :  u32,
                                      render_type :  u32,
                                      share_list :  context,
                                      is_direct :  u8) -> void_cookie;

pub fn xcb_glx_query_context_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_query_context (c : *mut connection,
                                 context :  context) -> query_context_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_query_context_unchecked (c : *mut connection,
                                           context :  context) -> query_context_cookie;

pub fn xcb_glx_query_context_attribs (R : *mut query_context_reply) -> *mut u32;


pub fn xcb_glx_query_context_attribs_length (R : *mut query_context_reply) -> c_int;


pub fn xcb_glx_query_context_attribs_end (R : *mut query_context_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_query_context_reply (c : *mut connection,
                                       cookie : query_context_cookie,
                                       e : *mut *mut generic_error) -> *mut query_context_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_make_context_current (c : *mut connection,
                                        old_context_tag :  context_tag,
                                        drawable :  drawable,
                                        read_drawable :  drawable,
                                        context :  context) -> make_context_current_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_make_context_current_unchecked (c : *mut connection,
                                                  old_context_tag :  context_tag,
                                                  drawable :  drawable,
                                                  read_drawable :  drawable,
                                                  context :  context) -> make_context_current_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_make_context_current_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_make_context_current_reply (c : *mut connection,
                                              cookie : make_context_current_cookie,
                                              e : *mut *mut generic_error) -> *mut make_context_current_reply;

pub fn xcb_glx_create_pbuffer_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_create_pbuffer_checked (c : *mut connection,
                                          screen :  u32,
                                          fbconfig :  fbconfig,
                                          pbuffer :  pbuffer,
                                          num_attribs :  u32,
                                          attribs : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_create_pbuffer (c : *mut connection,
                                  screen :  u32,
                                  fbconfig :  fbconfig,
                                  pbuffer :  pbuffer,
                                  num_attribs :  u32,
                                  attribs : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_destroy_pbuffer_checked (c : *mut connection,
                                           pbuffer :  pbuffer) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_destroy_pbuffer (c : *mut connection,
                                   pbuffer :  pbuffer) -> void_cookie;

pub fn xcb_glx_get_drawable_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_drawable_attributes (c : *mut connection,
                                           drawable :  drawable) -> get_drawable_attributes_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_drawable_attributes_unchecked (c : *mut connection,
                                                     drawable :  drawable) -> get_drawable_attributes_cookie;

pub fn xcb_glx_get_drawable_attributes_attribs (R : *mut get_drawable_attributes_reply) -> *mut u32;


pub fn xcb_glx_get_drawable_attributes_attribs_length (R : *mut get_drawable_attributes_reply) -> c_int;


pub fn xcb_glx_get_drawable_attributes_attribs_end (R : *mut get_drawable_attributes_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_drawable_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_drawable_attributes_reply (c : *mut connection,
                                                 cookie : get_drawable_attributes_cookie,
                                                 e : *mut *mut generic_error) -> *mut get_drawable_attributes_reply;

pub fn xcb_glx_change_drawable_attributes_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_change_drawable_attributes_checked (c : *mut connection,
                                                      drawable :  drawable,
                                                      num_attribs :  u32,
                                                      attribs : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_change_drawable_attributes (c : *mut connection,
                                              drawable :  drawable,
                                              num_attribs :  u32,
                                              attribs : *mut u32) -> void_cookie;

pub fn xcb_glx_create_window_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_create_window_checked (c : *mut connection,
                                         screen :  u32,
                                         fbconfig :  fbconfig,
                                         window :  ffi::xproto::window,
                                         glx_window :  window,
                                         num_attribs :  u32,
                                         attribs : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_create_window (c : *mut connection,
                                 screen :  u32,
                                 fbconfig :  fbconfig,
                                 window :  ffi::xproto::window,
                                 glx_window :  window,
                                 num_attribs :  u32,
                                 attribs : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_delete_window_checked (c : *mut connection,
                                         glxwindow :  window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_delete_window (c : *mut connection,
                                 glxwindow :  window) -> void_cookie;

pub fn xcb_glx_set_client_info_arb_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_set_client_info_arb_checked (c : *mut connection,
                                               major_version :  u32,
                                               minor_version :  u32,
                                               num_versions :  u32,
                                               gl_str_len :  u32,
                                               glx_str_len :  u32,
                                               gl_versions : *mut u32,
                                               gl_extension_string : *mut c_char,
                                               glx_extension_string : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_set_client_info_arb (c : *mut connection,
                                       major_version :  u32,
                                       minor_version :  u32,
                                       num_versions :  u32,
                                       gl_str_len :  u32,
                                       glx_str_len :  u32,
                                       gl_versions : *mut u32,
                                       gl_extension_string : *mut c_char,
                                       glx_extension_string : *mut c_char) -> void_cookie;

pub fn xcb_glx_create_context_attribs_arb_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_create_context_attribs_arb_checked (c : *mut connection,
                                                      context :  context,
                                                      fbconfig :  fbconfig,
                                                      screen :  u32,
                                                      share_list :  context,
                                                      is_direct :  u8,
                                                      num_attribs :  u32,
                                                      attribs : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_create_context_attribs_arb (c : *mut connection,
                                              context :  context,
                                              fbconfig :  fbconfig,
                                              screen :  u32,
                                              share_list :  context,
                                              is_direct :  u8,
                                              num_attribs :  u32,
                                              attribs : *mut u32) -> void_cookie;

pub fn xcb_glx_set_client_info_2arb_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_set_client_info_2arb_checked (c : *mut connection,
                                                major_version :  u32,
                                                minor_version :  u32,
                                                num_versions :  u32,
                                                gl_str_len :  u32,
                                                glx_str_len :  u32,
                                                gl_versions : *mut u32,
                                                gl_extension_string : *mut c_char,
                                                glx_extension_string : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_set_client_info_2arb (c : *mut connection,
                                        major_version :  u32,
                                        minor_version :  u32,
                                        num_versions :  u32,
                                        gl_str_len :  u32,
                                        glx_str_len :  u32,
                                        gl_versions : *mut u32,
                                        gl_extension_string : *mut c_char,
                                        glx_extension_string : *mut c_char) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_new_list_checked (c : *mut connection,
                                    context_tag :  context_tag,
                                    list :  u32,
                                    mode :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_new_list (c : *mut connection,
                            context_tag :  context_tag,
                            list :  u32,
                            mode :  u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_end_list_checked (c : *mut connection,
                                    context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_end_list (c : *mut connection,
                            context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_delete_lists_checked (c : *mut connection,
                                        context_tag :  context_tag,
                                        list :  u32,
                                        range :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_delete_lists (c : *mut connection,
                                context_tag :  context_tag,
                                list :  u32,
                                range :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_gen_lists (c : *mut connection,
                             context_tag :  context_tag,
                             range :  i32) -> gen_lists_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_gen_lists_unchecked (c : *mut connection,
                                       context_tag :  context_tag,
                                       range :  i32) -> gen_lists_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_gen_lists_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_gen_lists_reply (c : *mut connection,
                                   cookie : gen_lists_cookie,
                                   e : *mut *mut generic_error) -> *mut gen_lists_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_feedback_buffer_checked (c : *mut connection,
                                           context_tag :  context_tag,
                                           size :  i32,
                                           type_ :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_feedback_buffer (c : *mut connection,
                                   context_tag :  context_tag,
                                   size :  i32,
                                   type_ :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_select_buffer_checked (c : *mut connection,
                                         context_tag :  context_tag,
                                         size :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_select_buffer (c : *mut connection,
                                 context_tag :  context_tag,
                                 size :  i32) -> void_cookie;

pub fn xcb_glx_render_mode_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_render_mode (c : *mut connection,
                               context_tag :  context_tag,
                               mode :  u32) -> render_mode_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_render_mode_unchecked (c : *mut connection,
                                         context_tag :  context_tag,
                                         mode :  u32) -> render_mode_cookie;

pub fn xcb_glx_render_mode_data (R : *mut render_mode_reply) -> *mut u32;


pub fn xcb_glx_render_mode_data_length (R : *mut render_mode_reply) -> c_int;


pub fn xcb_glx_render_mode_data_end (R : *mut render_mode_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_render_mode_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_render_mode_reply (c : *mut connection,
                                     cookie : render_mode_cookie,
                                     e : *mut *mut generic_error) -> *mut render_mode_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_finish (c : *mut connection,
                          context_tag :  context_tag) -> finish_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_finish_unchecked (c : *mut connection,
                                    context_tag :  context_tag) -> finish_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_finish_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_finish_reply (c : *mut connection,
                                cookie : finish_cookie,
                                e : *mut *mut generic_error) -> *mut finish_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_pixel_storef_checked (c : *mut connection,
                                        context_tag :  context_tag,
                                        pname :  u32,
                                        datum :  float32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_pixel_storef (c : *mut connection,
                                context_tag :  context_tag,
                                pname :  u32,
                                datum :  float32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_pixel_storei_checked (c : *mut connection,
                                        context_tag :  context_tag,
                                        pname :  u32,
                                        datum :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_pixel_storei (c : *mut connection,
                                context_tag :  context_tag,
                                pname :  u32,
                                datum :  i32) -> void_cookie;

pub fn xcb_glx_read_pixels_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_read_pixels (c : *mut connection,
                               context_tag :  context_tag,
                               x :  i32,
                               y :  i32,
                               width :  i32,
                               height :  i32,
                               format :  u32,
                               type_ :  u32,
                               swap_bytes :  u8,
                               lsb_first :  u8) -> read_pixels_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_read_pixels_unchecked (c : *mut connection,
                                         context_tag :  context_tag,
                                         x :  i32,
                                         y :  i32,
                                         width :  i32,
                                         height :  i32,
                                         format :  u32,
                                         type_ :  u32,
                                         swap_bytes :  u8,
                                         lsb_first :  u8) -> read_pixels_cookie;

pub fn xcb_glx_read_pixels_data (R : *mut read_pixels_reply) -> *mut u8;


pub fn xcb_glx_read_pixels_data_length (R : *mut read_pixels_reply) -> c_int;


pub fn xcb_glx_read_pixels_data_end (R : *mut read_pixels_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_read_pixels_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_read_pixels_reply (c : *mut connection,
                                     cookie : read_pixels_cookie,
                                     e : *mut *mut generic_error) -> *mut read_pixels_reply;

pub fn xcb_glx_get_booleanv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_booleanv (c : *mut connection,
                                context_tag :  context_tag,
                                pname :  i32) -> get_booleanv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_booleanv_unchecked (c : *mut connection,
                                          context_tag :  context_tag,
                                          pname :  i32) -> get_booleanv_cookie;

pub fn xcb_glx_get_booleanv_data (R : *mut get_booleanv_reply) -> *mut u8;


pub fn xcb_glx_get_booleanv_data_length (R : *mut get_booleanv_reply) -> c_int;


pub fn xcb_glx_get_booleanv_data_end (R : *mut get_booleanv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_booleanv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_booleanv_reply (c : *mut connection,
                                      cookie : get_booleanv_cookie,
                                      e : *mut *mut generic_error) -> *mut get_booleanv_reply;

pub fn xcb_glx_get_clip_plane_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_clip_plane (c : *mut connection,
                                  context_tag :  context_tag,
                                  plane :  i32) -> get_clip_plane_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_clip_plane_unchecked (c : *mut connection,
                                            context_tag :  context_tag,
                                            plane :  i32) -> get_clip_plane_cookie;

pub fn xcb_glx_get_clip_plane_data (R : *mut get_clip_plane_reply) -> *mut float64;


pub fn xcb_glx_get_clip_plane_data_length (R : *mut get_clip_plane_reply) -> c_int;


pub fn xcb_glx_get_clip_plane_data_end (R : *mut get_clip_plane_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_clip_plane_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_clip_plane_reply (c : *mut connection,
                                        cookie : get_clip_plane_cookie,
                                        e : *mut *mut generic_error) -> *mut get_clip_plane_reply;

pub fn xcb_glx_get_doublev_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_doublev (c : *mut connection,
                               context_tag :  context_tag,
                               pname :  u32) -> get_doublev_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_doublev_unchecked (c : *mut connection,
                                         context_tag :  context_tag,
                                         pname :  u32) -> get_doublev_cookie;

pub fn xcb_glx_get_doublev_data (R : *mut get_doublev_reply) -> *mut float64;


pub fn xcb_glx_get_doublev_data_length (R : *mut get_doublev_reply) -> c_int;


pub fn xcb_glx_get_doublev_data_end (R : *mut get_doublev_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_doublev_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_doublev_reply (c : *mut connection,
                                     cookie : get_doublev_cookie,
                                     e : *mut *mut generic_error) -> *mut get_doublev_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_error (c : *mut connection,
                             context_tag :  context_tag) -> get_error_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_error_unchecked (c : *mut connection,
                                       context_tag :  context_tag) -> get_error_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_error_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_error_reply (c : *mut connection,
                                   cookie : get_error_cookie,
                                   e : *mut *mut generic_error) -> *mut get_error_reply;

pub fn xcb_glx_get_floatv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_floatv (c : *mut connection,
                              context_tag :  context_tag,
                              pname :  u32) -> get_floatv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_floatv_unchecked (c : *mut connection,
                                        context_tag :  context_tag,
                                        pname :  u32) -> get_floatv_cookie;

pub fn xcb_glx_get_floatv_data (R : *mut get_floatv_reply) -> *mut float32;


pub fn xcb_glx_get_floatv_data_length (R : *mut get_floatv_reply) -> c_int;


pub fn xcb_glx_get_floatv_data_end (R : *mut get_floatv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_floatv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_floatv_reply (c : *mut connection,
                                    cookie : get_floatv_cookie,
                                    e : *mut *mut generic_error) -> *mut get_floatv_reply;

pub fn xcb_glx_get_integerv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_integerv (c : *mut connection,
                                context_tag :  context_tag,
                                pname :  u32) -> get_integerv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_integerv_unchecked (c : *mut connection,
                                          context_tag :  context_tag,
                                          pname :  u32) -> get_integerv_cookie;

pub fn xcb_glx_get_integerv_data (R : *mut get_integerv_reply) -> *mut i32;


pub fn xcb_glx_get_integerv_data_length (R : *mut get_integerv_reply) -> c_int;


pub fn xcb_glx_get_integerv_data_end (R : *mut get_integerv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_integerv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_integerv_reply (c : *mut connection,
                                      cookie : get_integerv_cookie,
                                      e : *mut *mut generic_error) -> *mut get_integerv_reply;

pub fn xcb_glx_get_lightfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_lightfv (c : *mut connection,
                               context_tag :  context_tag,
                               light :  u32,
                               pname :  u32) -> get_lightfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_lightfv_unchecked (c : *mut connection,
                                         context_tag :  context_tag,
                                         light :  u32,
                                         pname :  u32) -> get_lightfv_cookie;

pub fn xcb_glx_get_lightfv_data (R : *mut get_lightfv_reply) -> *mut float32;


pub fn xcb_glx_get_lightfv_data_length (R : *mut get_lightfv_reply) -> c_int;


pub fn xcb_glx_get_lightfv_data_end (R : *mut get_lightfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_lightfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_lightfv_reply (c : *mut connection,
                                     cookie : get_lightfv_cookie,
                                     e : *mut *mut generic_error) -> *mut get_lightfv_reply;

pub fn xcb_glx_get_lightiv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_lightiv (c : *mut connection,
                               context_tag :  context_tag,
                               light :  u32,
                               pname :  u32) -> get_lightiv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_lightiv_unchecked (c : *mut connection,
                                         context_tag :  context_tag,
                                         light :  u32,
                                         pname :  u32) -> get_lightiv_cookie;

pub fn xcb_glx_get_lightiv_data (R : *mut get_lightiv_reply) -> *mut i32;


pub fn xcb_glx_get_lightiv_data_length (R : *mut get_lightiv_reply) -> c_int;


pub fn xcb_glx_get_lightiv_data_end (R : *mut get_lightiv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_lightiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_lightiv_reply (c : *mut connection,
                                     cookie : get_lightiv_cookie,
                                     e : *mut *mut generic_error) -> *mut get_lightiv_reply;

pub fn xcb_glx_get_mapdv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_mapdv (c : *mut connection,
                             context_tag :  context_tag,
                             target :  u32,
                             query :  u32) -> get_mapdv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_mapdv_unchecked (c : *mut connection,
                                       context_tag :  context_tag,
                                       target :  u32,
                                       query :  u32) -> get_mapdv_cookie;

pub fn xcb_glx_get_mapdv_data (R : *mut get_mapdv_reply) -> *mut float64;


pub fn xcb_glx_get_mapdv_data_length (R : *mut get_mapdv_reply) -> c_int;


pub fn xcb_glx_get_mapdv_data_end (R : *mut get_mapdv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_mapdv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_mapdv_reply (c : *mut connection,
                                   cookie : get_mapdv_cookie,
                                   e : *mut *mut generic_error) -> *mut get_mapdv_reply;

pub fn xcb_glx_get_mapfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_mapfv (c : *mut connection,
                             context_tag :  context_tag,
                             target :  u32,
                             query :  u32) -> get_mapfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_mapfv_unchecked (c : *mut connection,
                                       context_tag :  context_tag,
                                       target :  u32,
                                       query :  u32) -> get_mapfv_cookie;

pub fn xcb_glx_get_mapfv_data (R : *mut get_mapfv_reply) -> *mut float32;


pub fn xcb_glx_get_mapfv_data_length (R : *mut get_mapfv_reply) -> c_int;


pub fn xcb_glx_get_mapfv_data_end (R : *mut get_mapfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_mapfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_mapfv_reply (c : *mut connection,
                                   cookie : get_mapfv_cookie,
                                   e : *mut *mut generic_error) -> *mut get_mapfv_reply;

pub fn xcb_glx_get_mapiv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_mapiv (c : *mut connection,
                             context_tag :  context_tag,
                             target :  u32,
                             query :  u32) -> get_mapiv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_mapiv_unchecked (c : *mut connection,
                                       context_tag :  context_tag,
                                       target :  u32,
                                       query :  u32) -> get_mapiv_cookie;

pub fn xcb_glx_get_mapiv_data (R : *mut get_mapiv_reply) -> *mut i32;


pub fn xcb_glx_get_mapiv_data_length (R : *mut get_mapiv_reply) -> c_int;


pub fn xcb_glx_get_mapiv_data_end (R : *mut get_mapiv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_mapiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_mapiv_reply (c : *mut connection,
                                   cookie : get_mapiv_cookie,
                                   e : *mut *mut generic_error) -> *mut get_mapiv_reply;

pub fn xcb_glx_get_materialfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_materialfv (c : *mut connection,
                                  context_tag :  context_tag,
                                  face :  u32,
                                  pname :  u32) -> get_materialfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_materialfv_unchecked (c : *mut connection,
                                            context_tag :  context_tag,
                                            face :  u32,
                                            pname :  u32) -> get_materialfv_cookie;

pub fn xcb_glx_get_materialfv_data (R : *mut get_materialfv_reply) -> *mut float32;


pub fn xcb_glx_get_materialfv_data_length (R : *mut get_materialfv_reply) -> c_int;


pub fn xcb_glx_get_materialfv_data_end (R : *mut get_materialfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_materialfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_materialfv_reply (c : *mut connection,
                                        cookie : get_materialfv_cookie,
                                        e : *mut *mut generic_error) -> *mut get_materialfv_reply;

pub fn xcb_glx_get_materialiv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_materialiv (c : *mut connection,
                                  context_tag :  context_tag,
                                  face :  u32,
                                  pname :  u32) -> get_materialiv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_materialiv_unchecked (c : *mut connection,
                                            context_tag :  context_tag,
                                            face :  u32,
                                            pname :  u32) -> get_materialiv_cookie;

pub fn xcb_glx_get_materialiv_data (R : *mut get_materialiv_reply) -> *mut i32;


pub fn xcb_glx_get_materialiv_data_length (R : *mut get_materialiv_reply) -> c_int;


pub fn xcb_glx_get_materialiv_data_end (R : *mut get_materialiv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_materialiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_materialiv_reply (c : *mut connection,
                                        cookie : get_materialiv_cookie,
                                        e : *mut *mut generic_error) -> *mut get_materialiv_reply;

pub fn xcb_glx_get_pixel_mapfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_pixel_mapfv (c : *mut connection,
                                   context_tag :  context_tag,
                                   map :  u32) -> get_pixel_mapfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_pixel_mapfv_unchecked (c : *mut connection,
                                             context_tag :  context_tag,
                                             map :  u32) -> get_pixel_mapfv_cookie;

pub fn xcb_glx_get_pixel_mapfv_data (R : *mut get_pixel_mapfv_reply) -> *mut float32;


pub fn xcb_glx_get_pixel_mapfv_data_length (R : *mut get_pixel_mapfv_reply) -> c_int;


pub fn xcb_glx_get_pixel_mapfv_data_end (R : *mut get_pixel_mapfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_pixel_mapfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_pixel_mapfv_reply (c : *mut connection,
                                         cookie : get_pixel_mapfv_cookie,
                                         e : *mut *mut generic_error) -> *mut get_pixel_mapfv_reply;

pub fn xcb_glx_get_pixel_mapuiv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_pixel_mapuiv (c : *mut connection,
                                    context_tag :  context_tag,
                                    map :  u32) -> get_pixel_mapuiv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_pixel_mapuiv_unchecked (c : *mut connection,
                                              context_tag :  context_tag,
                                              map :  u32) -> get_pixel_mapuiv_cookie;

pub fn xcb_glx_get_pixel_mapuiv_data (R : *mut get_pixel_mapuiv_reply) -> *mut u32;


pub fn xcb_glx_get_pixel_mapuiv_data_length (R : *mut get_pixel_mapuiv_reply) -> c_int;


pub fn xcb_glx_get_pixel_mapuiv_data_end (R : *mut get_pixel_mapuiv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_pixel_mapuiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_pixel_mapuiv_reply (c : *mut connection,
                                          cookie : get_pixel_mapuiv_cookie,
                                          e : *mut *mut generic_error) -> *mut get_pixel_mapuiv_reply;

pub fn xcb_glx_get_pixel_mapusv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_pixel_mapusv (c : *mut connection,
                                    context_tag :  context_tag,
                                    map :  u32) -> get_pixel_mapusv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_pixel_mapusv_unchecked (c : *mut connection,
                                              context_tag :  context_tag,
                                              map :  u32) -> get_pixel_mapusv_cookie;

pub fn xcb_glx_get_pixel_mapusv_data (R : *mut get_pixel_mapusv_reply) -> *mut u16;


pub fn xcb_glx_get_pixel_mapusv_data_length (R : *mut get_pixel_mapusv_reply) -> c_int;


pub fn xcb_glx_get_pixel_mapusv_data_end (R : *mut get_pixel_mapusv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_pixel_mapusv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_pixel_mapusv_reply (c : *mut connection,
                                          cookie : get_pixel_mapusv_cookie,
                                          e : *mut *mut generic_error) -> *mut get_pixel_mapusv_reply;

pub fn xcb_glx_get_polygon_stipple_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_polygon_stipple (c : *mut connection,
                                       context_tag :  context_tag,
                                       lsb_first :  u8) -> get_polygon_stipple_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_polygon_stipple_unchecked (c : *mut connection,
                                                 context_tag :  context_tag,
                                                 lsb_first :  u8) -> get_polygon_stipple_cookie;

pub fn xcb_glx_get_polygon_stipple_data (R : *mut get_polygon_stipple_reply) -> *mut u8;


pub fn xcb_glx_get_polygon_stipple_data_length (R : *mut get_polygon_stipple_reply) -> c_int;


pub fn xcb_glx_get_polygon_stipple_data_end (R : *mut get_polygon_stipple_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_polygon_stipple_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_polygon_stipple_reply (c : *mut connection,
                                             cookie : get_polygon_stipple_cookie,
                                             e : *mut *mut generic_error) -> *mut get_polygon_stipple_reply;

pub fn xcb_glx_get_string_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_string (c : *mut connection,
                              context_tag :  context_tag,
                              name :  u32) -> get_string_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_string_unchecked (c : *mut connection,
                                        context_tag :  context_tag,
                                        name :  u32) -> get_string_cookie;

pub fn xcb_glx_get_string_string (R : *mut get_string_reply) -> *mut c_char;


pub fn xcb_glx_get_string_string_length (R : *mut get_string_reply) -> c_int;


pub fn xcb_glx_get_string_string_end (R : *mut get_string_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_string_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_string_reply (c : *mut connection,
                                    cookie : get_string_cookie,
                                    e : *mut *mut generic_error) -> *mut get_string_reply;

pub fn xcb_glx_get_tex_envfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_tex_envfv (c : *mut connection,
                                 context_tag :  context_tag,
                                 target :  u32,
                                 pname :  u32) -> get_tex_envfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_tex_envfv_unchecked (c : *mut connection,
                                           context_tag :  context_tag,
                                           target :  u32,
                                           pname :  u32) -> get_tex_envfv_cookie;

pub fn xcb_glx_get_tex_envfv_data (R : *mut get_tex_envfv_reply) -> *mut float32;


pub fn xcb_glx_get_tex_envfv_data_length (R : *mut get_tex_envfv_reply) -> c_int;


pub fn xcb_glx_get_tex_envfv_data_end (R : *mut get_tex_envfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_envfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_tex_envfv_reply (c : *mut connection,
                                       cookie : get_tex_envfv_cookie,
                                       e : *mut *mut generic_error) -> *mut get_tex_envfv_reply;

pub fn xcb_glx_get_tex_enviv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_tex_enviv (c : *mut connection,
                                 context_tag :  context_tag,
                                 target :  u32,
                                 pname :  u32) -> get_tex_enviv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_tex_enviv_unchecked (c : *mut connection,
                                           context_tag :  context_tag,
                                           target :  u32,
                                           pname :  u32) -> get_tex_enviv_cookie;

pub fn xcb_glx_get_tex_enviv_data (R : *mut get_tex_enviv_reply) -> *mut i32;


pub fn xcb_glx_get_tex_enviv_data_length (R : *mut get_tex_enviv_reply) -> c_int;


pub fn xcb_glx_get_tex_enviv_data_end (R : *mut get_tex_enviv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_enviv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_tex_enviv_reply (c : *mut connection,
                                       cookie : get_tex_enviv_cookie,
                                       e : *mut *mut generic_error) -> *mut get_tex_enviv_reply;

pub fn xcb_glx_get_tex_gendv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_tex_gendv (c : *mut connection,
                                 context_tag :  context_tag,
                                 coord :  u32,
                                 pname :  u32) -> get_tex_gendv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_tex_gendv_unchecked (c : *mut connection,
                                           context_tag :  context_tag,
                                           coord :  u32,
                                           pname :  u32) -> get_tex_gendv_cookie;

pub fn xcb_glx_get_tex_gendv_data (R : *mut get_tex_gendv_reply) -> *mut float64;


pub fn xcb_glx_get_tex_gendv_data_length (R : *mut get_tex_gendv_reply) -> c_int;


pub fn xcb_glx_get_tex_gendv_data_end (R : *mut get_tex_gendv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_gendv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_tex_gendv_reply (c : *mut connection,
                                       cookie : get_tex_gendv_cookie,
                                       e : *mut *mut generic_error) -> *mut get_tex_gendv_reply;

pub fn xcb_glx_get_tex_genfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_tex_genfv (c : *mut connection,
                                 context_tag :  context_tag,
                                 coord :  u32,
                                 pname :  u32) -> get_tex_genfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_tex_genfv_unchecked (c : *mut connection,
                                           context_tag :  context_tag,
                                           coord :  u32,
                                           pname :  u32) -> get_tex_genfv_cookie;

pub fn xcb_glx_get_tex_genfv_data (R : *mut get_tex_genfv_reply) -> *mut float32;


pub fn xcb_glx_get_tex_genfv_data_length (R : *mut get_tex_genfv_reply) -> c_int;


pub fn xcb_glx_get_tex_genfv_data_end (R : *mut get_tex_genfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_genfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_tex_genfv_reply (c : *mut connection,
                                       cookie : get_tex_genfv_cookie,
                                       e : *mut *mut generic_error) -> *mut get_tex_genfv_reply;

pub fn xcb_glx_get_tex_geniv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_tex_geniv (c : *mut connection,
                                 context_tag :  context_tag,
                                 coord :  u32,
                                 pname :  u32) -> get_tex_geniv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_tex_geniv_unchecked (c : *mut connection,
                                           context_tag :  context_tag,
                                           coord :  u32,
                                           pname :  u32) -> get_tex_geniv_cookie;

pub fn xcb_glx_get_tex_geniv_data (R : *mut get_tex_geniv_reply) -> *mut i32;


pub fn xcb_glx_get_tex_geniv_data_length (R : *mut get_tex_geniv_reply) -> c_int;


pub fn xcb_glx_get_tex_geniv_data_end (R : *mut get_tex_geniv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_geniv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_tex_geniv_reply (c : *mut connection,
                                       cookie : get_tex_geniv_cookie,
                                       e : *mut *mut generic_error) -> *mut get_tex_geniv_reply;

pub fn xcb_glx_get_tex_image_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_tex_image (c : *mut connection,
                                 context_tag :  context_tag,
                                 target :  u32,
                                 level :  i32,
                                 format :  u32,
                                 type_ :  u32,
                                 swap_bytes :  u8) -> get_tex_image_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_tex_image_unchecked (c : *mut connection,
                                           context_tag :  context_tag,
                                           target :  u32,
                                           level :  i32,
                                           format :  u32,
                                           type_ :  u32,
                                           swap_bytes :  u8) -> get_tex_image_cookie;

pub fn xcb_glx_get_tex_image_data (R : *mut get_tex_image_reply) -> *mut u8;


pub fn xcb_glx_get_tex_image_data_length (R : *mut get_tex_image_reply) -> c_int;


pub fn xcb_glx_get_tex_image_data_end (R : *mut get_tex_image_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_tex_image_reply (c : *mut connection,
                                       cookie : get_tex_image_cookie,
                                       e : *mut *mut generic_error) -> *mut get_tex_image_reply;

pub fn xcb_glx_get_tex_parameterfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_tex_parameterfv (c : *mut connection,
                                       context_tag :  context_tag,
                                       target :  u32,
                                       pname :  u32) -> get_tex_parameterfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_tex_parameterfv_unchecked (c : *mut connection,
                                                 context_tag :  context_tag,
                                                 target :  u32,
                                                 pname :  u32) -> get_tex_parameterfv_cookie;

pub fn xcb_glx_get_tex_parameterfv_data (R : *mut get_tex_parameterfv_reply) -> *mut float32;


pub fn xcb_glx_get_tex_parameterfv_data_length (R : *mut get_tex_parameterfv_reply) -> c_int;


pub fn xcb_glx_get_tex_parameterfv_data_end (R : *mut get_tex_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_tex_parameterfv_reply (c : *mut connection,
                                             cookie : get_tex_parameterfv_cookie,
                                             e : *mut *mut generic_error) -> *mut get_tex_parameterfv_reply;

pub fn xcb_glx_get_tex_parameteriv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_tex_parameteriv (c : *mut connection,
                                       context_tag :  context_tag,
                                       target :  u32,
                                       pname :  u32) -> get_tex_parameteriv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_tex_parameteriv_unchecked (c : *mut connection,
                                                 context_tag :  context_tag,
                                                 target :  u32,
                                                 pname :  u32) -> get_tex_parameteriv_cookie;

pub fn xcb_glx_get_tex_parameteriv_data (R : *mut get_tex_parameteriv_reply) -> *mut i32;


pub fn xcb_glx_get_tex_parameteriv_data_length (R : *mut get_tex_parameteriv_reply) -> c_int;


pub fn xcb_glx_get_tex_parameteriv_data_end (R : *mut get_tex_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_tex_parameteriv_reply (c : *mut connection,
                                             cookie : get_tex_parameteriv_cookie,
                                             e : *mut *mut generic_error) -> *mut get_tex_parameteriv_reply;

pub fn xcb_glx_get_tex_level_parameterfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_tex_level_parameterfv (c : *mut connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             level :  i32,
                                             pname :  u32) -> get_tex_level_parameterfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_tex_level_parameterfv_unchecked (c : *mut connection,
                                                       context_tag :  context_tag,
                                                       target :  u32,
                                                       level :  i32,
                                                       pname :  u32) -> get_tex_level_parameterfv_cookie;

pub fn xcb_glx_get_tex_level_parameterfv_data (R : *mut get_tex_level_parameterfv_reply) -> *mut float32;


pub fn xcb_glx_get_tex_level_parameterfv_data_length (R : *mut get_tex_level_parameterfv_reply) -> c_int;


pub fn xcb_glx_get_tex_level_parameterfv_data_end (R : *mut get_tex_level_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_level_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_tex_level_parameterfv_reply (c : *mut connection,
                                                   cookie : get_tex_level_parameterfv_cookie,
                                                   e : *mut *mut generic_error) -> *mut get_tex_level_parameterfv_reply;

pub fn xcb_glx_get_tex_level_parameteriv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_tex_level_parameteriv (c : *mut connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             level :  i32,
                                             pname :  u32) -> get_tex_level_parameteriv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_tex_level_parameteriv_unchecked (c : *mut connection,
                                                       context_tag :  context_tag,
                                                       target :  u32,
                                                       level :  i32,
                                                       pname :  u32) -> get_tex_level_parameteriv_cookie;

pub fn xcb_glx_get_tex_level_parameteriv_data (R : *mut get_tex_level_parameteriv_reply) -> *mut i32;


pub fn xcb_glx_get_tex_level_parameteriv_data_length (R : *mut get_tex_level_parameteriv_reply) -> c_int;


pub fn xcb_glx_get_tex_level_parameteriv_data_end (R : *mut get_tex_level_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_level_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_tex_level_parameteriv_reply (c : *mut connection,
                                                   cookie : get_tex_level_parameteriv_cookie,
                                                   e : *mut *mut generic_error) -> *mut get_tex_level_parameteriv_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_is_list (c : *mut connection,
                           context_tag :  context_tag,
                           list :  u32) -> is_list_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_is_list_unchecked (c : *mut connection,
                                     context_tag :  context_tag,
                                     list :  u32) -> is_list_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_is_list_reply (c : *mut connection,
                                 cookie : is_list_cookie,
                                 e : *mut *mut generic_error) -> *mut is_list_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_flush_checked (c : *mut connection,
                                 context_tag :  context_tag) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_flush (c : *mut connection,
                         context_tag :  context_tag) -> void_cookie;

pub fn xcb_glx_are_textures_resident_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_are_textures_resident (c : *mut connection,
                                         context_tag :  context_tag,
                                         n :  i32,
                                         textures : *mut u32) -> are_textures_resident_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_are_textures_resident_unchecked (c : *mut connection,
                                                   context_tag :  context_tag,
                                                   n :  i32,
                                                   textures : *mut u32) -> are_textures_resident_cookie;

pub fn xcb_glx_are_textures_resident_data (R : *mut are_textures_resident_reply) -> *mut u8;


pub fn xcb_glx_are_textures_resident_data_length (R : *mut are_textures_resident_reply) -> c_int;


pub fn xcb_glx_are_textures_resident_data_end (R : *mut are_textures_resident_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_are_textures_resident_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_are_textures_resident_reply (c : *mut connection,
                                               cookie : are_textures_resident_cookie,
                                               e : *mut *mut generic_error) -> *mut are_textures_resident_reply;

pub fn xcb_glx_delete_textures_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_delete_textures_checked (c : *mut connection,
                                           context_tag :  context_tag,
                                           n :  i32,
                                           textures : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_delete_textures (c : *mut connection,
                                   context_tag :  context_tag,
                                   n :  i32,
                                   textures : *mut u32) -> void_cookie;

pub fn xcb_glx_gen_textures_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_gen_textures (c : *mut connection,
                                context_tag :  context_tag,
                                n :  i32) -> gen_textures_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_gen_textures_unchecked (c : *mut connection,
                                          context_tag :  context_tag,
                                          n :  i32) -> gen_textures_cookie;

pub fn xcb_glx_gen_textures_data (R : *mut gen_textures_reply) -> *mut u32;


pub fn xcb_glx_gen_textures_data_length (R : *mut gen_textures_reply) -> c_int;


pub fn xcb_glx_gen_textures_data_end (R : *mut gen_textures_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_gen_textures_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_gen_textures_reply (c : *mut connection,
                                      cookie : gen_textures_cookie,
                                      e : *mut *mut generic_error) -> *mut gen_textures_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_is_texture (c : *mut connection,
                              context_tag :  context_tag,
                              texture :  u32) -> is_texture_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_is_texture_unchecked (c : *mut connection,
                                        context_tag :  context_tag,
                                        texture :  u32) -> is_texture_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_texture_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_is_texture_reply (c : *mut connection,
                                    cookie : is_texture_cookie,
                                    e : *mut *mut generic_error) -> *mut is_texture_reply;

pub fn xcb_glx_get_color_table_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_color_table (c : *mut connection,
                                   context_tag :  context_tag,
                                   target :  u32,
                                   format :  u32,
                                   type_ :  u32,
                                   swap_bytes :  u8) -> get_color_table_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_color_table_unchecked (c : *mut connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             format :  u32,
                                             type_ :  u32,
                                             swap_bytes :  u8) -> get_color_table_cookie;

pub fn xcb_glx_get_color_table_data (R : *mut get_color_table_reply) -> *mut u8;


pub fn xcb_glx_get_color_table_data_length (R : *mut get_color_table_reply) -> c_int;


pub fn xcb_glx_get_color_table_data_end (R : *mut get_color_table_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_color_table_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_color_table_reply (c : *mut connection,
                                         cookie : get_color_table_cookie,
                                         e : *mut *mut generic_error) -> *mut get_color_table_reply;

pub fn xcb_glx_get_color_table_parameterfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_color_table_parameterfv (c : *mut connection,
                                               context_tag :  context_tag,
                                               target :  u32,
                                               pname :  u32) -> get_color_table_parameterfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_color_table_parameterfv_unchecked (c : *mut connection,
                                                         context_tag :  context_tag,
                                                         target :  u32,
                                                         pname :  u32) -> get_color_table_parameterfv_cookie;

pub fn xcb_glx_get_color_table_parameterfv_data (R : *mut get_color_table_parameterfv_reply) -> *mut float32;


pub fn xcb_glx_get_color_table_parameterfv_data_length (R : *mut get_color_table_parameterfv_reply) -> c_int;


pub fn xcb_glx_get_color_table_parameterfv_data_end (R : *mut get_color_table_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_color_table_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_color_table_parameterfv_reply (c : *mut connection,
                                                     cookie : get_color_table_parameterfv_cookie,
                                                     e : *mut *mut generic_error) -> *mut get_color_table_parameterfv_reply;

pub fn xcb_glx_get_color_table_parameteriv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_color_table_parameteriv (c : *mut connection,
                                               context_tag :  context_tag,
                                               target :  u32,
                                               pname :  u32) -> get_color_table_parameteriv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_color_table_parameteriv_unchecked (c : *mut connection,
                                                         context_tag :  context_tag,
                                                         target :  u32,
                                                         pname :  u32) -> get_color_table_parameteriv_cookie;

pub fn xcb_glx_get_color_table_parameteriv_data (R : *mut get_color_table_parameteriv_reply) -> *mut i32;


pub fn xcb_glx_get_color_table_parameteriv_data_length (R : *mut get_color_table_parameteriv_reply) -> c_int;


pub fn xcb_glx_get_color_table_parameteriv_data_end (R : *mut get_color_table_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_color_table_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_color_table_parameteriv_reply (c : *mut connection,
                                                     cookie : get_color_table_parameteriv_cookie,
                                                     e : *mut *mut generic_error) -> *mut get_color_table_parameteriv_reply;

pub fn xcb_glx_get_convolution_filter_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_convolution_filter (c : *mut connection,
                                          context_tag :  context_tag,
                                          target :  u32,
                                          format :  u32,
                                          type_ :  u32,
                                          swap_bytes :  u8) -> get_convolution_filter_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_convolution_filter_unchecked (c : *mut connection,
                                                    context_tag :  context_tag,
                                                    target :  u32,
                                                    format :  u32,
                                                    type_ :  u32,
                                                    swap_bytes :  u8) -> get_convolution_filter_cookie;

pub fn xcb_glx_get_convolution_filter_data (R : *mut get_convolution_filter_reply) -> *mut u8;


pub fn xcb_glx_get_convolution_filter_data_length (R : *mut get_convolution_filter_reply) -> c_int;


pub fn xcb_glx_get_convolution_filter_data_end (R : *mut get_convolution_filter_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_convolution_filter_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_convolution_filter_reply (c : *mut connection,
                                                cookie : get_convolution_filter_cookie,
                                                e : *mut *mut generic_error) -> *mut get_convolution_filter_reply;

pub fn xcb_glx_get_convolution_parameterfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_convolution_parameterfv (c : *mut connection,
                                               context_tag :  context_tag,
                                               target :  u32,
                                               pname :  u32) -> get_convolution_parameterfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_convolution_parameterfv_unchecked (c : *mut connection,
                                                         context_tag :  context_tag,
                                                         target :  u32,
                                                         pname :  u32) -> get_convolution_parameterfv_cookie;

pub fn xcb_glx_get_convolution_parameterfv_data (R : *mut get_convolution_parameterfv_reply) -> *mut float32;


pub fn xcb_glx_get_convolution_parameterfv_data_length (R : *mut get_convolution_parameterfv_reply) -> c_int;


pub fn xcb_glx_get_convolution_parameterfv_data_end (R : *mut get_convolution_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_convolution_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_convolution_parameterfv_reply (c : *mut connection,
                                                     cookie : get_convolution_parameterfv_cookie,
                                                     e : *mut *mut generic_error) -> *mut get_convolution_parameterfv_reply;

pub fn xcb_glx_get_convolution_parameteriv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_convolution_parameteriv (c : *mut connection,
                                               context_tag :  context_tag,
                                               target :  u32,
                                               pname :  u32) -> get_convolution_parameteriv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_convolution_parameteriv_unchecked (c : *mut connection,
                                                         context_tag :  context_tag,
                                                         target :  u32,
                                                         pname :  u32) -> get_convolution_parameteriv_cookie;

pub fn xcb_glx_get_convolution_parameteriv_data (R : *mut get_convolution_parameteriv_reply) -> *mut i32;


pub fn xcb_glx_get_convolution_parameteriv_data_length (R : *mut get_convolution_parameteriv_reply) -> c_int;


pub fn xcb_glx_get_convolution_parameteriv_data_end (R : *mut get_convolution_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_convolution_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_convolution_parameteriv_reply (c : *mut connection,
                                                     cookie : get_convolution_parameteriv_cookie,
                                                     e : *mut *mut generic_error) -> *mut get_convolution_parameteriv_reply;

pub fn xcb_glx_get_separable_filter_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_separable_filter (c : *mut connection,
                                        context_tag :  context_tag,
                                        target :  u32,
                                        format :  u32,
                                        type_ :  u32,
                                        swap_bytes :  u8) -> get_separable_filter_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_separable_filter_unchecked (c : *mut connection,
                                                  context_tag :  context_tag,
                                                  target :  u32,
                                                  format :  u32,
                                                  type_ :  u32,
                                                  swap_bytes :  u8) -> get_separable_filter_cookie;

pub fn xcb_glx_get_separable_filter_rows_and_cols (R : *mut get_separable_filter_reply) -> *mut u8;


pub fn xcb_glx_get_separable_filter_rows_and_cols_length (R : *mut get_separable_filter_reply) -> c_int;


pub fn xcb_glx_get_separable_filter_rows_and_cols_end (R : *mut get_separable_filter_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_separable_filter_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_separable_filter_reply (c : *mut connection,
                                              cookie : get_separable_filter_cookie,
                                              e : *mut *mut generic_error) -> *mut get_separable_filter_reply;

pub fn xcb_glx_get_histogram_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_histogram (c : *mut connection,
                                 context_tag :  context_tag,
                                 target :  u32,
                                 format :  u32,
                                 type_ :  u32,
                                 swap_bytes :  u8,
                                 reset :  u8) -> get_histogram_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_histogram_unchecked (c : *mut connection,
                                           context_tag :  context_tag,
                                           target :  u32,
                                           format :  u32,
                                           type_ :  u32,
                                           swap_bytes :  u8,
                                           reset :  u8) -> get_histogram_cookie;

pub fn xcb_glx_get_histogram_data (R : *mut get_histogram_reply) -> *mut u8;


pub fn xcb_glx_get_histogram_data_length (R : *mut get_histogram_reply) -> c_int;


pub fn xcb_glx_get_histogram_data_end (R : *mut get_histogram_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_histogram_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_histogram_reply (c : *mut connection,
                                       cookie : get_histogram_cookie,
                                       e : *mut *mut generic_error) -> *mut get_histogram_reply;

pub fn xcb_glx_get_histogram_parameterfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_histogram_parameterfv (c : *mut connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             pname :  u32) -> get_histogram_parameterfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_histogram_parameterfv_unchecked (c : *mut connection,
                                                       context_tag :  context_tag,
                                                       target :  u32,
                                                       pname :  u32) -> get_histogram_parameterfv_cookie;

pub fn xcb_glx_get_histogram_parameterfv_data (R : *mut get_histogram_parameterfv_reply) -> *mut float32;


pub fn xcb_glx_get_histogram_parameterfv_data_length (R : *mut get_histogram_parameterfv_reply) -> c_int;


pub fn xcb_glx_get_histogram_parameterfv_data_end (R : *mut get_histogram_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_histogram_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_histogram_parameterfv_reply (c : *mut connection,
                                                   cookie : get_histogram_parameterfv_cookie,
                                                   e : *mut *mut generic_error) -> *mut get_histogram_parameterfv_reply;

pub fn xcb_glx_get_histogram_parameteriv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_histogram_parameteriv (c : *mut connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             pname :  u32) -> get_histogram_parameteriv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_histogram_parameteriv_unchecked (c : *mut connection,
                                                       context_tag :  context_tag,
                                                       target :  u32,
                                                       pname :  u32) -> get_histogram_parameteriv_cookie;

pub fn xcb_glx_get_histogram_parameteriv_data (R : *mut get_histogram_parameteriv_reply) -> *mut i32;


pub fn xcb_glx_get_histogram_parameteriv_data_length (R : *mut get_histogram_parameteriv_reply) -> c_int;


pub fn xcb_glx_get_histogram_parameteriv_data_end (R : *mut get_histogram_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_histogram_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_histogram_parameteriv_reply (c : *mut connection,
                                                   cookie : get_histogram_parameteriv_cookie,
                                                   e : *mut *mut generic_error) -> *mut get_histogram_parameteriv_reply;

pub fn xcb_glx_get_minmax_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_minmax (c : *mut connection,
                              context_tag :  context_tag,
                              target :  u32,
                              format :  u32,
                              type_ :  u32,
                              swap_bytes :  u8,
                              reset :  u8) -> get_minmax_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_minmax_unchecked (c : *mut connection,
                                        context_tag :  context_tag,
                                        target :  u32,
                                        format :  u32,
                                        type_ :  u32,
                                        swap_bytes :  u8,
                                        reset :  u8) -> get_minmax_cookie;

pub fn xcb_glx_get_minmax_data (R : *mut get_minmax_reply) -> *mut u8;


pub fn xcb_glx_get_minmax_data_length (R : *mut get_minmax_reply) -> c_int;


pub fn xcb_glx_get_minmax_data_end (R : *mut get_minmax_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_minmax_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_minmax_reply (c : *mut connection,
                                    cookie : get_minmax_cookie,
                                    e : *mut *mut generic_error) -> *mut get_minmax_reply;

pub fn xcb_glx_get_minmax_parameterfv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_minmax_parameterfv (c : *mut connection,
                                          context_tag :  context_tag,
                                          target :  u32,
                                          pname :  u32) -> get_minmax_parameterfv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_minmax_parameterfv_unchecked (c : *mut connection,
                                                    context_tag :  context_tag,
                                                    target :  u32,
                                                    pname :  u32) -> get_minmax_parameterfv_cookie;

pub fn xcb_glx_get_minmax_parameterfv_data (R : *mut get_minmax_parameterfv_reply) -> *mut float32;


pub fn xcb_glx_get_minmax_parameterfv_data_length (R : *mut get_minmax_parameterfv_reply) -> c_int;


pub fn xcb_glx_get_minmax_parameterfv_data_end (R : *mut get_minmax_parameterfv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_minmax_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_minmax_parameterfv_reply (c : *mut connection,
                                                cookie : get_minmax_parameterfv_cookie,
                                                e : *mut *mut generic_error) -> *mut get_minmax_parameterfv_reply;

pub fn xcb_glx_get_minmax_parameteriv_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_minmax_parameteriv (c : *mut connection,
                                          context_tag :  context_tag,
                                          target :  u32,
                                          pname :  u32) -> get_minmax_parameteriv_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_minmax_parameteriv_unchecked (c : *mut connection,
                                                    context_tag :  context_tag,
                                                    target :  u32,
                                                    pname :  u32) -> get_minmax_parameteriv_cookie;

pub fn xcb_glx_get_minmax_parameteriv_data (R : *mut get_minmax_parameteriv_reply) -> *mut i32;


pub fn xcb_glx_get_minmax_parameteriv_data_length (R : *mut get_minmax_parameteriv_reply) -> c_int;


pub fn xcb_glx_get_minmax_parameteriv_data_end (R : *mut get_minmax_parameteriv_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_minmax_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_minmax_parameteriv_reply (c : *mut connection,
                                                cookie : get_minmax_parameteriv_cookie,
                                                e : *mut *mut generic_error) -> *mut get_minmax_parameteriv_reply;

pub fn xcb_glx_get_compressed_tex_image_arb_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_compressed_tex_image_arb (c : *mut connection,
                                                context_tag :  context_tag,
                                                target :  u32,
                                                level :  i32) -> get_compressed_tex_image_arb_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_compressed_tex_image_arb_unchecked (c : *mut connection,
                                                          context_tag :  context_tag,
                                                          target :  u32,
                                                          level :  i32) -> get_compressed_tex_image_arb_cookie;

pub fn xcb_glx_get_compressed_tex_image_arb_data (R : *mut get_compressed_tex_image_arb_reply) -> *mut u8;


pub fn xcb_glx_get_compressed_tex_image_arb_data_length (R : *mut get_compressed_tex_image_arb_reply) -> c_int;


pub fn xcb_glx_get_compressed_tex_image_arb_data_end (R : *mut get_compressed_tex_image_arb_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_compressed_tex_image_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_compressed_tex_image_arb_reply (c : *mut connection,
                                                      cookie : get_compressed_tex_image_arb_cookie,
                                                      e : *mut *mut generic_error) -> *mut get_compressed_tex_image_arb_reply;

pub fn xcb_glx_delete_queries_arb_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_glx_delete_queries_arb_checked (c : *mut connection,
                                              context_tag :  context_tag,
                                              n :  i32,
                                              ids : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_delete_queries_arb (c : *mut connection,
                                      context_tag :  context_tag,
                                      n :  i32,
                                      ids : *mut u32) -> void_cookie;

pub fn xcb_glx_gen_queries_arb_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_gen_queries_arb (c : *mut connection,
                                   context_tag :  context_tag,
                                   n :  i32) -> gen_queries_arb_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_gen_queries_arb_unchecked (c : *mut connection,
                                             context_tag :  context_tag,
                                             n :  i32) -> gen_queries_arb_cookie;

pub fn xcb_glx_gen_queries_arb_data (R : *mut gen_queries_arb_reply) -> *mut u32;


pub fn xcb_glx_gen_queries_arb_data_length (R : *mut gen_queries_arb_reply) -> c_int;


pub fn xcb_glx_gen_queries_arb_data_end (R : *mut gen_queries_arb_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_gen_queries_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_gen_queries_arb_reply (c : *mut connection,
                                         cookie : gen_queries_arb_cookie,
                                         e : *mut *mut generic_error) -> *mut gen_queries_arb_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_is_query_arb (c : *mut connection,
                                context_tag :  context_tag,
                                id :  u32) -> is_query_arb_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_is_query_arb_unchecked (c : *mut connection,
                                          context_tag :  context_tag,
                                          id :  u32) -> is_query_arb_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_query_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_is_query_arb_reply (c : *mut connection,
                                      cookie : is_query_arb_cookie,
                                      e : *mut *mut generic_error) -> *mut is_query_arb_reply;

pub fn xcb_glx_get_queryiv_arb_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_queryiv_arb (c : *mut connection,
                                   context_tag :  context_tag,
                                   target :  u32,
                                   pname :  u32) -> get_queryiv_arb_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_queryiv_arb_unchecked (c : *mut connection,
                                             context_tag :  context_tag,
                                             target :  u32,
                                             pname :  u32) -> get_queryiv_arb_cookie;

pub fn xcb_glx_get_queryiv_arb_data (R : *mut get_queryiv_arb_reply) -> *mut i32;


pub fn xcb_glx_get_queryiv_arb_data_length (R : *mut get_queryiv_arb_reply) -> c_int;


pub fn xcb_glx_get_queryiv_arb_data_end (R : *mut get_queryiv_arb_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_queryiv_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_queryiv_arb_reply (c : *mut connection,
                                         cookie : get_queryiv_arb_cookie,
                                         e : *mut *mut generic_error) -> *mut get_queryiv_arb_reply;

pub fn xcb_glx_get_query_objectiv_arb_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_query_objectiv_arb (c : *mut connection,
                                          context_tag :  context_tag,
                                          id :  u32,
                                          pname :  u32) -> get_query_objectiv_arb_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_query_objectiv_arb_unchecked (c : *mut connection,
                                                    context_tag :  context_tag,
                                                    id :  u32,
                                                    pname :  u32) -> get_query_objectiv_arb_cookie;

pub fn xcb_glx_get_query_objectiv_arb_data (R : *mut get_query_objectiv_arb_reply) -> *mut i32;


pub fn xcb_glx_get_query_objectiv_arb_data_length (R : *mut get_query_objectiv_arb_reply) -> c_int;


pub fn xcb_glx_get_query_objectiv_arb_data_end (R : *mut get_query_objectiv_arb_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_query_objectiv_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_query_objectiv_arb_reply (c : *mut connection,
                                                cookie : get_query_objectiv_arb_cookie,
                                                e : *mut *mut generic_error) -> *mut get_query_objectiv_arb_reply;

pub fn xcb_glx_get_query_objectuiv_arb_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_glx_get_query_objectuiv_arb (c : *mut connection,
                                           context_tag :  context_tag,
                                           id :  u32,
                                           pname :  u32) -> get_query_objectuiv_arb_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_glx_get_query_objectuiv_arb_unchecked (c : *mut connection,
                                                     context_tag :  context_tag,
                                                     id :  u32,
                                                     pname :  u32) -> get_query_objectuiv_arb_cookie;

pub fn xcb_glx_get_query_objectuiv_arb_data (R : *mut get_query_objectuiv_arb_reply) -> *mut u32;


pub fn xcb_glx_get_query_objectuiv_arb_data_length (R : *mut get_query_objectuiv_arb_reply) -> c_int;


pub fn xcb_glx_get_query_objectuiv_arb_data_end (R : *mut get_query_objectuiv_arb_reply) -> generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_query_objectuiv_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_glx_get_query_objectuiv_arb_reply (c : *mut connection,
                                                 cookie : get_query_objectuiv_arb_cookie,
                                                 e : *mut *mut generic_error) -> *mut get_query_objectuiv_arb_reply;
}

