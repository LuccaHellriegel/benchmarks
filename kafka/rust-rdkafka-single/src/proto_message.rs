// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `proto_message.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:main.ProtoMessage)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ProtoMessage {
    // message fields
    // @@protoc_insertion_point(field:main.ProtoMessage.message_type)
    pub message_type: ::std::string::String,
    // @@protoc_insertion_point(field:main.ProtoMessage.payload)
    pub payload: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:main.ProtoMessage.some_id)
    pub some_id: ::std::string::String,
    // @@protoc_insertion_point(field:main.ProtoMessage.headers)
    pub headers: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:main.ProtoMessage.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ProtoMessage {
    fn default() -> &'a ProtoMessage {
        <ProtoMessage as ::protobuf::Message>::default_instance()
    }
}

impl ProtoMessage {
    pub fn new() -> ProtoMessage {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "message_type",
            |m: &ProtoMessage| { &m.message_type },
            |m: &mut ProtoMessage| { &mut m.message_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "payload",
            |m: &ProtoMessage| { &m.payload },
            |m: &mut ProtoMessage| { &mut m.payload },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "some_id",
            |m: &ProtoMessage| { &m.some_id },
            |m: &mut ProtoMessage| { &mut m.some_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "headers",
            |m: &ProtoMessage| { &m.headers },
            |m: &mut ProtoMessage| { &mut m.headers },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ProtoMessage>(
            "ProtoMessage",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ProtoMessage {
    const NAME: &'static str = "ProtoMessage";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.message_type = is.read_string()?;
                },
                18 => {
                    self.payload = is.read_bytes()?;
                },
                26 => {
                    self.some_id = is.read_string()?;
                },
                34 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            18 => value = is.read_string()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.headers.insert(key, value);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.message_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message_type);
        }
        if !self.payload.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.payload);
        }
        if !self.some_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.some_id);
        }
        for (k, v) in &self.headers {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.message_type.is_empty() {
            os.write_string(1, &self.message_type)?;
        }
        if !self.payload.is_empty() {
            os.write_bytes(2, &self.payload)?;
        }
        if !self.some_id.is_empty() {
            os.write_string(3, &self.some_id)?;
        }
        for (k, v) in &self.headers {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            os.write_raw_varint32(34)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ProtoMessage {
        ProtoMessage::new()
    }

    fn clear(&mut self) {
        self.message_type.clear();
        self.payload.clear();
        self.some_id.clear();
        self.headers.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ProtoMessage {
        static instance: ::protobuf::rt::Lazy<ProtoMessage> = ::protobuf::rt::Lazy::new();
        instance.get(ProtoMessage::new)
    }
}

impl ::protobuf::MessageFull for ProtoMessage {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ProtoMessage").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ProtoMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProtoMessage {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13proto_message.proto\x12\x04main\"\xdb\x01\n\x0cProtoMessage\x12!\n\
    \x0cmessage_type\x18\x01\x20\x01(\tR\x0bmessageType\x12\x18\n\x07payload\
    \x18\x02\x20\x01(\x0cR\x07payload\x12\x17\n\x07some_id\x18\x03\x20\x01(\
    \tR\x06someId\x129\n\x07headers\x18\x04\x20\x03(\x0b2\x1f.main.ProtoMess\
    age.HeadersEntryR\x07headers\x1a:\n\x0cHeadersEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\
    \x028\x01B\x10Z\x0ego-single/mainJ\xa5\x02\n\x06\x12\x04\0\0\n\x01\n\x08\
    \n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0\r\n\x08\n\x01\
    \x08\x12\x03\x03\0%\n\t\n\x02\x08\x0b\x12\x03\x03\0%\n\n\n\x02\x04\0\x12\
    \x04\x05\0\n\x01\n\n\n\x03\x04\0\x01\x12\x03\x05\x08\x14\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03\x06\x02\x1a\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x06\x02\
    \x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\t\x15\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03\x06\x18\x19\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x07\x02\x14\
    \n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x07\x02\x07\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x07\x08\x0f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x07\
    \x12\x13\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x08\x02\x15\n\x0c\n\x05\x04\0\
    \x02\x02\x05\x12\x03\x08\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\
    \x08\t\x10\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x08\x13\x14\n\x0b\n\x04\
    \x04\0\x02\x03\x12\x03\t\x02!\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\t\
    \x02\x14\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\t\x15\x1c\n\x0c\n\x05\x04\
    \0\x02\x03\x03\x12\x03\t\x1f\x20b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ProtoMessage::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
