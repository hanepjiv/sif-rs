// -*- mode:rust; coding:utf-8-unix; -*-

//! scene.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/02/27
//  @date 2018/06/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::collections::BTreeMap;
// use ::std::result::Result as StdResult;
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, Manager};
use sif_three::{Armature, Graph, Node, NodeHolder, Pose, TraRotSca};
// ----------------------------------------------------------------------------
use super::{
    lbf, lbf::LBF, Camera, Error, Image, Light, Material, Mesh, Model, Object,
    ObjectData, Result, Texture,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Scene
#[derive(Debug)]
pub struct Scene {
    /// graph
    graph: Graph<GLfloat>,
    /// images
    images: Manager<Image>,
    /// textures
    textures: Manager<Texture>,
    /// materials
    materials: Manager<Material>,
    /// meshes
    meshes: Manager<Mesh>,
    /// armatures
    armatures: Manager<Armature<GLfloat>>,
    /// models
    models: Manager<Model>,
    /// lights
    lights: Manager<Light>,
    /// cameras
    cameras: Manager<Camera>,
    /// objects
    objects: Manager<Object>,
}
// ============================================================================
impl Scene {
    // ========================================================================
    /// new
    pub fn new(uuid: Uuid) -> Result<Self> {
        Ok(Scene {
            graph: Graph::<GLfloat>::new(uuid)?,
            images: Manager::default(),
            textures: Manager::default(),
            materials: Manager::default(),
            meshes: Manager::default(),
            armatures: Manager::default(),
            models: Manager::default(),
            lights: Manager::default(),
            cameras: Manager::default(),
            objects: Manager::default(),
        })
    }
    // ========================================================================
    /// update
    pub fn update(&mut self) -> Result<&mut Self> {
        self.graph.update();

        for (ref mut _k, ref mut v) in self.objects.iter() {
            let _ = v.as_ref().borrow_mut().update()?;
        }

        Ok(self)
    }
    // ========================================================================
    /// load_lbf
    pub fn load_lbf(
        &mut self,
        lbf: &mut LBF,
        texture_size: GLint,
    ) -> Result<&mut Self> {
        {
            // image
            let r: &mut BTreeMap<Uuid, Image> = lbf.as_mut();
            let keys: Vec<_> = r.keys().cloned().collect();
            for k in keys {
                if let Some(v) = r.remove(&k) {
                    info!(
                        "Image: \"{}\", {:?}, {:?}",
                        AsRef::<String>::as_ref(&v),
                        k,
                        k.as_bytes()
                    );
                    let _ = self.images.insert(v);
                }
            }
        }
        {
            // texture
            let r: &mut BTreeMap<Uuid, Texture> = lbf.as_mut();
            let keys: Vec<_> = r.keys().cloned().collect();
            for k in keys {
                if let Some(v) = r.remove(&k) {
                    info!(
                        "Texture: \"{}\", {:?}, {:?}",
                        AsRef::<String>::as_ref(&v),
                        k,
                        k.as_bytes()
                    );
                    let _ = self
                        .textures
                        .insert(Texture::from_lbf(v, &self.images)?);
                }
            }
        }
        {
            // material
            let r: &mut BTreeMap<Uuid, Material> = lbf.as_mut();
            let keys: Vec<_> = r.keys().cloned().collect();
            for k in keys {
                if let Some(mut v) = r.remove(&k) {
                    info!(
                        "Material: \"{}\", {:?}, {:?}",
                        AsRef::<String>::as_ref(&v),
                        k,
                        k.as_bytes()
                    );
                    v.prepare(&self.textures)?;
                    let _ = self.materials.insert(v)?;
                }
            }
        }
        {
            // mesh
            let r: &mut BTreeMap<Uuid, lbf::LBFMesh> = lbf.as_mut();
            let keys: Vec<_> = r.keys().cloned().collect();
            for k in keys {
                if let Some(ref v) = r.remove(&k) {
                    info!(
                        "Mesh: \"{}\", {:?}, {:?}",
                        AsRef::<String>::as_ref(&v),
                        k,
                        k.as_bytes()
                    );
                    let _ = self.meshes.insert(Mesh::from_lbf(v)?)?;
                }
            }
        }
        {
            // armature
            let r: &mut BTreeMap<Uuid, Armature<GLfloat>> = lbf.as_mut();
            let keys: Vec<_> = r.keys().cloned().collect();
            for k in keys {
                if let Some(v) = r.remove(&k) {
                    info!(
                        "Armature: \"{}\", {:?}, {:?}",
                        AsRef::<String>::as_ref(&v),
                        k,
                        k.as_bytes()
                    );
                    let _ = self.armatures.insert(v)?;
                }
            }
        }
        {
            // models
            let r: &mut BTreeMap<Uuid, Model> = lbf.as_mut();
            let keys: Vec<_> = r.keys().cloned().collect();
            for k in keys {
                if let Some(mut v) = r.remove(&k) {
                    info!(
                        "Model: \"{}\", {:?}, {:?}",
                        AsRef::<String>::as_ref(&v),
                        k,
                        k.as_bytes()
                    );
                    v.prepare(&self.meshes, &self.materials, &self.armatures)?;
                    let _ = self.models.insert(v)?;
                }
            }
        }
        {
            // lights
            let r: &mut BTreeMap<Uuid, lbf::LBFLight> = lbf.as_mut();
            let keys: Vec<_> = r.keys().cloned().collect();
            for k in keys {
                if let Some(v) = r.remove(&k) {
                    info!(
                        "Light: \"{}\", {:?}, {:?}",
                        AsRef::<String>::as_ref(&v),
                        k,
                        k.as_bytes()
                    );
                    let _ =
                        self.lights.insert(Light::from_lbf(v, texture_size)?)?;
                }
            }
        }
        {
            // cameras
            let r: &mut BTreeMap<Uuid, Camera> = lbf.as_mut();
            let keys: Vec<_> = r.keys().cloned().collect();
            for k in keys {
                if let Some(v) = r.remove(&k) {
                    info!(
                        "Camera: \"{}\", {:?}, {:?}",
                        AsRef::<String>::as_ref(&v),
                        k,
                        k.as_bytes()
                    );
                    let _ = self.cameras.insert(v)?;
                }
            }
        }
        {
            // objects
            for v in AsRef::<Vec<lbf::LBFObject>>::as_ref(lbf).iter() {
                let u = AsRef::<Uuid>::as_ref(&v);
                info!(
                    "Object: \"{}\", {:?}, {:?}",
                    AsRef::<String>::as_ref(&v),
                    u,
                    u.as_bytes()
                );
                if let Some(mut obj) = match v.data_type.as_str() {
                    "LIGHT" => self.lights.get(&v.data_uuid).map(|m| {
                        Object::new(
                            *AsRef::<Uuid>::as_ref(&v),
                            AsRef::<String>::as_ref(&v).as_str(),
                            ObjectData::Light(m.clone()),
                        )
                    }),
                    "CAMERA" => self.cameras.get(&v.data_uuid).map(|m| {
                        Object::new(
                            *AsRef::<Uuid>::as_ref(&v),
                            AsRef::<String>::as_ref(&v).as_str(),
                            ObjectData::Camera(m.clone()),
                        )
                    }),
                    "ARMATURE" => self.armatures.get(&v.data_uuid).map(|m| {
                        Object::new(
                            *AsRef::<Uuid>::as_ref(&v),
                            AsRef::<String>::as_ref(&v).as_str(),
                            ObjectData::Armature(m.clone()),
                        )
                    }),
                    "MODEL" => self.models.get(&v.data_uuid).map(|m| {
                        let armature_len =
                            (*m.as_ref().borrow()).armature_len();
                        let pose = if 0 < armature_len {
                            Some(Pose::<GLfloat>::new(armature_len))
                        } else {
                            None
                        };
                        Object::new(
                            *AsRef::<Uuid>::as_ref(&v),
                            AsRef::<String>::as_ref(&v).as_str(),
                            ObjectData::Model(m.clone(), pose),
                        )
                    }),
                    _ => None,
                } {
                    let parent: Result<
                        Option<ManagedValue<Node<GLfloat>>>,
                    > = if let Some(p) = v.parent {
                        Ok(Some(self.graph.get(&p).ok_or_else(|| {
                            Error::OptNone(
                                "graphics: scene: load_lbf: self.graph.get"
                                    .to_string(),
                            )
                        })?))
                    } else {
                        Ok(None)
                    };
                    let _ = self
                        .graph
                        .insert(AsRef::<Uuid>::as_ref(&v).clone(), parent?)?;
                    let node = self.graph.get(v.as_ref()).ok_or_else(|| {
                        Error::OptNone(
                            "graphics: scene: load_lbf: self.graph.get"
                                .to_string(),
                        )
                    })?;
                    {
                        let mut m = node.as_ref().borrow_mut();
                        let trs = AsMut::<TraRotSca<GLfloat>>::as_mut(&mut *m);
                        trs.translation = v.trarotsca.translation;
                        trs.rotation = v.trarotsca.rotation;
                        trs.scaling = v.trarotsca.scaling;
                    }
                    obj.set_node(Some(node));
                    let _ = self.objects.insert(obj)?;
                } else {
                    return Err(Error::ManagedNotFound(v.data_uuid));
                }
            }
        }
        Ok(self)
    }
}
// ============================================================================
impl AsRef<Manager<Image>> for Scene {
    fn as_ref(&self) -> &Manager<Image> {
        &self.images
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Image>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Image> {
        &mut self.images
    }
}
// ============================================================================
impl AsRef<Manager<Texture>> for Scene {
    fn as_ref(&self) -> &Manager<Texture> {
        &self.textures
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Texture>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Texture> {
        &mut self.textures
    }
}
// ============================================================================
impl AsRef<Manager<Material>> for Scene {
    fn as_ref(&self) -> &Manager<Material> {
        &self.materials
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Material>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Material> {
        &mut self.materials
    }
}
// ============================================================================
impl AsRef<Manager<Mesh>> for Scene {
    fn as_ref(&self) -> &Manager<Mesh> {
        &self.meshes
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Mesh>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Mesh> {
        &mut self.meshes
    }
}
// ============================================================================
impl AsRef<Manager<Model>> for Scene {
    fn as_ref(&self) -> &Manager<Model> {
        &self.models
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Model>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Model> {
        &mut self.models
    }
}
// ============================================================================
impl AsRef<Manager<Light>> for Scene {
    fn as_ref(&self) -> &Manager<Light> {
        &self.lights
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Light>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Light> {
        &mut self.lights
    }
}
// ============================================================================
impl AsRef<Manager<Camera>> for Scene {
    fn as_ref(&self) -> &Manager<Camera> {
        &self.cameras
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Camera>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Camera> {
        &mut self.cameras
    }
}
// ============================================================================
impl AsRef<Manager<Object>> for Scene {
    fn as_ref(&self) -> &Manager<Object> {
        &self.objects
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Object>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Object> {
        &mut self.objects
    }
}
