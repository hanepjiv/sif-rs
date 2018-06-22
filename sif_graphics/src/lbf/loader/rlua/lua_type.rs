// -*- mode:rust; coding:utf-8-unix; -*-

//! lua_type.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/06/13
//  @date 2018/06/22

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{collections::BTreeMap, fmt::Debug, hash::Hash, path::PathBuf};
// ----------------------------------------------------------------------------
use gl::types::*;
use rlua::{Integer, Value};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::{Number, Quaternion, Vector3, Vector4};
use sif_three::{Armature, Bone, TraRotSca};
// ----------------------------------------------------------------------------
use super::{
    super::super::{
        super::{Camera, Image, LightFlags, Material, Model, Texture},
        texture_filter_match, texture_wrap_match, LBFLight, LBFMesh,
        LBFObject, LBFPolygon, LBFPolygonFlags,
    },
    Error, Result,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait NotNil
trait NotNil: ::std::fmt::Debug + ::std::marker::Sized {
    /// notnil_or_else
    fn notnil_or_else(self, f: impl FnOnce() -> Self) -> Self;
}
// ----------------------------------------------------------------------------
impl<'lua> NotNil for Value<'lua> {
    fn notnil_or_else(self, f: impl FnOnce() -> Self) -> Self {
        match self {
            Value::Nil => f(),
            _ => self,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait LuaType
pub(crate) trait LuaType: ::std::marker::Sized {
    fn from_lua(v: Value) -> Result<Self>;
}
// ============================================================================
impl LuaType for Integer {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Integer(ret) => Ok(ret),
            _ => {
                Err(Error::Type("<Integer as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ============================================================================
impl LuaType for bool {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Boolean(ret) => Ok(ret),
            _ => Err(Error::Type("<bool as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for u8 {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Integer(ret) => Ok(ret as Self),
            _ => Err(Error::Type("<u8 as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for u32 {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Integer(ret) => Ok(ret as Self),
            _ => Err(Error::Type("<u32 as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for usize {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Integer(ret) => Ok(ret as Self),
            _ => Err(Error::Type("<usize as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for isize {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Integer(ret) => Ok(ret as Self),
            _ => Err(Error::Type("<isize as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Vec<T>
where
    T: 'static + LuaType,
{
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let mut ret = Self::default();
                for pairs in tbl.pairs::<Integer, Value>() {
                    let (_k, t) = pairs?;
                    ret.push(T::from_lua(t)?);
                }
                Ok(ret)
            }
            _ => Err(Error::Type("<Vec<T> as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl<K, T> LuaType for BTreeMap<K, T>
where
    K: 'static + Hash + Ord + Clone + LuaType,
    T: 'static + Debug + AsRef<K> + LuaType,
{
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let mut ret = Self::default();
                for pairs in tbl.pairs::<Value, Value>() {
                    let (k, t) = pairs?;
                    if let Some(ref x) =
                        ret.insert(K::from_lua(k)?, T::from_lua(t)?)
                    {
                        return Err(Error::Insert(format!("{:?}", x)));
                    }
                }
                Ok(ret)
            }
            _ => Err(Error::Type(
                "<BTreeMap<K, T> as LuaType>::from_lua}".to_string(),
            )),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for String {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::String(ret) => Ok(ret.to_str()?.to_string()),
            _ => Err(Error::Type("<String as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Option<T>
where
    T: 'static + LuaType,
{
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Boolean(true) => Err(Error::Type(
                "<Option<T> as LuaType>::from_lua".to_string(),
            )),
            Value::Boolean(false) => Ok(None),
            _ => Ok(Some(T::from_lua(v)?)),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for PathBuf {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::String(ret) => Ok(ret.to_str()?.into()),
            _ => {
                Err(Error::Type("<PathBuf as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ============================================================================
impl LuaType for Uuid {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::String(ret) => Ok(Uuid::parse_str(ret.to_str()?)?),
            _ => Err(Error::Type("<Uuid as LuaType>::from_lua".to_string())),
        }
    }
}
// ============================================================================
impl LuaType for GLfloat {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Integer(ret) => Ok(ret as Self),
            Value::Number(ret) => Ok(ret as Self),
            _ => {
                Err(Error::Type("<GLfloat as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ============================================================================
impl<T> LuaType for Vector3<T>
where
    Vector3<T>: 'static,
    T: 'static + Number + LuaType,
{
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                T::from_lua(tbl.get(1)?)?,
                T::from_lua(tbl.get(2)?)?,
                T::from_lua(tbl.get(3)?)?,
            )),
            _ => Err(Error::Type(
                "<Vector3<T> as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Vector4<T>
where
    Vector4<T>: 'static,
    T: 'static + Number + LuaType,
{
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                T::from_lua(tbl.get(1)?)?,
                T::from_lua(tbl.get(2)?)?,
                T::from_lua(tbl.get(3)?)?,
                T::from_lua(tbl.get(4)?)?,
            )),
            _ => Err(Error::Type(
                "<Vector4<T> as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Quaternion<T>
where
    Quaternion<T>: 'static,
    T: 'static + Number + LuaType,
{
    fn from_lua(v: Value) -> Result<Self> {
        Vector4::from_lua(v).map(Self::from)
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for TraRotSca<T>
where
    T: 'static + Number + LuaType,
{
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                Vector3::from_lua(tbl.get("translation")?)?,
                Quaternion::from_lua(tbl.get("rotation")?)?,
                Vector3::from_lua(tbl.get("scaling")?)?,
            )),
            _ => Err(Error::Type(
                "<TraRotSca<T> as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
// ============================================================================
impl LuaType for Image {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                match String::from_lua(tbl.get("source")?)?.as_str() {
                    "FILE" => Ok(Self::new_file(
                        Uuid::from_lua(tbl.get("uuid")?)?,
                        String::from_lua(tbl.get("name")?)?,
                        u8::from_lua(tbl.get("dimension")?)?,
                        PathBuf::from_lua(tbl.get("path")?)?,
                    )),
                    _ => Err(Error::Type(
                        "<Image as LuaType>::from_lua".to_string(),
                    )),
                }
            }
            _ => Err(Error::Type("<Image as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Texture {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                Uuid::from_lua(tbl.get("uuid")?)?,
                String::from_lua(tbl.get("name")?)?,
                texture_wrap_match(&String::from_lua(tbl.get("wrap_s")?)?)?,
                texture_wrap_match(&String::from_lua(tbl.get("wrap_t")?)?)?,
                texture_filter_match(&String::from_lua(
                    tbl.get("filter_mag")?,
                )?)?,
                texture_filter_match(&String::from_lua(
                    tbl.get("filter_min")?,
                )?)?,
                bool::from_lua(tbl.get("mipmap")?)?,
                Uuid::from_lua(tbl.get("image")?)?,
            )),
            _ => {
                Err(Error::Type("<Texture as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Material {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let mut ret = Self::new(
                    Uuid::from_lua(tbl.get("uuid")?)?,
                    String::from_lua(tbl.get("name")?)?,
                );
                ret.diffuse.color =
                    Vector3::from_lua(tbl.get("diffuse.color")?)?;
                ret.diffuse.intensity =
                    GLfloat::from_lua(tbl.get("diffuse.intensity")?)?;
                ret.specular.color =
                    Vector3::from_lua(tbl.get("specular.color")?)?;
                ret.specular.intensity =
                    GLfloat::from_lua(tbl.get("specular.intensity")?)?;
                ret.emissive.color =
                    Vector3::from_lua(tbl.get("emissive.color")?)?;
                ret.emissive.intensity =
                    GLfloat::from_lua(tbl.get("emissive.intensity")?)?;
                ret.shininess = GLfloat::from_lua(tbl.get("shininess")?)?;
                ret.alpha = GLfloat::from_lua(tbl.get("alpha")?)?;
                ret.parallax.height = GLfloat::from_lua(
                    tbl.get::<_, Value>("parallax.height")?
                        .notnil_or_else(|| Value::Number(0.025)),
                )?;
                if let Ok(textures) =
                    Vec::<Option<Uuid>>::from_lua(tbl.get("textures")?)
                {
                    ret.textures = Some(Err(textures));
                }
                Ok(ret)
            }
            _ => {
                Err(Error::Type("<Material as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LBFMesh {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => Self::new(
                Uuid::from_lua(tbl.get("uuid")?)?,
                String::from_lua(tbl.get("name")?)?,
                Vec::<GLfloat>::from_lua(tbl.get("position")?)?,
                Vec::<GLfloat>::from_lua(tbl.get("normal")?)?,
                match tbl.get("coord")? {
                    Value::Nil => Vec::<GLfloat>::default(),
                    v @ Value::Table(_) => Vec::<GLfloat>::from_lua(v)?,
                    _ => {
                        return Err(Error::Type(
                            "<LBFMesh as LuaType>::from_lua".to_string(),
                        ));
                    }
                },
                match tbl.get("bone")? {
                    Value::Nil => Vec::<GLfloat>::default(),
                    v @ Value::Table(_) => Vec::<GLfloat>::from_lua(v)?,
                    _ => {
                        return Err(Error::Type(
                            "<LBFMesh as LuaType>::from_lua".to_string(),
                        ));
                    }
                },
                match tbl.get("weight")? {
                    Value::Nil => Vec::<GLfloat>::default(),
                    v @ Value::Table(_) => Vec::<GLfloat>::from_lua(v)?,
                    _ => {
                        return Err(Error::Type(
                            "<LBFMesh as LuaType>::from_lua".to_string(),
                        ));
                    }
                },
                Vec::<LBFPolygon>::from_lua(tbl.get("polygons")?)?,
            ),
            _ => {
                Err(Error::Type("<LBFMesh as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LBFPolygon {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let bits = u32::from_lua(tbl.get(1)?)?;
                let midx = isize::from_lua(tbl.get(2)?)?;
                LBFPolygon::new(
                    LBFPolygonFlags::from_bits(bits).ok_or_else(|| {
                        Error::Type(format!(
                            "LBFPolygon: flags from_bits({})",
                            bits
                        ))
                    })?,
                    if -1 < midx { Some(midx as usize) } else { None },
                    &(Vec::<u32>::from_lua(tbl.get(3)?)?)[..],
                )
            }
            _ => Err(Error::Type(
                "<LBFPolygon as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
// ----------------------------------------------------------------------------
impl<V> LuaType for Bone<V>
where
    V: 'static + Number + LuaType,
{
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let parent = isize::from_lua(tbl.get(2)?)?;
                Ok(Self::new(
                    Vector3::<V>::from_lua(tbl.get(1)?)?,
                    if parent < 0 {
                        None
                    } else {
                        Some(parent as usize)
                    },
                ))
            }
            _ => {
                Err(Error::Type("<Bone<T> as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ----------------------------------------------------------------------------
impl<V> LuaType for Armature<V>
where
    V: 'static + Number + LuaType,
{
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                Uuid::from_lua(tbl.get("uuid")?)?,
                String::from_lua(tbl.get("name")?)?,
                Vec::<Bone<V>>::from_lua(tbl.get("bones")?)?,
            )),
            _ => Err(Error::Type(
                "<Armature<T> as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Model {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let mut ret = Self::new(
                    Uuid::from_lua(tbl.get("uuid")?)?,
                    String::from_lua(tbl.get("name")?)?,
                );
                ret.meshes =
                    Some(Err(Vec::<Uuid>::from_lua(tbl.get("meshes")?)?));
                ret.materials =
                    Some(Err(Vec::<Uuid>::from_lua(tbl.get("materials")?)?));
                if let Ok(armature) = Uuid::from_lua(tbl.get("armature")?) {
                    ret.armature = Some(Err(armature));
                }
                Ok(ret)
            }
            _ => Err(Error::Type("<Model as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LBFLight {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let mut ret = Self::new(
                    Uuid::from_lua(tbl.get("uuid")?)?,
                    String::from_lua(tbl.get("name")?)?,
                );
                ret.color = Vector3::<GLfloat>::from_lua(tbl.get("color")?)?;
                ret.intensity = GLfloat::from_lua(tbl.get("intensity")?)?;
                let light_type = String::from_lua(tbl.get("light_type")?)?;
                if "SUN" != light_type.as_str() {
                    ret.flags.insert(LightFlags::POINT);
                    ret.kcklkq =
                        Vector3::<GLfloat>::from_lua(tbl.get("kcklkq")?)?;
                    if "SPOT" == light_type.as_str() {
                        ret.flags.insert(LightFlags::SPOT);
                        ret.exponent =
                            GLfloat::from_lua(tbl.get("exponent")?)?;
                        ret.cutoff = GLfloat::from_lua(tbl.get("cutoff")?)?;
                    }
                }
                if bool::from_lua(
                    tbl.get::<_, Value>("shadow")?
                        .notnil_or_else(|| Value::Boolean(false)),
                )? {
                    ret.flags.insert(LightFlags::SHADOW);
                }
                Ok(ret)
            }
            _ => {
                Err(Error::Type("<LBFLight as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Camera {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => match String::from_lua(
                tbl.get("camera_type")?,
            )?.as_str()
            {
                "FRUSTUM" => Ok(Camera::new_frustum(
                    Uuid::from_lua(tbl.get("uuid")?)?,
                    String::from_lua(tbl.get("name")?)?,
                    GLfloat::from_lua(tbl.get("near")?)?,
                    GLfloat::from_lua(tbl.get("far")?)?,
                    Camera::alpha2focus(GLfloat::from_lua(tbl.get("alpha")?)?),
                    GLfloat::from_lua(tbl.get("aspect")?)?,
                )),
                "ORTHO" => Ok(Camera::new_ortho(
                    Uuid::from_lua(tbl.get("uuid")?)?,
                    String::from_lua(tbl.get("name")?)?,
                    GLfloat::from_lua(tbl.get("near")?)?,
                    GLfloat::from_lua(tbl.get("far")?)?,
                    GLfloat::from_lua(tbl.get("width")?)?,
                    GLfloat::from_lua(tbl.get("height")?)?,
                )),
                _ => Err(Error::Type(
                    "<Camera as LuaType>::from_lua".to_string(),
                )),
            },
            _ => Err(Error::Type("<Camera as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LBFObject {
    fn from_lua(v: Value) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                Uuid::from_lua(tbl.get("uuid")?)?,
                String::from_lua(tbl.get("name")?)?,
                Option::<Uuid>::from_lua(
                    tbl.get::<_, Value>("parent")?
                        .notnil_or_else(|| Value::Boolean(false)),
                )?,
                String::from_lua(tbl.get("data_type")?)?,
                Uuid::from_lua(tbl.get("data_uuid")?)?,
                TraRotSca::from_lua(tbl.get("trarotsca")?)?,
            )),
            _ => Err(Error::Type(
                "<LBFObject as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
