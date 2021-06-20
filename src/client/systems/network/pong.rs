// This file is generated by rust-protobuf 2.18.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `pong.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_2;

#[derive(PartialEq,Clone,Default)]
pub struct SayRequest {
    // message fields
    pub name: ::std::string::String,
    pub networkname: f32,
    pub networkname2: f32,
    pub networkname3: f32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SayRequest {
    fn default() -> &'a SayRequest {
        <SayRequest as ::protobuf::Message>::default_instance()
    }
}

impl SayRequest {
    pub fn new() -> SayRequest {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // float networkname = 2;


    pub fn get_networkname(&self) -> f32 {
        self.networkname
    }
    pub fn clear_networkname(&mut self) {
        self.networkname = 0.;
    }

    // Param is passed by value, moved
    pub fn set_networkname(&mut self, v: f32) {
        self.networkname = v;
    }

    // float networkname2 = 3;


    pub fn get_networkname2(&self) -> f32 {
        self.networkname2
    }
    pub fn clear_networkname2(&mut self) {
        self.networkname2 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_networkname2(&mut self, v: f32) {
        self.networkname2 = v;
    }

    // float networkname3 = 4;


    pub fn get_networkname3(&self) -> f32 {
        self.networkname3
    }
    pub fn clear_networkname3(&mut self) {
        self.networkname3 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_networkname3(&mut self, v: f32) {
        self.networkname3 = v;
    }
}

impl ::protobuf::Message for SayRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.networkname = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.networkname2 = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.networkname3 = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.networkname != 0. {
            my_size += 5;
        }
        if self.networkname2 != 0. {
            my_size += 5;
        }
        if self.networkname3 != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.networkname != 0. {
            os.write_float(2, self.networkname)?;
        }
        if self.networkname2 != 0. {
            os.write_float(3, self.networkname2)?;
        }
        if self.networkname3 != 0. {
            os.write_float(4, self.networkname3)?;
        }
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SayRequest {
        SayRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &SayRequest| { &m.name },
                |m: &mut SayRequest| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "networkname",
                |m: &SayRequest| { &m.networkname },
                |m: &mut SayRequest| { &mut m.networkname },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "networkname2",
                |m: &SayRequest| { &m.networkname2 },
                |m: &mut SayRequest| { &mut m.networkname2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "networkname3",
                |m: &SayRequest| { &m.networkname3 },
                |m: &mut SayRequest| { &mut m.networkname3 },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SayRequest>(
                "SayRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SayRequest {
        static instance: ::protobuf::rt::LazyV2<SayRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SayRequest::new)
    }
}

impl ::protobuf::Clear for SayRequest {
    fn clear(&mut self) {
        self.name.clear();
        self.networkname = 0.;
        self.networkname2 = 0.;
        self.networkname3 = 0.;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SayRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SayRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SayResponse {
    // message fields
    pub message: f32,
    pub message2: f32,
    pub message3: f32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SayResponse {
    fn default() -> &'a SayResponse {
        <SayResponse as ::protobuf::Message>::default_instance()
    }
}

impl SayResponse {
    pub fn new() -> SayResponse {
        ::std::default::Default::default()
    }

    // float message = 1;


    pub fn get_message(&self) -> f32 {
        self.message
    }
    pub fn clear_message(&mut self) {
        self.message = 0.;
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: f32) {
        self.message = v;
    }

    // float message2 = 2;


    pub fn get_message2(&self) -> f32 {
        self.message2
    }
    pub fn clear_message2(&mut self) {
        self.message2 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_message2(&mut self, v: f32) {
        self.message2 = v;
    }

    // float message3 = 3;


    pub fn get_message3(&self) -> f32 {
        self.message3
    }
    pub fn clear_message3(&mut self) {
        self.message3 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_message3(&mut self, v: f32) {
        self.message3 = v;
    }
}

impl ::protobuf::Message for SayResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.message = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.message2 = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.message3 = tmp;
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
        if self.message != 0. {
            my_size += 5;
        }
        if self.message2 != 0. {
            my_size += 5;
        }
        if self.message3 != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.message != 0. {
            os.write_float(1, self.message)?;
        }
        if self.message2 != 0. {
            os.write_float(2, self.message2)?;
        }
        if self.message3 != 0. {
            os.write_float(3, self.message3)?;
        }
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SayResponse {
        SayResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "message",
                |m: &SayResponse| { &m.message },
                |m: &mut SayResponse| { &mut m.message },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "message2",
                |m: &SayResponse| { &m.message2 },
                |m: &mut SayResponse| { &mut m.message2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "message3",
                |m: &SayResponse| { &m.message3 },
                |m: &mut SayResponse| { &mut m.message3 },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SayResponse>(
                "SayResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SayResponse {
        static instance: ::protobuf::rt::LazyV2<SayResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SayResponse::new)
    }
}

impl ::protobuf::Clear for SayResponse {
    fn clear(&mut self) {
        self.message = 0.;
        self.message2 = 0.;
        self.message3 = 0.;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SayResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SayResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\npong.proto\x12\x04pong\"\x8a\x01\n\nSayRequest\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12\x20\n\x0bnetworkname\x18\x02\x20\x01(\x02R\
    \x0bnetworkname\x12\"\n\x0cnetworkname2\x18\x03\x20\x01(\x02R\x0cnetwork\
    name2\x12\"\n\x0cnetworkname3\x18\x04\x20\x01(\x02R\x0cnetworkname3\"_\n\
    \x0bSayResponse\x12\x18\n\x07message\x18\x01\x20\x01(\x02R\x07message\
    \x12\x1a\n\x08message2\x18\x02\x20\x01(\x02R\x08message2\x12\x1a\n\x08me\
    ssage3\x18\x03\x20\x01(\x02R\x08message322\n\x03Say\x12+\n\x04Send\x12\
    \x10.pong.SayRequest\x1a\x11.pong.SayResponseb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}