// -*- mode:glsl; coding:utf-8-unix; -*-
#line 2 3

// attribute  /////////////////////////////////////////////////////////////////
// ============================================================================
attribute       vec3            iv_Position;
attribute       vec3            iv_Normal;
attribute       vec2            iv_Coord;
attribute       vec4            iv_Tangent;
attribute       vec4            iv_BoneIdx;
attribute       vec4            iv_Weight;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
void main(void) {
  vf_Position     = iv_Position;
  vf_Coord        = iv_Coord;
  vf_Normal       = iv_Normal;
  vf_Tangent      = iv_Tangent.xyz;
  vf_WorldPos     = iv_Position;

  if (u_Skinning && iv_BoneIdx[0] >= 0.0) {
    mat4 m        = skinning(u_Bones, iv_BoneIdx, iv_Weight);
    vf_Position   = (m * vec4(vf_Position, 1.0)).xyz;
    vf_WorldPos   = vf_Position;
    vf_Normal     = normalize((m * vec4(vf_Normal, 0.0)).xyz);
    if (u_Material.is_map_normal) {
      vf_Tangent  = normalize((m * vec4(iv_Tangent.xyz, 0.0)).xyz);
    }
  }
  vf_WorldPos     = (u_Mat4_Model * vec4(vf_Position, 1.0)).xyz;
  vf_Position     = (u_Mat4_ViewModel * vec4(vf_Position, 1.0)).xyz;
  vf_Normal       = normalize((u_Mat4_ViewModel * vec4(vf_Normal, 0.0)).xyz);

  if (u_Material.is_map_normal) {
    vf_Tangent    = normalize((u_Mat4_ViewModel * vec4(vf_Tangent, 0.0)).xyz);
    vf_Binormal   = normalize(cross(vf_Normal, vf_Tangent)) * iv_Tangent.w;
  }

  gl_Position     = u_Mat4_Proj * vec4(vf_Position, 1.0);
  gl_PointSize    = u_PointSize;
}
