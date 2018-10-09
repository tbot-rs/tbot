use super::*;
use serde::ser::{self, Serialize};

mod error;
use self::error::{Error, Result};

pub struct Serializer {
    output: Vec<u8>,
}

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer {
        output: Vec::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        let mut value = if v { b"true".to_vec() } else { b"false".to_vec() };
        self.output.append(&mut value);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.output.push(v as u8);
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output.append(&mut v.as_bytes().to_vec());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output.append(&mut v.to_vec());
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.output.append(&mut b"null".to_vec());
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()> {
        self.output.append(&mut b"--boundary".to_vec());
        self.output.append(
            &mut b"Content-Disposition: form-data; name=".to_vec(),
        );
        variant.serialize(&mut *self)?;
        self.output.append(&mut b"\r\n".to_vec());
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output.append(&mut b"[".to_vec());
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output.append(&mut b"--boundary".to_vec());
        self.output.append(
            &mut b"Content-Disposition: form-data; name=".to_vec(),
        );
        variant.serialize(&mut *self)?;
        self.output.append(&mut b"\r\n".to_vec());
        Ok(self)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> {
        self.output.push(b'{');
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output.append(&mut b"--boundary".to_vec());
        self.output.append(
            &mut b"Content-Disposition: form-data; name=".to_vec(),
        );
        variant.serialize(&mut *self)?;
        self.output.append(&mut b"\r\n".to_vec());
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize,
    {
        if !self.output.ends_with(b"[") {
            self.output.push(b',');
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(b']');
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize,
    {
        if !self.output.ends_with(b"[") {
            self.output.push(b',');
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(b']');
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize,
    {
        if !self.output.ends_with(b"[") {
            self.output.push(b',');
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(b']');
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize,
    {
        if !self.output.ends_with(b"[") {
            self.output.push(b',');
        }
        self.output.append(
            &mut serde_json::to_string(&value).unwrap().as_bytes().to_vec(),
        );
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output.push(b']');
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
        where T: ?Sized + Serialize,
    {
        if !self.output.ends_with(b"{") {
            self.output.push(b',');
        }
        self.output.append(
            &mut serde_json::to_string(&key).unwrap().as_bytes().to_vec(),
        );
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize,
    {
        self.output.push(b':');
        self.output.append(
            &mut serde_json::to_string(&value).unwrap().as_bytes().to_vec(),
        );
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output.push(b'}');
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize,
    {
        if !self.output.ends_with(b"{") {
            self.output.push(b',');
        }
        self.output.append(
            &mut serde_json::to_string(&key).unwrap().as_bytes().to_vec(),
        );
        self.output.push(b':');
        self.output.append(
            &mut serde_json::to_string(&value).unwrap().as_bytes().to_vec(),
        );
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output.push(b'}');
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize,
    {
        if !self.output.ends_with(b"{") {
            self.output.push(b',');
        }
        self.output.append(
            &mut serde_json::to_string(&key).unwrap().as_bytes().to_vec(),
        );
        self.output.push(b':');
        self.output.append(
            &mut serde_json::to_string(&value).unwrap().as_bytes().to_vec(),
        );
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output.append(&mut b"}}".to_vec());
        Ok(())
    }
}
