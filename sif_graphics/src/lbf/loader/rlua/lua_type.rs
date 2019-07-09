// -*- mode:rust; coding:utf-8-unix; -*-

//! lua_type.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/06/13
//  @date 2019/07/09

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{collections::BTreeMap, fmt::Debug, hash::Hash, path::PathBuf};
// ----------------------------------------------------------------------------
use gl::types::*;
use rlua::Value;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::{Float, Integer, Quaternion, Vector3, Vector4};
use sif_three::{Armature, Bone, TraRotSca};
// ----------------------------------------------------------------------------
use super::{
    super::super::{
        super::{ColorIntensity, LightFlags, MaterialFlags, Parallax},
        texture_filter_match, texture_wrap_match, Animation, Camera, Curve,
        CurveType, Image, Interpolation, Keyframe, LBFAnimationDriver,
        LBFLight, LBFMaterial, LBFMesh, LBFModel, LBFObject, LBFPolygon,
        LBFPolygonFlags, LBFTexture,
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
    fn from_lua(v: Value<'_>) -> Result<Self>;
}
// ============================================================================
impl LuaType for ::rlua::Integer {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Integer(x) => Ok(x),
            _ => {
                Err(Error::Type("<Integer as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ============================================================================
impl LuaType for GLint {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Integer(x) => Ok(x as Self),
            Value::Number(x) => Ok(x as Self),
            _ => Err(Error::Type("<GLint as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for GLfloat {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Integer(x) => Ok(x as Self),
            Value::Number(x) => Ok(x as Self),
            _ => {
                Err(Error::Type("<GLfloat as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ============================================================================
impl LuaType for bool {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Boolean(x) => Ok(x),
            _ => Err(Error::Type("<bool as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for u8 {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Integer(x) => Ok(x as Self),
            _ => Err(Error::Type("<u8 as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for u32 {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Integer(x) => Ok(x as Self),
            _ => Err(Error::Type("<u32 as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for u64 {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Integer(x) => Ok(x as Self),
            _ => Err(Error::Type("<u32 as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for usize {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Integer(x) => Ok(x as Self),
            _ => Err(Error::Type("<usize as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for isize {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Integer(x) => Ok(x as Self),
            _ => Err(Error::Type("<isize as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Vec<T>
where
    T: 'static + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Nil => Ok(Self::default()),
            Value::Table(tbl) => {
                let mut x = Self::default();
                for pairs in tbl.pairs::<::rlua::Integer, Value<'_>>() {
                    let (_k, t) = pairs?;
                    x.push(T::from_lua(t)?);
                }
                Ok(x)
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
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Nil => Ok(Self::default()),
            Value::Table(tbl) => {
                let mut x = Self::default();
                for pairs in tbl.pairs::<Value<'_>, Value<'_>>() {
                    let (k, t) = pairs?;
                    if let Some(ref x) =
                        x.insert(K::from_lua(k)?, T::from_lua(t)?)
                    {
                        return Err(Error::Insert(format!("{:?}", x)));
                    }
                }
                Ok(x)
            }
            _ => Err(Error::Type(
                "<BTreeMap<K, T> as LuaType>::from_lua}".to_string(),
            )),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for String {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::String(x) => Ok(x.to_str()?.to_string()),
            _ => Err(Error::Type("<String as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Option<T>
where
    T: 'static + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
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
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::String(x) => Ok(x.to_str()?.into()),
            _ => {
                Err(Error::Type("<PathBuf as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ============================================================================
impl LuaType for Uuid {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::String(x) => Ok(Uuid::parse_str(x.to_str()?)?),
            _ => Err(Error::Type("<Uuid as LuaType>::from_lua".to_string())),
        }
    }
}
// ============================================================================
impl<T> LuaType for Vector3<T>
where
    Vector3<T>: 'static,
    T: 'static + Float + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
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
    T: 'static + Float + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
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
    T: 'static + Float + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
        Vector4::from_lua(v).map(Self::from)
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for TraRotSca<T>
where
    T: 'static + Float + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
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
    fn from_lua(v: Value<'_>) -> Result<Self> {
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
impl<'a, 'b> LuaType for LBFTexture<'a, 'b> {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                Uuid::from_lua(tbl.get("uuid")?)?,
                String::from_lua(tbl.get("name")?)?,
                Uuid::from_lua(tbl.get("image")?)?,
                texture_wrap_match(&String::from_lua(tbl.get("wrap_s")?)?)?,
                texture_wrap_match(&String::from_lua(tbl.get("wrap_t")?)?)?,
                texture_filter_match(&String::from_lua(
                    tbl.get("filter_mag")?,
                )?)?,
                texture_filter_match(&String::from_lua(
                    tbl.get("filter_min")?,
                )?)?,
                bool::from_lua(tbl.get("mipmap")?)?,
            )),
            _ => {
                Err(Error::Type("<Texture as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ----------------------------------------------------------------------------
impl<'a, 'b> LuaType for LBFMaterial<'a, 'b> {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                Uuid::from_lua(tbl.get("uuid")?)?,
                String::from_lua(tbl.get("name")?)?,
                Vec::<Option<Uuid>>::from_lua(tbl.get("textures")?)?,
                {
                    let default = Parallax::default();
                    Parallax::new(
                        GLfloat::from_lua(
                            tbl.get::<_, Value<'_>>("parallax.height")?
                                .notnil_or_else(|| {
                                    Value::Number(default.height.into())
                                }),
                        )?,
                        GLfloat::from_lua(
                            tbl.get::<_, Value<'_>>(
                                "parallax.shadow_exponent",
                            )?
                            .notnil_or_else(
                                || {
                                    Value::Number(
                                        default.shadow_exponent.into(),
                                    )
                                },
                            ),
                        )?,
                        GLint::from_lua(
                            tbl.get::<_, Value<'_>>("parallax.loop")?
                                .notnil_or_else(|| {
                                    Value::Number(default.loop_.into())
                                }),
                        )?,
                        GLint::from_lua(
                            tbl.get::<_, Value<'_>>("parallax.shadow_loop")?
                                .notnil_or_else(|| {
                                    Value::Number(default.shadow_loop.into())
                                }),
                        )?,
                    )
                },
                ColorIntensity::from_vec(
                    Vector3::from_lua(tbl.get("diffuse.color")?)?,
                    GLfloat::from_lua(tbl.get("diffuse.intensity")?)?,
                ),
                ColorIntensity::from_vec(
                    Vector3::from_lua(tbl.get("specular.color")?)?,
                    GLfloat::from_lua(tbl.get("specular.intensity")?)?,
                ),
                ColorIntensity::from_vec(
                    Vector3::from_lua(tbl.get("emissive.color")?)?,
                    GLfloat::from_lua(tbl.get("emissive.intensity")?)?,
                ),
                GLfloat::from_lua(tbl.get("shininess")?)?,
                GLfloat::from_lua(tbl.get("alpha")?)?,
                MaterialFlags::default(),
            )),
            _ => {
                Err(Error::Type("<Material as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LBFMesh {
    fn from_lua(v: Value<'_>) -> Result<Self> {
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
    fn from_lua(v: Value<'_>) -> Result<Self> {
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
    V: 'static + Float + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let parent = isize::from_lua(tbl.get(3)?)?;
                Ok(Self::new(
                    String::from_lua(tbl.get(1)?)?,
                    Vector3::<V>::from_lua(tbl.get(2)?)?,
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
    V: 'static + Float + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                Uuid::from_lua(tbl.get("uuid")?)?,
                String::from_lua(tbl.get("name")?)?,
                Vec::<Bone<V>>::from_lua(tbl.get("bones")?)?,
            )),
            _ => {
                Err(Error::Type("<Armature as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ----------------------------------------------------------------------------
impl<'a, 'b> LuaType for LBFModel<'a, 'b> {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => Self::new(
                Uuid::from_lua(tbl.get("uuid")?)?,
                String::from_lua(tbl.get("name")?)?,
                Vec::<Uuid>::from_lua(tbl.get("meshes")?)?,
                Vec::<Uuid>::from_lua(tbl.get("materials")?)?,
                Option::<Uuid>::from_lua(
                    tbl.get::<_, Value<'_>>("armature")?
                        .notnil_or_else(|| Value::Boolean(false)),
                )?,
            ),
            _ => Err(Error::Type("<Model as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LBFLight {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let mut x = Self::new(
                    Uuid::from_lua(tbl.get("uuid")?)?,
                    String::from_lua(tbl.get("name")?)?,
                );
                x.color = Vector3::<GLfloat>::from_lua(tbl.get("color")?)?;
                x.intensity = GLfloat::from_lua(tbl.get("intensity")?)?;
                let light_type = String::from_lua(tbl.get("light_type")?)?;
                if "SUN" != light_type.as_str() {
                    x.flags.insert(LightFlags::POINT);
                    x.kcklkq =
                        Vector3::<GLfloat>::from_lua(tbl.get("kcklkq")?)?;
                    if "SPOT" == light_type.as_str() {
                        x.flags.insert(LightFlags::SPOT);
                        x.exponent = GLfloat::from_lua(tbl.get("exponent")?)?;
                        x.cutoff = GLfloat::from_lua(tbl.get("cutoff")?)?;
                    }
                }
                if bool::from_lua(
                    tbl.get::<_, Value<'_>>("shadow")?
                        .notnil_or_else(|| Value::Boolean(false)),
                )? {
                    x.flags.insert(LightFlags::SHADOW);
                }
                Ok(x)
            }
            _ => {
                Err(Error::Type("<LBFLight as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ----------------------------------------------------------------------------
impl<V> LuaType for Camera<V>
where
    V: Float + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                match String::from_lua(tbl.get("camera_type")?)?.as_str() {
                    "FRUSTUM" => Ok(Camera::new_frustum(
                        Uuid::from_lua(tbl.get("uuid")?)?,
                        String::from_lua(tbl.get("name")?)?,
                        V::from_lua(tbl.get("near")?)?,
                        V::from_lua(tbl.get("far")?)?,
                        Camera::alpha2focus(V::from_lua(tbl.get("alpha")?)?),
                        V::from_lua(tbl.get("aspect")?)?,
                    )),
                    "ORTHO" => Ok(Camera::new_ortho(
                        Uuid::from_lua(tbl.get("uuid")?)?,
                        String::from_lua(tbl.get("name")?)?,
                        V::from_lua(tbl.get("near")?)?,
                        V::from_lua(tbl.get("far")?)?,
                        V::from_lua(tbl.get("width")?)?,
                        V::from_lua(tbl.get("height")?)?,
                    )),
                    _ => Err(Error::Type(
                        "<Camera<VF, VI> as LuaType>::from_lua".to_string(),
                    )),
                }
            }
            _ => Err(Error::Type("<Camera as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl<V> LuaType for Animation<V>
where
    V: 'static + Float + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                Uuid::from_lua(tbl.get("uuid")?)?,
                String::from_lua(tbl.get("name")?)?,
                V::from_lua(tbl.get("fps")?)?,
                &Vec::<Curve<V>>::from_lua(tbl.get("curves")?)?,
            )),
            _ => Err(Error::Type(
                "<Animation as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Interpolation {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::String(x) => match x.to_str()? {
                "CONSTANT" => Ok(Interpolation::Constant),
                "LINEAR" => Ok(Interpolation::Linear),
                "BEZIER" => Ok(Interpolation::Bezier),
                _ => Err(Error::Type(
                    format!(
                        "<Interpolation as LuaType>::from_lua: {}",
                        x.to_str()?
                    )
                    .to_string(),
                )),
            },
            _ => Err(Error::Type(
                "<Interpolation as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
// ----------------------------------------------------------------------------
impl<V> LuaType for Curve<V>
where
    V: 'static + Float + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let type_ = match String::from_lua(tbl.get("type")?)?.as_str()
                {
                    "LOCATION_X" => CurveType::Translate(0),
                    "LOCATION_Y" => CurveType::Translate(1),
                    "LOCATION_Z" => CurveType::Translate(2),
                    "ROTATION_QUATERNION_X" => CurveType::RotateQuaternion(0),
                    "ROTATION_QUATERNION_Y" => CurveType::RotateQuaternion(1),
                    "ROTATION_QUATERNION_Z" => CurveType::RotateQuaternion(2),
                    "ROTATION_QUATERNION_W" => CurveType::RotateQuaternion(3),
                    "SCALE_X" => CurveType::Scale(0),
                    "SCALE_Y" => CurveType::Scale(1),
                    "SCALE_Z" => CurveType::Scale(2),
                    "BONE_LOCATION_X" => CurveType::BoneTranslate(String::from_lua(tbl.get("target")?)?, 0),
                    "BONE_LOCATION_Y" => CurveType::BoneTranslate(String::from_lua(tbl.get("target")?)?, 1),
                    "BONE_LOCATION_Z" => CurveType::BoneTranslate(String::from_lua(tbl.get("target")?)?, 2),
                    "BONE_ROTATION_QUATERNION_X" => CurveType::BoneRotateQuaternion(String::from_lua(tbl.get("target")?)?, 0),
                    "BONE_ROTATION_QUATERNION_Y" => CurveType::BoneRotateQuaternion(String::from_lua(tbl.get("target")?)?, 1),
                    "BONE_ROTATION_QUATERNION_Z" => CurveType::BoneRotateQuaternion(String::from_lua(tbl.get("target")?)?, 2),
                    "BONE_ROTATION_QUATERNION_W" => CurveType::BoneRotateQuaternion(String::from_lua(tbl.get("target")?)?, 3),
                    "BONE_SCALE_X" => CurveType::BoneScale(String::from_lua(tbl.get("target")?)?, 0),
                    "BONE_SCALE_Y" => CurveType::BoneScale(String::from_lua(tbl.get("target")?)?, 1),
                    "BONE_SCALE_Z" => CurveType::BoneScale(String::from_lua(tbl.get("target")?)?, 2),
                    x => {
                        return Err(Error::Type(
                            format!("<Curve as LuaType>::from_lua: invalid type \"{}\"", x).to_string(),
                        ))
                    }
                };
                Ok(Curve::new(
                    type_,
                    &Vec::<Keyframe<V>>::from_lua(tbl.get("keyframes")?)?,
                ))
            }
            _ => Err(Error::Type("<Curve as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl<V> LuaType for Keyframe<V>
where
    V: 'static + Float + LuaType,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Keyframe::new(
                isize::from_lua(tbl.get(1)?)?,
                V::from_lua(tbl.get(2)?)?,
                Interpolation::from_lua(tbl.get(3)?)?,
                V::from_lua(tbl.get(4)?)?,
                V::from_lua(tbl.get(5)?)?,
                V::from_lua(tbl.get(6)?)?,
                V::from_lua(tbl.get(7)?)?,
            )),
            _ => {
                Err(Error::Type("<Keyframe as LuaType>::from_lua".to_string()))
            }
        }
    }
}
// ----------------------------------------------------------------------------
impl<'a, 'b> LuaType for LBFObject<'a, 'b> {
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => {
                let data_type = String::from_lua(tbl.get("data_type")?)?;
                let data_uuid = match data_type.as_str() {
                    "EMPTY" | "ARMATURE" => None,
                    _ => Some(Uuid::from_lua(tbl.get("data_uuid")?)?),
                };
                Ok(Self::new(
                    Uuid::from_lua(tbl.get("uuid")?)?,
                    String::from_lua(tbl.get("name")?)?,
                    Option::<Uuid>::from_lua(
                        tbl.get::<_, Value<'_>>("parent")?
                            .notnil_or_else(|| Value::Boolean(false)),
                    )?,
                    data_type,
                    data_uuid,
                    TraRotSca::from_lua(tbl.get("trarotsca")?)?,
                ))
            }
            _ => Err(Error::Type(
                "<LBFObject as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
// ----------------------------------------------------------------------------
impl<'a, 'b, VF, VI> LuaType for LBFAnimationDriver<'a, 'b, VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn from_lua(v: Value<'_>) -> Result<Self> {
        match v {
            Value::Table(tbl) => Ok(Self::new(
                Uuid::from_lua(tbl.get("uuid")?)?,
                String::from_lua(tbl.get("name")?)?,
                Uuid::from_lua(tbl.get("animation")?)?,
                Uuid::from_lua(tbl.get("object")?)?,
            )),
            _ => Err(Error::Type(
                "<AnimationDriver as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
