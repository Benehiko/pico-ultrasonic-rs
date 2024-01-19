// Copyright (c) 2017 The Robigalia Project Developers Licensed under the Apache License, Version
// 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. All files in the project
// carrying such notice may not be copied, modified, or distributed except according to those
// terms.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

extern crate serde;
extern crate encode_unicode;

use core::intrinsics::transmute;

use serde::{Serialize, Deserialize};
use serde::de::{Visitor, DeserializeSeed, DeserializeOwned, IntoDeserializer};
//use serde::de::value::ValueDeserializer;
use encode_unicode::CharExt;
use encode_unicode::Utf8Char;

use core::fmt::Display;

const NS: &'static str = "not support";

#[inline(never)]
#[cold]
fn ns<T>() -> Result<T, Error> {
    if cfg!(debug_assertions) {
        panic!(NS)
    }
    Err(Error::NotSupported)
}

#[derive(Debug)]
pub enum Error {
    EndOfStream,
    InvalidRepresentation,
    MoreElements,
    TooManyVariants,
    NotSupported,
    ApplicationError(&'static str),
    #[cfg(not(feature = "std"))]
    Custom,
    #[cfg(feature = "std")]
    Custom(String),
}


impl core::fmt::Display for Error {
     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        #[cfg(feature = "std")]
        use std::error::Error;

        match self {
            &::Error::ApplicationError(s) => write!(f, "application error: {}", s),
            _ => f.write_str(self.description()),
        }
    }
}

#[cfg(not(feature = "std"))]
impl serde::de::Error for Error {
     fn custom<T: Display>(_msg: T) -> Error {
        Error::Custom
    }
}

#[cfg(not(feature = "std"))]
impl serde::ser::Error for Error {
     fn custom<T: Display>(_msg: T) -> Error {
        Error::Custom
    }
}

#[cfg(feature = "std")]
impl serde::de::Error for Error {
     fn custom<T: Display>(msg: T) -> Error {
        Error::Custom(format!("{}", msg))
    }
}

#[cfg(feature = "std")]
impl serde::ser::Error for Error {
     fn custom<T: Display>(msg: T) -> Error {
        Error::Custom(format!("{}", msg))
    }
}

#[cfg(not(feature = "std"))]
impl Error {
     fn description(&self) -> &str {
        match self {
            &Error::EndOfStream => "end of stream reached but more data was needed",
            &Error::InvalidRepresentation => "invalid representation for a value",
            &Error::MoreElements => "there are more elements of the sequence remaining",
            &Error::TooManyVariants => "too many variants, only up to 256 are supported",
            &Error::NotSupported => "feature not supported",
            &Error::ApplicationError(s) => s,
            &Error::Custom => "some custom error that couldn't be reported",
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
     fn description(&self) -> &str {
        match self {
            &Error::EndOfStream => "end of stream reached but more data was needed",
            &Error::InvalidRepresentation => "invalid representation for a value",
            &Error::MoreElements => "there are more elements of the sequence remaining",
            &Error::TooManyVariants => "too many variants, only up to 256 are supported",
            &Error::NotSupported => "feature not supported",
            &Error::ApplicationError(s) => s,
            &Error::Custom(ref s) => &s
        }
    }
}

/// Serialize a value into a buffer. Returns the number of bytes used.
pub fn serialize<T: Serialize>(buf: &mut [u8], val: &T) -> SerializeResult<usize> {
    let mut serializer = Serializer { buf: buf, idx: 0 };
    T::serialize(val, &mut serializer)?;
    debug_assert!(serializer.idx <= core::mem::size_of::<T>(), "{} <=? {}", serializer.idx, core::mem::size_of::<T>());
    Ok(serializer.idx)
}

/// Deserialize a value from a buffer. Returns the number of bytes used.
pub fn deserialize<T: DeserializeOwned>(buf: &[u8]) -> SerializeResult<(T, usize)> {
    let mut deserializer = Deserializer { buf: buf, idx: 0 };
    let val = T::deserialize(&mut deserializer)?;
    debug_assert!(deserializer.idx <= core::mem::size_of::<T>());
    Ok((val, deserializer.idx))
}

struct Serializer<'a> {
    buf: &'a mut [u8],
    idx: usize,
}

impl<'a> Serializer<'a> {
    #[inline]
     fn check_bounds(&self, len: usize) -> Result<(), Error> {
        if let Some(val) = self.idx.checked_add(len) {
            if val <= self.buf.len() {
                return Ok(());
            }
        }
        debug_assert!(false, "ran out of space serializing value; fix your buffer size");
        Err(Error::EndOfStream)
    }

