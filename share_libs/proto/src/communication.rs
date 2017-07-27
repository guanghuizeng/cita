// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    pub cmd_id: u32,
    pub field_type: MsgType,
    pub origin: u32,
    pub operate: OperateType,
    pub content: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // uint32 cmd_id = 1;

    pub fn clear_cmd_id(&mut self) {
        self.cmd_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_cmd_id(&mut self, v: u32) {
        self.cmd_id = v;
    }

    pub fn get_cmd_id(&self) -> u32 {
        self.cmd_id
    }

    fn get_cmd_id_for_reflect(&self) -> &u32 {
        &self.cmd_id
    }

    fn mut_cmd_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.cmd_id
    }

    // .MsgType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = MsgType::REQUEST;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MsgType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> MsgType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &MsgType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut MsgType {
        &mut self.field_type
    }

    // uint32 origin = 3;

    pub fn clear_origin(&mut self) {
        self.origin = 0;
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: u32) {
        self.origin = v;
    }

    pub fn get_origin(&self) -> u32 {
        self.origin
    }

    fn get_origin_for_reflect(&self) -> &u32 {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut u32 {
        &mut self.origin
    }

    // .OperateType operate = 4;

    pub fn clear_operate(&mut self) {
        self.operate = OperateType::BROADCAST;
    }

    // Param is passed by value, moved
    pub fn set_operate(&mut self, v: OperateType) {
        self.operate = v;
    }

    pub fn get_operate(&self) -> OperateType {
        self.operate
    }

    fn get_operate_for_reflect(&self) -> &OperateType {
        &self.operate
    }

    fn mut_operate_for_reflect(&mut self) -> &mut OperateType {
        &mut self.operate
    }

    // bytes content = 5;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.content, ::std::vec::Vec::new())
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.cmd_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.origin = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.operate = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.content)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.cmd_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.cmd_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.field_type != MsgType::REQUEST {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        };
        if self.origin != 0 {
            my_size += ::protobuf::rt::value_size(3, self.origin, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.operate != OperateType::BROADCAST {
            my_size += ::protobuf::rt::enum_size(4, self.operate);
        };
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.content);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cmd_id != 0 {
            os.write_uint32(1, self.cmd_id)?;
        };
        if self.field_type != MsgType::REQUEST {
            os.write_enum(2, self.field_type.value())?;
        };
        if self.origin != 0 {
            os.write_uint32(3, self.origin)?;
        };
        if self.operate != OperateType::BROADCAST {
            os.write_enum(4, self.operate.value())?;
        };
        if !self.content.is_empty() {
            os.write_bytes(5, &self.content)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cmd_id",
                    Message::get_cmd_id_for_reflect,
                    Message::mut_cmd_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MsgType>>(
                    "type",
                    Message::get_field_type_for_reflect,
                    Message::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "origin",
                    Message::get_origin_for_reflect,
                    Message::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OperateType>>(
                    "operate",
                    Message::get_operate_for_reflect,
                    Message::mut_operate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content",
                    Message::get_content_for_reflect,
                    Message::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_cmd_id();
        self.clear_field_type();
        self.clear_origin();
        self.clear_operate();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MsgType {
    REQUEST = 0,
    HEADER = 1,
    BODY = 2,
    BLOCK = 3,
    TX = 4,
    STATUS = 5,
    MSG = 6,
    RESPONSE = 7,
    TX_RESPONSE = 8,
}

impl ::protobuf::ProtobufEnum for MsgType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MsgType> {
        match value {
            0 => ::std::option::Option::Some(MsgType::REQUEST),
            1 => ::std::option::Option::Some(MsgType::HEADER),
            2 => ::std::option::Option::Some(MsgType::BODY),
            3 => ::std::option::Option::Some(MsgType::BLOCK),
            4 => ::std::option::Option::Some(MsgType::TX),
            5 => ::std::option::Option::Some(MsgType::STATUS),
            6 => ::std::option::Option::Some(MsgType::MSG),
            7 => ::std::option::Option::Some(MsgType::RESPONSE),
            8 => ::std::option::Option::Some(MsgType::TX_RESPONSE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MsgType] = &[
            MsgType::REQUEST,
            MsgType::HEADER,
            MsgType::BODY,
            MsgType::BLOCK,
            MsgType::TX,
            MsgType::STATUS,
            MsgType::MSG,
            MsgType::RESPONSE,
            MsgType::TX_RESPONSE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MsgType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MsgType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MsgType {
}

impl ::std::default::Default for MsgType {
    fn default() -> Self {
        MsgType::REQUEST
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OperateType {
    BROADCAST = 0,
    SINGLE = 1,
    SUBTRACT = 2,
}

impl ::protobuf::ProtobufEnum for OperateType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OperateType> {
        match value {
            0 => ::std::option::Option::Some(OperateType::BROADCAST),
            1 => ::std::option::Option::Some(OperateType::SINGLE),
            2 => ::std::option::Option::Some(OperateType::SUBTRACT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OperateType] = &[
            OperateType::BROADCAST,
            OperateType::SINGLE,
            OperateType::SUBTRACT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<OperateType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OperateType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OperateType {
}

impl ::std::default::Default for OperateType {
    fn default() -> Self {
        OperateType::BROADCAST
    }
}

impl ::protobuf::reflect::ProtobufValue for OperateType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x63, 0x6f, 0x6d, 0x6d, 0x75, 0x6e, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x98, 0x01, 0x0a, 0x07, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x15, 0x0a, 0x06, 0x63, 0x6d, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x05, 0x63, 0x6d, 0x64, 0x49, 0x64, 0x12, 0x1c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x08, 0x2e, 0x4d, 0x73, 0x67, 0x54, 0x79, 0x70, 0x65,
    0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x26,
    0x0a, 0x07, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x0c, 0x2e, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x07, 0x6f,
    0x70, 0x65, 0x72, 0x61, 0x74, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x2a, 0x73, 0x0a, 0x07, 0x4d, 0x73, 0x67, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x52,
    0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x48, 0x45, 0x41, 0x44,
    0x45, 0x52, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x42, 0x4f, 0x44, 0x59, 0x10, 0x02, 0x12, 0x09,
    0x0a, 0x05, 0x42, 0x4c, 0x4f, 0x43, 0x4b, 0x10, 0x03, 0x12, 0x06, 0x0a, 0x02, 0x54, 0x58, 0x10,
    0x04, 0x12, 0x0a, 0x0a, 0x06, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x10, 0x05, 0x12, 0x07, 0x0a,
    0x03, 0x4d, 0x53, 0x47, 0x10, 0x06, 0x12, 0x0c, 0x0a, 0x08, 0x52, 0x45, 0x53, 0x50, 0x4f, 0x4e,
    0x53, 0x45, 0x10, 0x07, 0x12, 0x0f, 0x0a, 0x0b, 0x54, 0x58, 0x5f, 0x52, 0x45, 0x53, 0x50, 0x4f,
    0x4e, 0x53, 0x45, 0x10, 0x08, 0x2a, 0x36, 0x0a, 0x0b, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x65,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x09, 0x42, 0x52, 0x4f, 0x41, 0x44, 0x43, 0x41, 0x53,
    0x54, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x53, 0x49, 0x4e, 0x47, 0x4c, 0x45, 0x10, 0x01, 0x12,
    0x0c, 0x0a, 0x08, 0x53, 0x55, 0x42, 0x54, 0x52, 0x41, 0x43, 0x54, 0x10, 0x02, 0x4a, 0xa4, 0x07,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x02, 0x00, 0x0c, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x02, 0x05, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x03, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x03, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x04, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x04, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x05, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x05, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06,
    0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x04, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x0c, 0x0d, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x07, 0x04, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x04, 0x02, 0x12, 0x03, 0x07, 0x09, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12,
    0x03, 0x08, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x08,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x08, 0x0d, 0x0e,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x09, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x09, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x07, 0x12, 0x03, 0x0a, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x0a,
    0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x0b, 0x04, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0b, 0x04, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x0b, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x05,
    0x01, 0x12, 0x04, 0x0e, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03,
    0x0e, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x04, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x10, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x10, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x10, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x11,
    0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x11, 0x0f, 0x10, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x14, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x14, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x15, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x15, 0x04,
    0x14, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x15, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x16, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x16, 0x04, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x16, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x16, 0x0c, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x16,
    0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x17, 0x04, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x17, 0x04, 0x16, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x17, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x18, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x18, 0x04, 0x17, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x18,
    0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x18, 0x10, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x18, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x19, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x19, 0x04, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x19, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x19, 0x0a, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x19, 0x14, 0x15, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
