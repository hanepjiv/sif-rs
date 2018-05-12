// -*- mode:glsl; coding:utf-8-unix; -*-
#line 2 1

// define  ////////////////////////////////////////////////////////////////////
// ============================================================================
#ifdef GL_ES
precision       mediump         float;
precision       mediump         int;
# define        EPSILON         (0.03125)                // sqrt(pow(2, -10));
#else
precision       highp           float;
precision       highp           int;
# define        EPSILON         (0.00034526698300124393) // sqrt(pow(2, -23));
#endif
// ============================================================================
#define         LIGHT_MAX_NUM   (16)
#define         BONE_MAX_NUM    (64)
#define         TEXTURE_MAX_NUM (16)
#define         GAMMA           (2.2)
// ============================================================================
#define FLAGS_IS(f, s)  (0 != int(mod(float(f) / pow(2.0, float(s)), 2.0)))
// function  //////////////////////////////////////////////////////////////////
// ============================================================================
//!
//! rand
//!
float rand(const in vec2 co){
  // return fract(sin(dot(co.xy ,vec2(12.9898, 78.233))) * 43758.5453);
  float a =
      fract(dot(co.xy, vec2(2.067390879775102, 12.451168662908249))) - 0.5;
  return fract(a * (6.182785114200511 + a*a *
                    (-38.026512460676566 + a*a * 53.392573080032137)) *
               43758.5453);
}
// ============================================================================
//!
//! cleanup
//!
void cleanup(inout mat4 l_Mat) {
  float m = -1.0 / 0.0;
  if (abs(l_Mat[0][0]) > m) { m = abs(l_Mat[0][0]); }
  if (abs(l_Mat[0][1]) > m) { m = abs(l_Mat[0][1]); }
  if (abs(l_Mat[0][2]) > m) { m = abs(l_Mat[0][2]); }
  if (abs(l_Mat[0][3]) > m) { m = abs(l_Mat[0][3]); }
  if (abs(l_Mat[1][0]) > m) { m = abs(l_Mat[1][0]); }
  if (abs(l_Mat[1][1]) > m) { m = abs(l_Mat[1][1]); }
  if (abs(l_Mat[1][2]) > m) { m = abs(l_Mat[1][2]); }
  if (abs(l_Mat[1][3]) > m) { m = abs(l_Mat[1][3]); }
  if (abs(l_Mat[2][0]) > m) { m = abs(l_Mat[2][0]); }
  if (abs(l_Mat[2][1]) > m) { m = abs(l_Mat[2][1]); }
  if (abs(l_Mat[2][2]) > m) { m = abs(l_Mat[2][2]); }
  if (abs(l_Mat[2][3]) > m) { m = abs(l_Mat[2][3]); }
  if (abs(l_Mat[3][0]) > m) { m = abs(l_Mat[3][0]); }
  if (abs(l_Mat[3][1]) > m) { m = abs(l_Mat[3][1]); }
  if (abs(l_Mat[3][2]) > m) { m = abs(l_Mat[3][2]); }
  if (abs(l_Mat[3][3]) > m) { m = abs(l_Mat[3][3]); }

  if (EPSILON > (abs(l_Mat[0][0]) / m)) { l_Mat[0][0] = 0.0; }
  if (EPSILON > (abs(l_Mat[0][1]) / m)) { l_Mat[0][1] = 0.0; }
  if (EPSILON > (abs(l_Mat[0][2]) / m)) { l_Mat[0][2] = 0.0; }
  if (EPSILON > (abs(l_Mat[0][3]) / m)) { l_Mat[0][3] = 0.0; }
  if (EPSILON > (abs(l_Mat[1][0]) / m)) { l_Mat[1][0] = 0.0; }
  if (EPSILON > (abs(l_Mat[1][1]) / m)) { l_Mat[1][1] = 0.0; }
  if (EPSILON > (abs(l_Mat[1][2]) / m)) { l_Mat[1][2] = 0.0; }
  if (EPSILON > (abs(l_Mat[1][3]) / m)) { l_Mat[1][3] = 0.0; }
  if (EPSILON > (abs(l_Mat[2][0]) / m)) { l_Mat[2][0] = 0.0; }
  if (EPSILON > (abs(l_Mat[2][1]) / m)) { l_Mat[2][1] = 0.0; }
  if (EPSILON > (abs(l_Mat[2][2]) / m)) { l_Mat[2][2] = 0.0; }
  if (EPSILON > (abs(l_Mat[2][3]) / m)) { l_Mat[2][3] = 0.0; }
  if (EPSILON > (abs(l_Mat[3][0]) / m)) { l_Mat[3][0] = 0.0; }
  if (EPSILON > (abs(l_Mat[3][1]) / m)) { l_Mat[3][1] = 0.0; }
  if (EPSILON > (abs(l_Mat[3][2]) / m)) { l_Mat[3][2] = 0.0; }
  if (EPSILON > (abs(l_Mat[3][3]) / m)) { l_Mat[3][3] = 0.0; }
}
// ============================================================================
//!
//! const in float a_v [0.0 - 1.0]
//!
vec4 float2rgba(in float v) {
  v *= 16581375.0;
  vec4 code = vec4(0.0);
  code.r = fract(v); v -= code.r; v /= 255.0;
  code.g = fract(v); v -= code.g; v /= 255.0;
  code.b = fract(v); v -= code.b; v /= 255.0;
  code.a = v;
  return code;
}
// ----------------------------------------------------------------------------
float rgba2float(const in vec4 v) {
  return dot(v, vec4(1.0 / 16581375.0, 1.0 / 65025.0, 1.0 / 255.0, 1.0));
}
// ============================================================================
//!
//! skinning
//!
//! return mat4
//!
mat4 skinning(const in mat4 a_Bones[BONE_MAX_NUM],
              const in vec4 a_BoneIdx, const in vec4 a_Weight) {
  mat4                           m  = a_Bones[int(a_BoneIdx[0])] * a_Weight[0];
  if (a_BoneIdx[1] >= 0.0) {     m += a_Bones[int(a_BoneIdx[1])] * a_Weight[1];
    if (a_BoneIdx[2] >= 0.0) {   m += a_Bones[int(a_BoneIdx[2])] * a_Weight[2];
      if (a_BoneIdx[3] >= 0.0) { m += a_Bones[int(a_BoneIdx[3])] * a_Weight[3];
      } } }
  return m;
}