    #[inline]
     fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.check_bounds(1)?;
        unsafe {
            *self.buf.get_unchecked_mut(self.idx) = val;
        }
        self.idx += 1;
        Ok(())
    }

    #[inline]
     fn write_u16(&mut self, val: u16) -> Result<(), Error> {
        self.check_bounds(2)?;
        unsafe {
            *self.buf.get_unchecked_mut(self.idx) = (val & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 1) = (val >> 8 & 0xFF) as u8;
        }
        self.idx += 2;
        Ok(())
    }

    #[inline]
     fn write_u32(&mut self, val: u32) -> Result<(), Error> {
        self.check_bounds(4)?;
        unsafe {
            *self.buf.get_unchecked_mut(self.idx) = (val & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 1) = (val >> 8 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 2) = (val >> 16 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 3) = (val >> 24 & 0xFF) as u8;
        }
        self.idx += 4;
        Ok(())
    }

    #[inline]
     fn write_u64(&mut self, val: u64) -> Result<(), Error> {
        self.check_bounds(8)?;
        unsafe { 
            *self.buf.get_unchecked_mut(self.idx) = (val & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 1) = (val >> 8 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 2) = (val >> 16 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 3) = (val >> 24 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 4) = (val >> 32 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 5) = (val >> 40 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 6) = (val >> 48 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.idx + 7) = (val >> 56 & 0xFF) as u8;
        }
        self.idx += 8;
        Ok(())
    }

    #[inline]
     fn write_usize(&mut self, val: usize) -> Result<(), Error> {
        self.write_u64(val as u64)
    }
}

type SerializeResult<T> = Result<T, Error>;

