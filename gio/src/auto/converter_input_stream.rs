// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Converter;
use crate::FilterInputStream;
use crate::InputStream;
use crate::PollableInputStream;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct ConverterInputStream(Object<ffi::GConverterInputStream, ffi::GConverterInputStreamClass>) @extends FilterInputStream, InputStream, @implements PollableInputStream;

    match fn {
        type_ => || ffi::g_converter_input_stream_get_type(),
    }
}

impl ConverterInputStream {
    #[doc(alias = "g_converter_input_stream_new")]
    pub fn new<P: IsA<InputStream>, Q: IsA<Converter>>(
        base_stream: &P,
        converter: &Q,
    ) -> ConverterInputStream {
        unsafe {
            InputStream::from_glib_full(ffi::g_converter_input_stream_new(
                base_stream.as_ref().to_glib_none().0,
                converter.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`ConverterInputStream`]
    /// This method returns an instance of [`ConverterInputStreamBuilder`] which can be used to create a [`ConverterInputStream`].
    pub fn builder() -> ConverterInputStreamBuilder {
        ConverterInputStreamBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`ConverterInputStream`].
pub struct ConverterInputStreamBuilder {
    converter: Option<Converter>,
    base_stream: Option<InputStream>,
    close_base_stream: Option<bool>,
}

impl ConverterInputStreamBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ConverterInputStreamBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ConverterInputStream`].
    pub fn build(self) -> ConverterInputStream {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref converter) = self.converter {
            properties.push(("converter", converter));
        }
        if let Some(ref base_stream) = self.base_stream {
            properties.push(("base-stream", base_stream));
        }
        if let Some(ref close_base_stream) = self.close_base_stream {
            properties.push(("close-base-stream", close_base_stream));
        }
        glib::Object::new::<ConverterInputStream>(&properties)
            .expect("Failed to create an instance of ConverterInputStream")
    }

    pub fn converter<P: IsA<Converter>>(mut self, converter: &P) -> Self {
        self.converter = Some(converter.clone().upcast());
        self
    }

    pub fn base_stream<P: IsA<InputStream>>(mut self, base_stream: &P) -> Self {
        self.base_stream = Some(base_stream.clone().upcast());
        self
    }

    pub fn close_base_stream(mut self, close_base_stream: bool) -> Self {
        self.close_base_stream = Some(close_base_stream);
        self
    }
}

pub const NONE_CONVERTER_INPUT_STREAM: Option<&ConverterInputStream> = None;

pub trait ConverterInputStreamExt: 'static {
    #[doc(alias = "g_converter_input_stream_get_converter")]
    #[doc(alias = "get_converter")]
    fn converter(&self) -> Converter;
}

impl<O: IsA<ConverterInputStream>> ConverterInputStreamExt for O {
    fn converter(&self) -> Converter {
        unsafe {
            from_glib_none(ffi::g_converter_input_stream_get_converter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ConverterInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConverterInputStream")
    }
}
