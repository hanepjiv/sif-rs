// -*- mode:rust; coding:utf-8-unix; -*-

//! scene.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/02/27
//  @date 2018/06/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::Manager;
use sif_three::{Armature, Graph};
// ----------------------------------------------------------------------------
use super::{
    lbf, lbf::LBF, Camera, Image, Light, Material, Mesh, Model, Object,
    Result, Texture,
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
    /// fn new
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
    /// fn from_lbf
    pub fn from_lbf(
        &mut self,
        mut lbf: LBF,
        texture_size: GLint,
    ) -> Result<&mut Self> {
        while let Some(v) = AsMut::<Vec<Image>>::as_mut(&mut lbf).pop() {
            info!(
                "Image: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = self.images.insert(v)?;
        }

        while let Some(v) = AsMut::<Vec<Texture>>::as_mut(&mut lbf).pop() {
            info!(
                "Texture: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = self.textures.insert(Texture::from_lbf(v, &self.images)?)?;
        }

        while let Some(mut v) = AsMut::<Vec<Material>>::as_mut(&mut lbf).pop()
        {
            info!(
                "Material: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            v.prepare(&self.textures)?;
            let _ = self.materials.insert(v)?;
        }

        while let Some(v) = AsMut::<Vec<lbf::LBFMesh>>::as_mut(&mut lbf).pop()
        {
            info!(
                "Mesh: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = self.meshes.insert(Mesh::from_lbf(v)?)?;
        }

        while let Some(v) =
            AsMut::<Vec<Armature<GLfloat>>>::as_mut(&mut lbf).pop()
        {
            info!(
                "Armature<GLfloat>: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = self.armatures.insert(v)?;
        }

        while let Some(mut v) = AsMut::<Vec<Model>>::as_mut(&mut lbf).pop() {
            info!(
                "Model: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            v.prepare(&self.meshes, &self.materials, &self.armatures)?;
            let _ = self.models.insert(v)?;
        }

        while let Some(v) = AsMut::<Vec<lbf::LBFLight>>::as_mut(&mut lbf).pop()
        {
            info!(
                "Light: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = self.lights.insert(Light::from_lbf(v, texture_size)?)?;
        }

        while let Some(v) = AsMut::<Vec<Camera>>::as_mut(&mut lbf).pop() {
            info!(
                "Camera: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = self.cameras.insert(v)?;
        }

        // Not use pop
        for v in AsRef::<Vec<lbf::LBFObject>>::as_ref(&lbf).into_iter() {
            info!(
                "Object: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = self.objects.insert(Object::from_lbf(
                v,
                &mut self.graph,
                &self.armatures,
                &self.models,
                &self.lights,
                &self.cameras,
            )?)?;
        }

        Ok(self)
    }
    // ========================================================================
    /// fn update
    pub fn update(&mut self) -> Result<&mut Self> {
        self.graph.update();
        for (ref mut _k, ref mut v) in self.objects.iter() {
            let _ = v.as_ref().borrow_mut().update()?;
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