impl<'b, 'a: 'b> serde::Serializer for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = serde::ser::Impossible<(), Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;


    #[inline]
     fn serialize_bool(self, v: bool) -> SerializeResult<()> {
        self.write_u8(if v { 1 } else { 0 })
    }

    #[inline]
     fn serialize_u8(self, v: u8) -> SerializeResult<()> {
        self.write_u8(v)
    }

    #[inline]
     fn serialize_u16(self, v: u16) -> SerializeResult<()> {
        self.write_u16(v)
    }

    #[inline]
     fn serialize_u32(self, v: u32) -> SerializeResult<()> {
        self.write_u32(v)
    }

    #[inline]
     fn serialize_u64(self, v: u64) -> SerializeResult<()> {
        self.write_u64(v)
    }

    #[inline]
     fn serialize_i8(self, v: i8) -> SerializeResult<()> {
        self.write_u8(v as u8)
    }

    #[inline]
     fn serialize_i16(self, v: i16) -> SerializeResult<()> {
        self.write_u16(v as u16)
    }

    #[inline]
     fn serialize_i32(self, v: i32) -> SerializeResult<()> {
        self.write_u32(v as u32)
    }

    #[inline]
     fn serialize_i64(self, v: i64) -> SerializeResult<()> {
        self.write_u64(v as u64)
    }

    #[inline]
     fn serialize_f32(self, v: f32) -> SerializeResult<()> {
        self.write_u32(unsafe { transmute(v) })
    }

    #[inline]
     fn serialize_f64(self, v: f64) -> SerializeResult<()> {
        self.write_u64(unsafe { transmute(v) })
    }

     fn serialize_str(self, _: &str) -> SerializeResult<()> {
        ns()
    }

     fn serialize_char(self, c: char) -> SerializeResult<()> {
        let (arr, sz) = c.to_utf8_array();
        self.check_bounds(sz)?;
        for (i, c) in arr[..sz].iter().enumerate() {
            unsafe {
                *self.buf.get_unchecked_mut(self.idx + i) = *c;
            }
        }
        self.idx += sz;
        Ok(())
    }

     fn serialize_bytes(self, _: &[u8]) -> SerializeResult<()> {
        ns()
    }

    #[inline]
     fn serialize_none(self) -> SerializeResult<()> {
        self.write_u8(0)
    }

     fn serialize_some<T: Serialize + ?Sized>(self, v: &T) -> SerializeResult<()> {
        self.write_u8(1)?;
        v.serialize(self)
    }

    #[inline]
     fn serialize_unit(self) -> SerializeResult<()> {
        Ok(())
    }

    #[inline]
     fn serialize_unit_struct(self, _: &'static str) -> SerializeResult<()> {
        Ok(())
    }

    #[inline]
     fn serialize_unit_variant(self,
                              _name: &'static str,
                              variant_index: u32,
                              _variant: &'static str)
                              -> SerializeResult<()> {
        if variant_index > 255 {
            debug_assert!(false, "too many enum variants: {}", _name);
            return Err(Error::TooManyVariants)
        }
        self.write_u8(variant_index as u8)
    }

     fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _name: &'static str, value: &T) -> SerializeResult<()> {
        value.serialize(self)
    }

     fn serialize_newtype_variant<T: Serialize + ?Sized>(self,
                                    _name: &'static str,
                                    variant_index: u32,
                                    _variant: &'static str,
                                    value: &T)
                                    -> SerializeResult<()> {
        if variant_index > 255 {
            debug_assert!(false, "too many enum variants: {}", _name);
            return Err(Error::TooManyVariants);
        }
        self.write_u8(variant_index as u8)?;
        value.serialize(self)
    }

     #[inline]
     fn serialize_seq(self, len: Option<usize>) -> SerializeResult<Self> {
        match len {
            None => {
                ns()
            },
            Some(len) => {
                self.write_usize(len)?;
                Ok(self)
            }
        }
    }

     #[inline]
     fn serialize_tuple(self, _len: usize) -> SerializeResult<Self> {
        Ok(self)
    }

     #[inline]
     fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> SerializeResult<Self> {
        Ok(self)
    }

     #[inline]
     fn serialize_tuple_variant(self,
                               _name: &'static str,
                               variant_index: u32,
                               _variant: &'static str,
                               _len: usize)
                               -> SerializeResult<Self> {
        if variant_index > 255 {
            debug_assert!(false, "too many enum variants: {}", _name);
            return Err(Error::TooManyVariants);
        }
        self.write_u8(variant_index as u8)?;
        Ok(self)
    }

     fn serialize_map(self, _len: Option<usize>) -> SerializeResult<Self::SerializeMap> {
        ns()
    }

     #[inline]
     fn serialize_struct(self, _name: &'static str, _len: usize) -> SerializeResult<Self> {
        Ok(self)
    }

     #[inline]
     fn serialize_struct_variant(self,
                                _name: &'static str,
                                variant_index: u32,
                                _variant: &'static str,
                                _len: usize)
                                -> SerializeResult<Self> {
        if variant_index > 255 {
            debug_assert!(false, "too many enum variants: {}", _name);
            return Err(Error::TooManyVariants);
        }
        self.write_u8(variant_index as u8)?;
        Ok(self)
    }

     #[cfg(not(feature = "std"))]
     fn collect_str<T: Display + ?Sized>(self, _value: &T) -> SerializeResult<()> {
         ns()
     }
}

impl<'b, 'a: 'b> serde::ser::SerializeSeq for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

     fn serialize_element<V: Serialize + ?Sized>(&mut self, value: &V) -> SerializeResult<()> {
        value.serialize(&mut **self)
    }
    
     #[inline]
     fn end(self) -> SerializeResult<()> { Ok(()) }
}

impl<'b, 'a: 'b> serde::ser::SerializeTuple for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

     fn serialize_element<V: Serialize + ?Sized>(&mut self, value: &V) -> SerializeResult<()> {
        value.serialize(&mut **self)
    }

     #[inline]
     fn end(self) -> SerializeResult<()> { Ok(()) }
}

impl<'b, 'a: 'b> serde::ser::SerializeTupleStruct for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), Error> {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), Error> { Ok(()) }
}

impl<'b, 'a: 'b> serde::ser::SerializeTupleVariant for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<V: Serialize + ?Sized>(&mut self, value: &V) -> Result<(), Error> {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), Error> { Ok(()) }
}

