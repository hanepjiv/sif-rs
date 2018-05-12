// -*- mode:glsl; coding:utf-8-unix; -*-
#line 2 2

// struct  ////////////////////////////////////////////////////////////////////
// ============================================================================
struct ColorExponent {
  vec3                          color;
  float                         exponent;               // <= 0.0: disable
};
// ============================================================================
struct ColorEnable {
  vec3                          color;
  bool                          is_enable;
};
// ============================================================================
struct Light {
  vec3                          position;
  bool                          is_enable;
  vec3                          direction;
  bool                          is_point;
  vec3                          color;
  float                         exponent;
  vec3                          kcklkq;
  float                         cutoff;                 // <= -1.0: disable
  mat4                          Mat4_ProjLight;
  bool                          is_shadow;
  float                         shadow_near;
  float                         shadow_far;
};
// ============================================================================
struct Parallax {
  float                         height;
  float                         shadow_exponent;
  int                           loop;
  int                           shadow_loop;
};
// ----------------------------------------------------------------------------
struct Material {
  Parallax                      parallax;
  vec3                          diffuse;
  float                         alpha;
  vec3                          specular;
  float                         shininess;
  vec3                          emissive;
  bool                          is_map_diffuse;
  bool                          is_map_specular;
  bool                          is_map_normal;
  bool                          is_map_emissive;
  bool                          is_anisotropic;
  bool                          is_bump;
  bool                          is_parallax;
};
// uniform  ///////////////////////////////////////////////////////////////////
// ============================================================================
uniform         bool            u_Skinning;
// ============================================================================
uniform         mat4            u_Mat4_Proj;
uniform         mat4            u_Mat4_Model;
uniform         mat4            u_Mat4_ViewModel;
// ============================================================================
uniform         Material        u_Material;
// ============================================================================
uniform         mat4            u_Bones[BONE_MAX_NUM];
uniform         float           u_PointSize;
// ============================================================================
uniform         ColorEnable     u_Ambient;
uniform         ColorExponent   u_Fog;
uniform         ColorExponent   u_Ink;
uniform         ColorExponent   u_Rim;
uniform         Light           u_Lights[LIGHT_MAX_NUM];
uniform         vec2            u_AlphaDitherShift;
// ============================================================================
uniform         sampler2D       u_Texture_Diffuse;      // 00;
uniform         sampler2D       u_Texture_Speculer;     // 01;
uniform         sampler2D       u_Texture_Normal;       // 02;
uniform         sampler2D       u_Texture_Emissive;     // 03;
uniform         sampler2D       u_Texture_BayerMatrix;  // 04;
uniform         sampler2D       u_Texture_05;           // 05
uniform         sampler2D       u_Texture_06;           // 06
uniform         sampler2D       u_Texture_07;           // 07
uniform         sampler2D       u_Texture_08;           // 08
uniform         sampler2D       u_Texture_09;           // 09
uniform         sampler2D       u_Texture_10;           // 10
uniform         sampler2D       u_Texture_11;           // 11
uniform         sampler2D       u_Texture_12;           // 12
uniform         sampler2D       u_Texture_13;           // 13
uniform         sampler2D       u_Texture_14;           // 14
uniform         sampler2D       u_Texture_15;           // 15
uniform         sampler2D       u_Texture_ShadowMap_00; // 16
uniform         sampler2D       u_Texture_ShadowMap_01; // 17
uniform         sampler2D       u_Texture_ShadowMap_02; // 18
uniform         sampler2D       u_Texture_ShadowMap_03; // 19
uniform         sampler2D       u_Texture_ShadowMap_04; // 20
uniform         sampler2D       u_Texture_ShadowMap_05; // 21
uniform         sampler2D       u_Texture_ShadowMap_06; // 22
uniform         sampler2D       u_Texture_ShadowMap_07; // 23
uniform         sampler2D       u_Texture_ShadowMap_08; // 24
uniform         sampler2D       u_Texture_ShadowMap_09; // 25
uniform         sampler2D       u_Texture_ShadowMap_10; // 26
uniform         sampler2D       u_Texture_ShadowMap_11; // 27
uniform         sampler2D       u_Texture_ShadowMap_12; // 28
uniform         sampler2D       u_Texture_ShadowMap_13; // 29
uniform         sampler2D       u_Texture_ShadowMap_14; // 30
uniform         sampler2D       u_Texture_ShadowMap_15; // 31
// varying  ///////////////////////////////////////////////////////////////////
// ============================================================================
varying         vec3            vf_Position;            // view
varying         vec2            vf_Coord;               // view
varying         vec3            vf_Normal;              // view
varying         vec3            vf_Tangent;             // view
varying         vec3            vf_Binormal;            // view
varying         vec3            vf_WorldPos;            // world
