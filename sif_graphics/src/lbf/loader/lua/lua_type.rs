// -*- mode:rust; coding:utf-8-unix; -*-

//! lua_type.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/06/13
//  @date 2018/06/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{collections::BTreeMap, fmt::Debug, hash::Hash, path::PathBuf};
// ----------------------------------------------------------------------------
use gl::types::*;
use lua::{ffi::lua_Integer, State as LuaState};
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
    lua_state_ex::LuaStateEx,
    Error, Result,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait LuaType
pub(crate) trait LuaType: ::std::marker::Sized {
    fn lua_type(t: ::lua::Type) -> bool;
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self>;
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
            Error::Type("<u8 as LuaType>::from_lua".to_string())
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
            Error::Type("<u32 as LuaType>::from_lua".to_string())
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
            Error::Type("<usize as LuaType>::from_lua".to_string())
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
            Error::Type("<isize as LuaType>::from_lua".to_string())
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
            Error::Type("<lua_Integer as LuaType>::from_lua".to_string())
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
            Error::Type("<GLfloat as LuaType>::from_lua".to_string())
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
                return Err(Error::Type(
                    "<Vector3<T> as LuaType>::from_lua".to_string(),
                ));
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
                return Err(Error::Type(
                    "<Vector4<T> as LuaType>::from_lua".to_string(),
                ));
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
        Vector4::from_lua(state, idx).map(|x| x.into())
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
            state.idxtbl(idx, &"translation")?,
            state.idxtbl(idx, &"rotation")?,
            state.idxtbl(idx, &"scaling")?,
        ))
    }
}
// ----------------------------------------------------------------------------
impl<T> LuaType for Vec<T>
where
    T: 'static + LuaType,
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
    K: 'static + Hash + Ord + Clone,
    T: 'static + Debug + AsRef<K> + LuaType,
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
            Error::Type("<String as LuaType>::from_lua".to_string())
        })
    }
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
                    Err(Error::Type(
                        "<Option<T> as LuaType>::from_lua".to_string(),
                    ))
                } else {
                    Ok(None)
                }
            } else {
                Ok(Some(T::from_lua(state, idx)?))
            }
        } else {
            Err(Error::Type("<Option<T> as LuaType>::from_lua".to_string()))
        }
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
                Error::Type("<PathBuf as LuaType>::from_lua".to_string())
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
            Err(Error::Type("<Uuid as LuaType>::from_lua".to_string()))
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Image {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        match state.idxtbl::<String>(idx, &"source")?.as_str() {
            "FILE" => Ok(Image::new_file(
                Uuid::from_lua(state, idx - 1)?,
                state.idxtbl::<String>(idx, &"name")?,
                state.idxtbl::<u8>(idx, &"dimension")?,
                state.idxtbl::<PathBuf>(idx, &"path")?,
            )),
            _ => Err(Error::Type("<Image as LuaType>::from_lua".to_string())),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for Texture {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        Ok(Texture::new(
            Uuid::from_lua(state, idx - 1)?,
            state.idxtbl::<String>(idx, &"name")?,
            texture_wrap_match(state.idxtbl(idx, &"wrap_s")?)?,
            texture_wrap_match(state.idxtbl(idx, &"wrap_t")?)?,
            texture_filter_match(state.idxtbl(idx, &"filter_mag")?)?,
            texture_filter_match(state.idxtbl(idx, &"filter_min")?)?,
            state.idxtbl(idx, &"mipmap")?,
            state.idxtbl(idx, &"image")?,
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
            state.idxtbl::<String>(idx, &"name")?,
        );
        m.diffuse.color = state.idxtbl(idx, &"diffuse.color")?;
        m.diffuse.intensity = state.idxtbl(idx, &"diffuse.intensity")?;
        m.specular.color = state.idxtbl(idx, &"specular.color")?;
        m.specular.intensity = state.idxtbl(idx, &"specular.intensity")?;
        m.emissive.color = state.idxtbl(idx, &"emissive.color")?;
        m.emissive.intensity = state.idxtbl(idx, &"emissive.intensity")?;
        m.shininess = state.idxtbl(idx, &"shininess")?;
        m.alpha = state.idxtbl(idx, &"alpha")?;
        m.parallax.height =
            state.idxtbl(idx, &"parallax.height").unwrap_or(0.025);
        if let Ok(textures) = state.idxtbl(idx, &"textures") {
            m.textures = Some(Err(textures));
        }
        Ok(m)
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LBFMesh {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        LBFMesh::new(
            Uuid::from_lua(state, idx - 1)?,
            state.idxtbl::<String>(idx, &"name")?,
            state.idxtbl::<Vec<GLfloat>>(idx, &"position")?,
            state.idxtbl::<Vec<GLfloat>>(idx, &"normal")?,
            state
                .idxtbl::<Vec<GLfloat>>(idx, &"coord")
                .unwrap_or_default(),
            state
                .idxtbl::<Vec<GLfloat>>(idx, &"bone")
                .unwrap_or_default(),
            state
                .idxtbl::<Vec<GLfloat>>(idx, &"weight")
                .unwrap_or_default(),
            state.idxtbl::<Vec<LBFPolygon>>(idx, &"polygons")?,
        )
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LBFPolygon {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        state.push_nil();

        let flags = if !state.next(idx - 1) {
            return Err(Error::Type("LBFPolygon: flags".to_string()));
        } else {
            u32::from_lua(state, -1)
        };
        state.pop(1); // next

        let material = if !state.next(idx - 1) {
            return Err(Error::Type("LBFPolygon: material_index".to_string()));
        } else {
            isize::from_lua(state, -1)
        };
        state.pop(1); // next

        let indices = if !state.next(idx - 1) {
            return Err(Error::Type("LBFPolygon: indices".to_string()));
        } else {
            Vec::<u32>::from_lua(state, -1)
        };
        state.pop(1); // next

        state.pop(1); // push_nil

        let bits = flags?;
        let midx = material?;

        LBFPolygon::new(
            LBFPolygonFlags::from_bits(bits).ok_or_else(|| {
                Error::Type(format!("LBFPolygon: flags from_bits({})", bits))
            })?,
            if -1 < midx { Some(midx as usize) } else { None },
            &(indices?)[..],
        )
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

        Ok(Bone::new(o, if p < 0 { None } else { Some(p as usize) }))
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
            state.idxtbl::<String>(idx, &"name")?,
            state.idxtbl(idx, &"bones")?,
        ))
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
            state.idxtbl::<String>(idx, &"name")?,
        );
        model.meshes = Some(Err(state.idxtbl(idx, &"meshes")?));
        model.materials = Some(Err(state.idxtbl(idx, &"materials")?));
        if let Ok(armature) = state.idxtbl(idx, &"armature") {
            model.armature = Some(Err(armature));
        }
        Ok(model)
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LBFLight {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        let mut light = LBFLight::new(
            Uuid::from_lua(state, idx - 1)?,
            state.idxtbl::<String>(idx, &"name")?,
        );
        light.color = state.idxtbl::<Vector3<GLfloat>>(idx, &"color")?;
        light.intensity = state.idxtbl(idx, &"intensity")?;
        let light_type = state.idxtbl::<String>(idx, &"light_type")?;
        if "SUN" != light_type.as_str() {
            light.flags.insert(LightFlags::POINT);
            light.kcklkq = state.idxtbl::<Vector3<GLfloat>>(idx, &"kcklkq")?;
            if "SPOT" == light_type.as_str() {
                light.flags.insert(LightFlags::SPOT);
                light.exponent = state.idxtbl(idx, &"exponent")?;
                light.cutoff = state.idxtbl(idx, &"cutoff")?;
            }
        }
        if state.idxtbl(idx, &"shadow").unwrap_or(false) {
            light.flags.insert(LightFlags::SHADOW);
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
        match state.idxtbl::<String>(idx, &"camera_type")?.as_str() {
            "FRUSTUM" => Ok(Camera::new_frustum(
                Uuid::from_lua(state, idx - 1)?,
                state.idxtbl::<String>(idx, &"name")?,
                state.idxtbl(idx, &"near")?,
                state.idxtbl(idx, &"far")?,
                Camera::alpha2focus(state.idxtbl(idx, &"alpha")?),
                state.idxtbl(idx, &"aspect")?,
            )),
            "ORTHO" => Ok(Camera::new_ortho(
                Uuid::from_lua(state, idx - 1)?,
                state.idxtbl::<String>(idx, &"name")?,
                state.idxtbl(idx, &"near")?,
                state.idxtbl(idx, &"far")?,
                state.idxtbl(idx, &"width")?,
                state.idxtbl(idx, &"height")?,
            )),
            _ => Err(Error::Type(
                "<LBFPolygon as LuaType>::from_lua".to_string(),
            )),
        }
    }
}
// ----------------------------------------------------------------------------
impl LuaType for LBFObject {
    fn lua_type(t: ::lua::Type) -> bool {
        ::lua::Type::Table == t
    }
    fn from_lua(state: &mut LuaState, idx: ::lua::Index) -> Result<Self> {
        Ok(LBFObject::new(
            state.idxtbl::<Uuid>(idx, &"uuid")?,
            state.idxtbl::<String>(idx, &"name")?,
            state.idxtbl::<Uuid>(idx, &"parent").ok(),
            state.idxtbl::<String>(idx, &"data_type")?,
            state.idxtbl::<Uuid>(idx, &"data_uuid")?,
            state.idxtbl(idx, &"trarotsca")?,
        ))
    }
}