impl<'b, 'a: 'b> serde::ser::SerializeStruct for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<V: Serialize + ?Sized>(&mut self, _: &'static str, value: &V) -> Result<(), Error> {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'b, 'a: 'b> serde::ser::SerializeStructVariant for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<V: Serialize + ?Sized>(&mut self, _: &'static str, value: &V) -> Result<(), Error> {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), Error> { Ok(()) }
}

struct Deserializer<'a> {
    buf: &'a [u8],
    idx: usize,
}

impl<'a> Deserializer<'a> {
    #[inline]
     fn check_bounds(&self, len: usize) -> Result<(), Error> {
        if let Some(val) = self.idx.checked_add(len) {
            if val <= self.buf.len() {
                return Ok(());
            }
        }
        debug_assert!(false, "ran out of space deserializing value; fix your buffer size");
        Err(Error::EndOfStream)
    }

     #[inline]
     fn read_u8(&mut self) -> Result<u8, Error> {
        self.check_bounds(1)?;
        let val = unsafe {
            *self.buf.get_unchecked(self.idx)
        };
        self.idx += 1;
        Ok(val)
    }

     #[inline]
     fn read_u16(&mut self) -> Result<u16, Error> {
        self.check_bounds(2)?;
        let mut val;
        unsafe {
            val = *self.buf.get_unchecked(self.idx) as u16;
            val |= (*self.buf.get_unchecked(self.idx + 1) as u16) << 8;
        }
        self.idx += 2;
        Ok(val)
    }

     #[inline]
     fn read_u32(&mut self) -> Result<u32, Error> {
        self.check_bounds(4)?;
        let mut val;
        unsafe {
            val = *self.buf.get_unchecked(self.idx) as u32;
            val |= (*self.buf.get_unchecked(self.idx + 1) as u32) << 8;
            val |= (*self.buf.get_unchecked(self.idx + 2) as u32) << 16;
            val |= (*self.buf.get_unchecked(self.idx + 3) as u32) << 24;
        }
        self.idx += 4;
        Ok(val)
    }

     #[inline]
     fn read_u64(&mut self) -> Result<u64, Error> {
        self.check_bounds(8)?;
        let mut val;
        unsafe {
            val = *self.buf.get_unchecked(self.idx) as u64;
            val |= (*self.buf.get_unchecked(self.idx + 1) as u64) << 8;
            val |= (*self.buf.get_unchecked(self.idx + 2) as u64) << 16;
            val |= (*self.buf.get_unchecked(self.idx + 3) as u64) << 24;
            val |= (*self.buf.get_unchecked(self.idx + 4) as u64) << 32;
            val |= (*self.buf.get_unchecked(self.idx + 5) as u64) << 40;
            val |= (*self.buf.get_unchecked(self.idx + 6) as u64) << 48;
            val |= (*self.buf.get_unchecked(self.idx + 7) as u64) << 56;
        }
        self.idx += 8;
        Ok(val)
    }
}


struct SeqAccess<'a, 'b: 'a> {
    deserializer: &'a mut Deserializer<'b>,
    len: usize,
}

impl<'a, 'b: 'a> serde::de::SeqAccess<'b> for SeqAccess<'a, 'b> {
    type Error = Error;

     fn next_element_seed<V: DeserializeSeed<'b>>(&mut self, seed: V) -> Result<Option<V::Value>, Error> {
        if self.len > 0 {
            self.len -= 1;
            Ok(Some(DeserializeSeed::deserialize(seed, &mut *self.deserializer)?))
        } else {
            Ok(None)
        }
    }

     fn size_hint(&self) -> Option<usize> {
         Some(self.len)
     }
}

type DeserializeResult<T> = Result<T, Error>;

