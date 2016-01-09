/*
 * This file generated automatically from damage.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(unused_unsafe)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use base;
use base::*;
use ffi;
use ffi::damage::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
use render;
use shape;
use xfixes;
pub type Damage = damage;

pub type DamageIterator = damage_iterator;


pub type report_level = c_uint;//{
    pub static XCB_DAMAGE_REPORT_LEVEL_RAW_RECTANGLES : report_level = 1;
    pub static XCB_DAMAGE_REPORT_LEVEL_DELTA_RECTANGLES : report_level = 2;
    pub static XCB_DAMAGE_REPORT_LEVEL_BOUNDING_BOX : report_level = 3;
    pub static XCB_DAMAGE_REPORT_LEVEL_NON_EMPTY : report_level = 4;
//}
/** Opcode for xcb_damage_bad_damage. */
pub static XCB_DAMAGE_BAD_DAMAGE : u8 = 0;
pub struct BadDamageError { pub base : base::Error<bad_damage_error> }
pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

/** Opcode for xcb_damage_query_version. */
pub static XCB_DAMAGE_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
fn mk_reply_query_version_reply(reply:*mut query_version_reply) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_damage_create. */
pub static XCB_DAMAGE_CREATE : u8 = 1;
/** Opcode for xcb_damage_destroy. */
pub static XCB_DAMAGE_DESTROY : u8 = 2;
/** Opcode for xcb_damage_subtract. */
pub static XCB_DAMAGE_SUBTRACT : u8 = 3;
/** Opcode for xcb_damage_add. */
pub static XCB_DAMAGE_ADD : u8 = 4;
/** Opcode for xcb_damage_notify. */
pub static XCB_DAMAGE_NOTIFY : u8 = 0;
pub struct NotifyEvent {pub base : base::Event<notify_event>}

impl Iterator for DamageIterator {
    type Item = Damage;
    fn next(&mut self) -> Option<Damage> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut damage_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_damage_damage_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection,
                     client_major_version : u32,
                     client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_damage_query_version(c.get_raw_conn(),
        client_major_version as u32, //1
        client_minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major_version : u32,
                              client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_damage_query_version_unchecked(c.get_raw_conn(),
        client_major_version as u32, //1
        client_minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn major_version(&mut self) -> u32 {
    unsafe { accessor!(major_version -> u32, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u32 {
    unsafe { accessor!(minor_version -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_damage_query_version_reply);

pub fn CreateChecked<'r> (c : &'r Connection,
                      damage : Damage,
                      drawable : xproto::Drawable,
                      level : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_damage_create_checked(c.get_raw_conn(),
        damage as damage, //1
        drawable as ffi::xproto::drawable, //2
        level as u8); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Create<'r> (c : &'r Connection,
               damage : Damage,
               drawable : xproto::Drawable,
               level : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_damage_create(c.get_raw_conn(),
        damage as damage, //1
        drawable as ffi::xproto::drawable, //2
        level as u8); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DestroyChecked<'r> (c : &'r Connection,
                       damage : Damage) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_damage_destroy_checked(c.get_raw_conn(),
        damage as damage); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Destroy<'r> (c : &'r Connection,
                damage : Damage) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_damage_destroy(c.get_raw_conn(),
        damage as damage); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SubtractChecked<'r> (c : &'r Connection,
                        damage : Damage,
                        repair : xfixes::Region,
                        parts : xfixes::Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_damage_subtract_checked(c.get_raw_conn(),
        damage as damage, //1
        repair as ffi::xfixes::region, //2
        parts as ffi::xfixes::region); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Subtract<'r> (c : &'r Connection,
                 damage : Damage,
                 repair : xfixes::Region,
                 parts : xfixes::Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_damage_subtract(c.get_raw_conn(),
        damage as damage, //1
        repair as ffi::xfixes::region, //2
        parts as ffi::xfixes::region); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AddChecked<'r> (c : &'r Connection,
                   drawable : xproto::Drawable,
                   region : xfixes::Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_damage_add_checked(c.get_raw_conn(),
        drawable as ffi::xproto::drawable, //1
        region as ffi::xfixes::region); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Add<'r> (c : &'r Connection,
            drawable : xproto::Drawable,
            region : xfixes::Region) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_damage_add(c.get_raw_conn(),
        drawable as ffi::xproto::drawable, //1
        region as ffi::xfixes::region); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl NotifyEvent {
  pub fn level(&mut self) -> u8 {
    unsafe { accessor!(level -> u8, (*self.base.event)) }
  }

  pub fn drawable(&mut self) -> xproto::Drawable {
    unsafe { accessor!(drawable -> xproto::Drawable, (*self.base.event)) }
  }

  pub fn damage(&mut self) -> Damage {
    unsafe { accessor!(damage -> Damage, (*self.base.event)) }
  }

  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn area(&self) -> xproto::Rectangle {
    unsafe { mem::transmute((*self.base.event).area) }
  }
  pub fn geometry(&self) -> xproto::Rectangle {
    unsafe { mem::transmute((*self.base.event).geometry) }
  }
  pub fn new(level : u8,
         drawable : xproto::Drawable,
         damage : Damage,
         timestamp : xproto::Timestamp,
         area : xproto::Rectangle,
         geometry : xproto::Rectangle) -> NotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut notify_event;
      (*raw).level = level;
      (*raw).drawable = drawable;
      (*raw).damage = damage;
      (*raw).timestamp = timestamp;
      (*raw).area = area.base.strct;
      (*raw).geometry = geometry.base.strct;
      NotifyEvent { base : Event { event : raw as *mut notify_event }}
    }
  }
}
