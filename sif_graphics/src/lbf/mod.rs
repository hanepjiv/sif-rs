// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/02
//  @date 2018/05/26

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{
    any::TypeId, collections::BTreeMap, fmt::Debug, fs::File, hash::Hash,
    io::Read, path::{Path, PathBuf},
};
// ----------------------------------------------------------------------------
use gl::types::*;
use lua::{ffi::lua_Integer, State as LuaState};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::{Number, Quaternion, Vector3, Vector4};
use sif_three::{Armature, Bone, TraRotSca};
// ----------------------------------------------------------------------------
use super::{
    light, Camera, Image, LightSrc, Material, Model, ObjectSrc, Texture,
};
// ============================================================================
pub use self::{
    error::{Error, Result}, mesh::Mesh, polygon::Polygon,
};
// mod  =======================================================================
pub mod error;
pub mod mesh;
pub mod polygon;
// ////////////////////////////////////////////////////////////////////////////
static CURRENT: lua_Integer = 2;
static AGE: lua_Integer = 0;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait LuaType
trait LuaType: ::std::marker::Sized {
    fn lua_type(t: ::lua::Type) -> bool;
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self>;
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Option<T>
where
    T: 'static + LuaType,
{
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Boolean == t || T::lua_type(t)
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        if let Some(t) = state.type_of(idx) {
            if let ::lua::Type::Boolean = t {
                if bool::from_lua(state, idx)? {
                    Err(Error::Type(format!(
                        "<{:?} as LuaType>::from_lua: invalid 'true'",
                        TypeId::of::<Self>()
                    )))
                } else {
                    Ok(None)
                }
            } else {
                Ok(Some(T::from_lua(state, idx)?))
            }
        } else {
            Err(Error::Type(format!(
                "<{:?} as LuaType>::from_lua",
                TypeId::of::<Self>()
            )))
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for bool {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Boolean == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        Ok(state.to_bool(idx))
    }
}
// ----------------------------------------------------------------------------
impl LuaType for u8 {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Number == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state.to_integerx(idx).map(|x| x as u8).ok_or_else(|| {
            Error::Type(format!(
                "<{:?} as LuaType>::from_lua",
                TypeId::of::<Self>()
            ))
        })
    }
}
// ----------------------------------------------------------------------------
impl LuaType for u32 {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Number == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state.to_integerx(idx).map(|x| x as u32).ok_or_else(|| {
            Error::Type(format!(
                "<{:?} as LuaType>::from_lua",
                TypeId::of::<Self>()
            ))
        })
    }
}
// ----------------------------------------------------------------------------
impl LuaType for usize {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Number == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state.to_integerx(idx).map(|x| x as usize).ok_or_else(|| {
            Error::Type(format!(
                "<{:?} as LuaType>::from_lua",
                TypeId::of::<Self>()
            ))
        })
    }
}
// ----------------------------------------------------------------------------
impl LuaType for isize {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Number == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state.to_integerx(idx).map(|x| x as isize).ok_or_else(|| {
            Error::Type(format!(
                "<{:?} as LuaType>::from_lua",
                TypeId::of::<Self>()
            ))
        })
    }
}
// ----------------------------------------------------------------------------
impl LuaType for lua_Integer {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Number == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state.to_integerx(idx).ok_or_else(|| {
            Error::Type(format!(
                "<{:?} as LuaType>::from_lua",
                TypeId::of::<Self>()
            ))
        })
    }
}
// ----------------------------------------------------------------------------
impl LuaType for GLfloat {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Number == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state.to_numberx(idx).map(|x| x as GLfloat).ok_or_else(|| {
            Error::Type(format!(
                "<{:?} as LuaType>::from_lua",
                TypeId::of::<Self>()
            ))
        })
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Vector3<T>
where
    Vector3<T>: 'static,
    T: 'static + Number + LuaType,
{
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let mut v = Self::default();
        let mut i = 0usize;
        state.push_nil();
        while state.next(idx - 1) {
            if i >= 3 {
                state.pop(2); // next, push_nil
                return Err(Error::Type(format!(
                    "<{:?} as LuaType>::from_lua",
                    TypeId::of::<Self>()
                )));
            }
            let r = T::from_lua(state, -1).map(|n| v[i] = n);
            state.pop(1); // next
            if let Err(e) = r {
                state.pop(1); // push_nil
                return Err(e);
            }
            i += 1;
        }
        Ok(v)
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Vector4<T>
where
    Vector4<T>: 'static,
    T: 'static + Number + LuaType,
{
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let mut v = Self::default();
        let mut i = 0usize;
        state.push_nil();
        while state.next(idx - 1) {
            if i >= 4 {
                state.pop(2); // next, push_nil
                return Err(Error::Type(format!(
                    "<{:?} as LuaType>::from_lua",
                    TypeId::of::<Self>()
                )));
            }
            let r = T::from_lua(state, -1).map(|n| v[i] = n);
            state.pop(1); // next
            if let Err(e) = r {
                state.pop(1); // push_nil
                return Err(e);
            }
            i += 1;
        }
        Ok(v)
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Quaternion<T>
where
    Quaternion<T>: 'static,
    T: 'static + Number + LuaType,
{
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let mut v = Self::default();
        let mut i = 0usize;
        state.push_nil();
        while state.next(idx - 1) {
            if i >= 4 {
                state.pop(2); // next, push_nil
                return Err(Error::Type(format!(
                    "<{:?} as LuaType>::from_lua",
                    TypeId::of::<Self>()
                )));
            }
            let r = T::from_lua(state, -1).map(|n| v[i] = n);
            state.pop(1); // next
            if let Err(e) = r {
                state.pop(1); // push_nil
                return Err(e);
            }
            i += 1;
        }
        Ok(v)
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for TraRotSca<T>
where
    T: 'static + Number + LuaType,
{
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        Ok(TraRotSca::<T>::new(
            state.idxtbl(idx, "translation")?,
            state.idxtbl(idx, "rotation")?,
            state.idxtbl(idx, "scaling")?,
        ))
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Vec<T>
where
    T: LuaType,
{
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let mut v = Self::default();
        state.push_nil();
        while state.next(idx - 1) {
            let r = T::from_lua(state, -1).map(|n| v.push(n));
            state.pop(1); // next
            if let Err(e) = r {
                state.pop(1); // push_nil
                return Err(e);
            }
        }
        Ok(v)
    }
}
// ----------------------------------------------------------------------------
impl<K, T> LuaType for BTreeMap<K, T>
where
    K: Hash + Ord + Clone,
    T: Debug + AsRef<K> + LuaType,
{
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let mut v = Self::default();
        state.push_nil();
        while state.next(idx - 1) {
            let r = T::from_lua(state, -1).map(|n| {
                if let Some(ref x) = v.insert((*n.as_ref()).clone(), n) {
                    Err(Error::Insert(format!("{:?}", x)))
                } else {
                    Ok(())
                }
            });
            state.pop(1); // next
            if let Err(e) = r {
                state.pop(1); // push_nil
                return Err(e);
            }
        }
        Ok(v)
    }
}
// ----------------------------------------------------------------------------
impl LuaType for String {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::String == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state.to_str_in_place(idx).map(String::from).ok_or_else(|| {
            Error::Type(format!(
                "<{:?} as LuaType>::from_lua",
                TypeId::of::<Self>()
            ))
        })
    }
}
// ----------------------------------------------------------------------------
impl LuaType for PathBuf {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::String == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state
            .to_str_in_place(idx)
            .map(PathBuf::from)
            .ok_or_else(|| {
                Error::Type(format!(
                    "<{:?} as LuaType>::from_lua",
                    TypeId::of::<Self>()
                ))
            })
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Uuid {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::String == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        if let Some(key) = state.to_str_in_place(idx) {
            match Uuid::parse_str(key) {
                Err(e) => Err(Error::UuidParse(e)),
                Ok(uuid) => Ok(uuid),
            }
        } else {
            Err(Error::Type(format!(
                "<{:?} as LuaType>::from_lua",
                TypeId::of::<Self>()
            )))
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Image {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let uuid = Uuid::from_lua(state, idx - 1)?;
        let name = state.idxtbl::<String, _>(idx, "name")?;
        let dimension = state.idxtbl::<u8, _>(idx, "dimension")?;
        let source = state.idxtbl::<String, _>(idx, "source")?;
        match source.as_str() {
            "FILE" => {
                let path = state.idxtbl::<PathBuf, _>(idx, "path")?;
                Ok(Image::new_file(uuid, name, dimension, path))
            }
            _ => Err(Error::Type(format!(
                "<{:?} as LuaType>::from_lua: invalid image source",
                TypeId::of::<Self>()
            ))),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Texture {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let uuid = Uuid::from_lua(state, idx - 1)?;
        let name = state.idxtbl::<String, _>(idx, "name")?;
        let image = state.idxtbl(idx, "image")?;

        let wrap_match = |s: String| -> Result<GLenum> {
            match s.as_str() {
                "CLAMP_TO_EDGE" => Ok(::gl::CLAMP_TO_EDGE),
                "REPEAT" => Ok(::gl::REPEAT),
                "MIRRORED_REPEAT" => Ok(::gl::MIRRORED_REPEAT),
                _ => Err(Error::Type(format!(
                    "<{:?} as LuaType>::from_lua: invalid wrap",
                    TypeId::of::<Self>()
                ))),
            }
        };
        let wrap_s = wrap_match(state.idxtbl(idx, "wrap_s")?)?;
        let wrap_t = wrap_match(state.idxtbl(idx, "wrap_t")?)?;

        let filter_match = |s: String| -> Result<GLenum> {
            match s.as_str() {
                "NEAREST" => Ok(::gl::NEAREST),
                "LINEAR" => Ok(::gl::LINEAR),
                "NEAREST_MIPMAP_NEAREST" => Ok(::gl::NEAREST_MIPMAP_NEAREST),
                "NEAREST_MIPMAP_LINEAR" => Ok(::gl::NEAREST_MIPMAP_LINEAR),
                "LINEAR_MIPMAP_NEAREST" => Ok(::gl::LINEAR_MIPMAP_NEAREST),
                "LINEAR_MIPMAP_LINEAR" => Ok(::gl::LINEAR_MIPMAP_LINEAR),
                _ => Err(Error::Type(format!(
                    "<{:?} as LuaType>::from_lua: invalid filter",
                    TypeId::of::<Self>()
                ))),
            }
        };
        let filter_mag = filter_match(state.idxtbl(idx, "filter_mag")?)?;
        let filter_min = filter_match(state.idxtbl(idx, "filter_min")?)?;

        let mipmap = state.idxtbl(idx, "mipmap")?;

        Ok(Texture::new(
            uuid, name, wrap_s, wrap_t, filter_mag, filter_min, mipmap, image,
        ))
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Material {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let mut m = Material::new(
            Uuid::from_lua(state, idx - 1)?,
            state.idxtbl::<String, _>(idx, "name")?,
        );
        m.diffuse.color = state.idxtbl(idx, "diffuse.color")?;
        m.diffuse.intensity = state.idxtbl(idx, "diffuse.intensity")?;
        m.specular.color = state.idxtbl(idx, "specular.color")?;
        m.specular.intensity = state.idxtbl(idx, "specular.intensity")?;
        m.emissive.color = state.idxtbl(idx, "emissive.color")?;
        m.emissive.intensity = state.idxtbl(idx, "emissive.intensity")?;
        m.shininess = state.idxtbl(idx, "shininess")?;
        m.alpha = state.idxtbl(idx, "alpha")?;
        m.parallax.height =
            state.idxtbl(idx, "parallax.height").unwrap_or(0.025);
        if let Ok(textures) = state.idxtbl(idx, "textures") {
            m.textures = Some(Err(textures));
        }
        Ok(m)
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Mesh {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        Mesh::new(
            Uuid::from_lua(state, idx - 1)?,
            state.idxtbl::<String, _>(idx, "name")?,
            state.idxtbl::<Vec<GLfloat>, _>(idx, "position")?,
            state.idxtbl::<Vec<GLfloat>, _>(idx, "normal")?,
            state
                .idxtbl::<Vec<GLfloat>, _>(idx, "coord")
                .unwrap_or_default(),
            state
                .idxtbl::<Vec<GLfloat>, _>(idx, "bone")
                .unwrap_or_default(),
            state
                .idxtbl::<Vec<GLfloat>, _>(idx, "weight")
                .unwrap_or_default(),
            state.idxtbl::<Vec<Polygon>, _>(idx, "polygons")?,
        )
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Polygon {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state.push_nil();

        let flags = if !state.next(idx - 1) {
            return Err(Error::Type("Polygon: flags".to_string()));
        } else {
            u32::from_lua(state, -1)
        };
        state.pop(1); // next

        let material = if !state.next(idx - 1) {
            return Err(Error::Type("Polygon: material_index".to_string()));
        } else {
            isize::from_lua(state, -1)
        };
        state.pop(1); // next

        let indices = if !state.next(idx - 1) {
            return Err(Error::Type("Polygon: indices".to_string()));
        } else {
            Vec::<u32>::from_lua(state, -1)
        };
        state.pop(1); // next

        state.pop(1); // push_nil

        let bits = flags?;
        let midx = material?;

        Polygon::new(
            polygon::Flags::from_bits(bits).ok_or_else(|| {
                Error::Type(format!("Polygon: flags from_bits({})", bits))
            })?,
            if -1 < midx { Some(midx as usize) } else { None },
            &(indices?)[..],
        )
    }
}
// ----------------------------------------------------------------------------
impl<V> LuaType for Armature<V>
where
    V: 'static + Number + LuaType,
{
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        Ok(Armature::new(
            Uuid::from_lua(state, idx - 1)?,
            state.idxtbl::<String, _>(idx, "name")?,
            state.idxtbl(idx, "bones")?,
        ))
    }
}
// ----------------------------------------------------------------------------
impl<V> LuaType for Bone<V>
where
    V: 'static + Number + LuaType,
{
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state.push_nil();

        let offset = if !state.next(idx - 1) {
            return Err(Error::Type("Bone: offset".to_string()));
        } else {
            Vector3::<V>::from_lua(state, -1)
        };
        state.pop(1); // next

        let parent = if !state.next(idx - 1) {
            return Err(Error::Type("Bone: parent".to_string()));
        } else {
            isize::from_lua(state, -1)
        };
        state.pop(1); // next

        state.pop(1); // push_nil

        let o = offset?;
        let p = parent?;

        Ok(Bone::new(o, if 0 > p { None } else { Some(p as usize) }))
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Model {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let mut model = Model::new(
            Uuid::from_lua(state, idx - 1)?,
            state.idxtbl::<String, _>(idx, "name")?,
        );
        model.meshes = Some(Err(state.idxtbl(idx, "meshes")?));
        model.materials = Some(Err(state.idxtbl(idx, "materials")?));
        if let Ok(armature) = state.idxtbl(idx, "armature") {
            model.armature = Some(Err(armature));
        }
        Ok(model)
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LightSrc {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let mut light = LightSrc::new(
            Uuid::from_lua(state, idx - 1)?,
            state.idxtbl::<String, _>(idx, "name")?,
        );
        light.color = state.idxtbl::<Vector3<GLfloat>, _>(idx, "color")?;
        light.intensity = state.idxtbl(idx, "intensity")?;
        let light_type = state.idxtbl::<String, _>(idx, "light_type")?;
        if "SUN" != light_type.as_str() {
            light.flags.insert(light::Flags::POINT);
            light.kcklkq = state.idxtbl::<Vector3<GLfloat>, _>(idx, "kcklkq")?;
            if "SPOT" == light_type.as_str() {
                light.flags.insert(light::Flags::SPOT);
                light.exponent = state.idxtbl(idx, "exponent")?;
                light.cutoff = state.idxtbl(idx, "cutoff")?;
            }
        }
        if state.idxtbl(idx, "shadow").unwrap_or(false) {
            light.flags.insert(light::Flags::SHADOW);
        }
        Ok(light)
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Camera {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let uuid = Uuid::from_lua(state, idx - 1)?;
        let name = state.idxtbl::<String, _>(idx, "name")?;
        let near = state.idxtbl(idx, "near")?;
        let far = state.idxtbl(idx, "far")?;
        let camera_type = state.idxtbl::<String, _>(idx, "camera_type")?;
        match camera_type.as_str() {
            "FRUSTUM" => {
                let alpha = state.idxtbl(idx, "alpha")?;
                let aspect = state.idxtbl(idx, "aspect")?;
                Ok(Camera::new_frustum(
                    uuid,
                    name,
                    near,
                    far,
                    Camera::alpha2focus(alpha),
                    aspect,
                ))
            }
            "ORTHO" => {
                let width = state.idxtbl(idx, "width")?;
                let height = state.idxtbl(idx, "height")?;
                Ok(Camera::new_ortho(uuid, name, near, far, width, height))
            }
            _ => Err(Error::Type(format!(
                "<{:?} as LuaType>::from_lua: invalid camera_type",
                TypeId::of::<Self>()
            ))),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for ObjectSrc {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let object = ObjectSrc::new(
            state.idxtbl::<Uuid, _>(idx, "uuid")?,
            state.idxtbl::<String, _>(idx, "name")?,
            state.idxtbl(idx, "parent").ok(),
            state.idxtbl::<String, _>(idx, "data_type")?,
            state.idxtbl::<Uuid, _>(idx, "data_uuid")?,
            state.idxtbl(idx, "trarotsca")?,
        );
        Ok(object)
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
trait LuaStateEx {
    // ========================================================================
    /// idxtbl
    fn idxtbl<T>(&mut self, idx: i32, key: impl AsRef<str>) -> Result<T>
    where
        T: LuaType;
}
// ----------------------------------------------------------------------------
impl LuaStateEx for LuaState {
    // ========================================================================
    /// idxtbl
    fn idxtbl<T>(&mut self, idx: i32, key: impl AsRef<str>) -> Result<T>
    where
        T: LuaType,
    {
        debug!("::lbf::LuaStateEx::idxtbl: {}", key.as_ref());
        self.push_string(key.as_ref());
        let t = self.get_table(idx - 1);
        let result = if !T::lua_type(t) {
            Err(Error::Type(format!(
                "::lbf::LBF::idxtbl(idx, '{}')",
                key.as_ref()
            )))
        } else {
            let result = T::from_lua(self, -1);
            if let Err(ref e) = result {
                error!("::lbf::LuaStateEx::idxtbl: {}", e);
            }
            result
        };
        self.pop(1); // get_table
        result
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct LBF
#[derive(Debug, Default, Clone)]
pub struct LBF {
    /// current
    current: lua_Integer,
    /// images
    images: BTreeMap<Uuid, Image>,
    /// textures
    textures: BTreeMap<Uuid, Texture>,
    /// materials
    materials: BTreeMap<Uuid, Material>,
    /// meshes
    meshes: BTreeMap<Uuid, Mesh>,
    /// armatures
    armatures: BTreeMap<Uuid, Armature<GLfloat>>,
    /// models
    models: BTreeMap<Uuid, Model>,
    /// lights
    lights: BTreeMap<Uuid, LightSrc>,
    /// cameras
    cameras: BTreeMap<Uuid, Camera>,
    /// objects
    objects: Vec<ObjectSrc>,
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, LightSrc>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, LightSrc> {
        &self.lights
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, LightSrc>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, LightSrc> {
        &mut self.lights
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Camera>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Camera> {
        &self.cameras
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Camera>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Camera> {
        &mut self.cameras
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Image>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Image> {
        &self.images
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Image>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Image> {
        &mut self.images
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Texture>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Texture> {
        &self.textures
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Texture>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Texture> {
        &mut self.textures
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Material>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Material> {
        &self.materials
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Material>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Material> {
        &mut self.materials
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Mesh>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Mesh> {
        &self.meshes
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Mesh>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Mesh> {
        &mut self.meshes
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Armature<GLfloat>>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Armature<GLfloat>> {
        &self.armatures
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Armature<GLfloat>>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Armature<GLfloat>> {
        &mut self.armatures
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Model>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Model> {
        &self.models
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Model>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Model> {
        &mut self.models
    }
}
// ============================================================================
impl AsRef<Vec<ObjectSrc>> for LBF {
    fn as_ref(&self) -> &Vec<ObjectSrc> {
        &self.objects
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<ObjectSrc>> for LBF {
    fn as_mut(&mut self) -> &mut Vec<ObjectSrc> {
        &mut self.objects
    }
}
// ============================================================================
impl LBF {
    // ========================================================================
    /// get_current
    fn get_current(state: &mut LuaState, idx: i32) -> Result<lua_Integer> {
        let current = state.idxtbl(idx, "current")?;
        if current < (CURRENT - AGE) || CURRENT < current {
            Err(Error::Current)
        } else {
            Ok(current)
        }
    }
    // ========================================================================
    /// load
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let mut src = String::new();
        let _ = File::open(path.as_ref())?.read_to_string(&mut src)?;
        Self::load_str(path, &src)
    }
    // ========================================================================
    /// load_str
    pub fn load_str(
        path: impl AsRef<Path>,
        src: impl AsRef<str>,
    ) -> Result<Self> {
        let mut state = LuaState::new();
        let e = state.do_string(src.as_ref());
        if e.is_err() {
            Err(Error::Load(e))
        } else {
            let current = Self::get_current(&mut state, -1)?;
            let mut images: BTreeMap<Uuid, Image> =
                state.idxtbl(-1, "images").unwrap_or_default();
            let path_base = path.as_ref().parent().ok_or_else(|| {
                Error::OptNone(
                    "lbf::LBF::load_str: path.as_ref.parent".to_string(),
                )
            })?;
            for v in &mut images.values_mut() {
                if let Image::File(ref mut im) = *v {
                    let _ = im.set_path_base(path_base);
                }
            }
            Ok(LBF {
                current,
                images,
                textures: state.idxtbl(-1, "textures").unwrap_or_default(),
                materials: state.idxtbl(-1, "materials").unwrap_or_default(),
                meshes: state.idxtbl(-1, "meshes").unwrap_or_default(),
                armatures: state.idxtbl(-1, "armatures").unwrap_or_default(),
                models: state.idxtbl(-1, "models").unwrap_or_default(),
                lights: state.idxtbl(-1, "lights").unwrap_or_default(),
                cameras: state.idxtbl(-1, "cameras").unwrap_or_default(),
                objects: state.idxtbl(-1, "objects").unwrap_or_default(),
            })
        }
    }
}