impl<'b, 'de: 'b> serde::Deserializer<'de> for &'b mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> DeserializeResult<V::Value> {
        ns()
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        let value: u8 = Deserialize::deserialize(self)?;
        match value {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            _ => Err(Error::InvalidRepresentation),
        }
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_u8(self.read_u8()?)
    }

     fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_u16(self.read_u16()?)
    }

     fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_u32(self.read_u32()?)
    }

     fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_u64(self.read_u64()?)
    }

     fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_i8(self.read_u8()? as i8)
    }

     fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_i16(self.read_u16()? as i16)
    }

     fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_i32(self.read_u32()? as i32)
    }

     fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_i64(self.read_u64()? as i64)
    }

     fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_f32(unsafe { transmute(self.read_u32()?) })
    }
     fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_f64(unsafe { transmute(self.read_u64()?) })
    }

     fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        match Utf8Char::from_slice_start(&self.buf[self.idx..]) {
            Ok((c, count)) => {
                // this ought to be correct, if it weren't how did from_slice_start do its thing?
                self.idx = self.idx.wrapping_add(count); 
                visitor.visit_char(c.to_char())
            },
            Err(_) => Err(Error::InvalidRepresentation),
        }
    }

    fn deserialize_str<V: Visitor<'de>>(self, _visitor: V) -> DeserializeResult<V::Value> {
        ns()
    }

    fn deserialize_string<V: Visitor<'de>>(self, _visitor: V) -> DeserializeResult<V::Value> {
        ns()
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, _visitor: V) -> DeserializeResult<V::Value> {
        ns()
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, _visitor: V) -> DeserializeResult<V::Value> {
        ns()
    }

     fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        let value: u8 = Deserialize::deserialize(&mut *self)?;
        match value {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(Error::InvalidRepresentation),
        }
    }

     fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_unit()
    }

     fn deserialize_unit_struct<V: Visitor<'de>>(self,
                                  _name: &'static str,
                                  visitor: V)
                                  -> DeserializeResult<V::Value> {
        visitor.visit_unit()
    }

     fn deserialize_newtype_struct<V: Visitor<'de>>(self,
                                     _name: &str,
                                     visitor: V)
                                     -> DeserializeResult<V::Value> {
        visitor.visit_newtype_struct(self)
    }

     fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> DeserializeResult<V::Value> {
        let len = Deserialize::deserialize(&mut *self)?;

        visitor.visit_seq(SeqAccess {
            deserializer: self,
            len: len,
        })
    }

     fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> DeserializeResult<V::Value> {
        visitor.visit_seq(SeqAccess { deserializer: self, len: len })
    }

     fn deserialize_tuple_struct<V: Visitor<'de>>(self,
                                   _name: &'static str,
                                   len: usize,
                                   visitor: V)
                                   -> DeserializeResult<V::Value> {
        self.deserialize_tuple(len, visitor)
    }

     fn deserialize_map<V: Visitor<'de>>(self, _visitor: V) -> DeserializeResult<V::Value> {
        Err(Error::NotSupported)
    }

     fn deserialize_struct<V: Visitor<'de>>(self,
                             _name: &str,
                             fields: &'static [&'static str],
                             visitor: V)
                             -> DeserializeResult<V::Value> {
        self.deserialize_tuple(fields.len(), visitor)
    }

     fn deserialize_enum<V: Visitor<'de>>(self,
                           _enum: &'static str,
                           _variants: &'static [&'static str],
                           visitor: V)
                           -> Result<V::Value, Error> {
        visitor.visit_enum(self)
    }

    
    fn deserialize_identifier<V: Visitor<'de>>(self, _visitor: V) -> DeserializeResult<V::Value> {
        // not the panic because it seems noone cares about these?
        Err(Error::NotSupported)
    }

     fn deserialize_ignored_any<V: Visitor<'de>>(self, _visitor: V) -> DeserializeResult<V::Value> {
        ns()
    }
}

impl<'b, 'de: 'b> serde::de::VariantAccess<'de> for &'b mut Deserializer<'de> {
    type Error = Error;

     fn unit_variant(self) -> Result<(), Error> {
        Ok(())
    }

     fn newtype_variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> DeserializeResult<V::Value> {
        DeserializeSeed::deserialize(seed, self)
    }

     fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> DeserializeResult<V::Value> {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

     fn struct_variant<V: Visitor<'de>>(self,
                       fields: &'static [&'static str],
                       visitor: V)
                       -> Result<V::Value, Error> {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}

impl<'b, 'de: 'b> serde::de::EnumAccess<'de> for &'b mut Deserializer<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> DeserializeResult<(V::Value, Self)> {
        let x: u8 = Deserialize::deserialize(&mut *self)?;
        let v = DeserializeSeed::deserialize(seed, (x as u32).into_deserializer())?;
        Ok((v, self))
    }
}

