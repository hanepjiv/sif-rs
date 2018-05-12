// -*- mode:glsl; coding:utf-8-unix; -*-
#line 2 4

// function  //////////////////////////////////////////////////////////////////
// ============================================================================
//!
//! Bayer threshold
//!
float bayer_threshold(const in vec2 a_FragCoord) {
  vec2 l_FragCoord       = a_FragCoord;
  l_FragCoord           -= 0.5;
  l_FragCoord           +=
      gl_FrontFacing ? u_AlphaDitherShift.xy : u_AlphaDitherShift.yx;

  int   idx              = int(mod(l_FragCoord.y, 16.0));
  idx                   *= 16;
  idx                   += int(mod(l_FragCoord.x, 16.0));

  float   x              = mod(float(idx / 4), 8.0);
  x                     /= 8.0;

  float   y              = float(idx / 4 / 8);
  y                     /= 8.0;

  float  threshold       = texture2D(u_Texture_BayerMatrix,
                                     vec2(x, y))[int(mod(float(idx), 4.0))];
  threshold             *= 0.996078431372549;   // 1.0 / 256.0 * 255.0
  threshold             += 0.00196078431372549; // 1.0 / 256.0 / 2.0

  return threshold;
}
// ============================================================================
//!
//! Parallax Occlusion Mapping
//!
vec3 makeParallax(const in      sampler2D       a_HeightMap,
                  const in      vec2            a_Coord,
                  const in      float           a_Height,
                  const in      int             a_Loop,
                  in            vec3            l_View) {
  l_View                 /= abs(l_View.z);
  l_View.xy              *= a_Height;

  int   l_Signe           = 1;
  float l_C               = 0.5;

  vec3  l_Parallax        = l_View * float(l_Signe) * l_C;

  for (int i = 0; i < a_Loop; ++i) {
    float h               = texture2D(a_HeightMap, a_Coord + l_Parallax.xy).a;
    h                    += l_Parallax.z;

    if (EPSILON > abs(h)) { break; }

#if 0
    if (0.0 < h) {
      if (0 > l_Signe) { l_Signe =  1; l_C *= 0.5; }
    } else {
      if (0 < l_Signe) { l_Signe = -1; l_C *= 0.5; }
    }
#else
    int l_NewSigne        =  (0.0 < h) ? 1 : 0; //  l_NewSigne   =  (1 :  0);
    l_NewSigne           *=  2;                 //  l_NewSigne   =  (2 :  0);
    l_NewSigne           -=  1;                 //  l_NewSigne   =  (1 : -1);

    l_Signe              *=  l_NewSigne;        //  l_Signe      =  (1 : -1);
    l_Signe              +=  1;                 //  l_Signe      =  (2 :  0);
    l_Signe              /=  2;                 //  l_Signe      =  (1 :  0);
    l_Signe              +=  1;                 //  l_Signe      =  (2 :  1);

    l_C                  *=  float(l_Signe);    //
    l_C                  /=  2.0;               //  l_C         *= (1.0 : 0.5);

    l_Signe               =  l_NewSigne;
#endif

    l_Parallax           +=  l_View * float(l_Signe) * l_C;
  }

  l_Parallax.z           *=  a_Height;

  return l_Parallax;
}
// ============================================================================
//!
//! Parallax Occlusion Mapping - Soft Shadow -
//!
float makeParallaxSoftShadow(const in   sampler2D       a_HeightMap,
                             const in   vec2            a_Coord,
                             const in   vec3            a_Parallax,
                             in         int             l_ShadowLoop,
                             in         vec3            l_LightPos) {
  l_ShadowLoop            =  int(max(2.0, float(l_ShadowLoop)));

  l_LightPos             /=  abs(l_LightPos.z);
  l_LightPos.xy          *=  abs(a_Parallax.z);
  l_LightPos             /=  float(l_ShadowLoop);

  l_ShadowLoop           -=  1;

  vec3  l_Offset;
  float l_Factor          =  0.0;

  for (int i = 0; i < l_ShadowLoop; ++i) {
    l_Offset              =  l_LightPos;
    l_Offset             *=  float(i);
    l_Factor             +=  texture2D(a_HeightMap, a_Coord + l_Offset.xy).a;
    l_Factor             -=  l_Offset.z;
  }

  l_Factor               /=  float(l_ShadowLoop);
  l_Factor                =  clamp(l_Factor, 0.0, 1.0);
  l_Factor               *= -1.0;
  l_Factor               +=  1.0;

  return pow(l_Factor, u_Material.parallax.shadow_exponent);
}
// ============================================================================
//!
//! tangentSpace
//!
vec3 tangentSpace(in vec3 a_In, in vec3 a_Tan, in vec3 a_Bin, in vec3 a_Nor) {
  return vec3(dot(a_Tan, a_In), dot(a_Bin, a_In), dot(a_Nor, a_In));
}
// ----------------------------------------------------------------------------
vec3 tangentSpaceInverse(in vec3 a_In,
                         in vec3 a_Tan, in vec3 a_Bin, in vec3 a_Nor) {
  return vec3(a_Tan.x * a_In.x + a_Bin.x * a_In.y + a_Nor.x * a_In.z,
              a_Tan.y * a_In.x + a_Bin.y * a_In.y + a_Nor.y * a_In.z,
              a_Tan.z * a_In.x + a_Bin.z * a_In.y + a_Nor.z * a_In.z);
}
// ============================================================================
vec4 fetchShadowMap(const in int i, const in vec2 c) {
  if      ( 0 == i) { return texture2D(u_Texture_ShadowMap_00, c); }
  else if ( 1 == i) { return texture2D(u_Texture_ShadowMap_01, c); }
  else if ( 2 == i) { return texture2D(u_Texture_ShadowMap_02, c); }
  else if ( 3 == i) { return texture2D(u_Texture_ShadowMap_03, c); }
  else if ( 4 == i) { return texture2D(u_Texture_ShadowMap_04, c); }
  else if ( 5 == i) { return texture2D(u_Texture_ShadowMap_05, c); }
  else if ( 6 == i) { return texture2D(u_Texture_ShadowMap_06, c); }
  else if ( 7 == i) { return texture2D(u_Texture_ShadowMap_07, c); }
  /*
    else if ( 8 == i) { return texture2D(u_Texture_ShadowMap_08, c); }
    else if ( 9 == i) { return texture2D(u_Texture_ShadowMap_09, c); }
    else if (10 == i) { return texture2D(u_Texture_ShadowMap_10, c); }
    else if (11 == i) { return texture2D(u_Texture_ShadowMap_11, c); }
    else if (12 == i) { return texture2D(u_Texture_ShadowMap_12, c); }
    else if (13 == i) { return texture2D(u_Texture_ShadowMap_13, c); }
    else if (14 == i) { return texture2D(u_Texture_ShadowMap_14, c); }
    else if (15 == i) { return texture2D(u_Texture_ShadowMap_14, c); }
  */
  else              { return float2rgba(1.0); }
}
// ----------------------------------------------------------------------------
//!
//! shadowDepthFactor
//! return [0.0 - 1.0]
//!     0.0:   out of shadow volume / use light
//!     1.0:  inside shadow volume / not use light
//!
float shadowDepthFactor(const in vec3 a_WorldPos, const in int i,
                        const in float a_LightDot) {
  if (!u_Lights[i].is_shadow && gl_FrontFacing) { return 1.0; }

  vec4 pos        = u_Lights[i].Mat4_ProjLight * vec4(a_WorldPos, 1.0);
  float depth     = ((pos.w - u_Lights[i].shadow_near) /
                     (u_Lights[i].shadow_far - u_Lights[i].shadow_near));
  vec2 coord      = pos.xy;
  coord          /= pos.w;
  coord          *= 0.5;
  coord          += 0.5;
  float bias      = min(0.05 * (1.0 - a_LightDot), 0.005);
  if ((depth - bias) <= rgba2float(fetchShadowMap(i, coord))) {
    return 1.0;
  } else {
    return 0.382;
  }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
void main(void) {
  /*
  if (gl_FrontFacing || !gl_FrontFacing) {
    // gl_FragData[0] = vec4(vf_Normal, 1.0);
    gl_FragData[0] = texture2D(u_Texture_Diffuse, vf_Coord);
    return;
  }
  */

  vec3  l_Pos             = vf_Position;
  vec3  l_View            = normalize(l_Pos);
  vec2  l_Coo             = vf_Coord;
  vec3  l_Tan             = normalize(vf_Tangent);
  vec3  l_Bin             = normalize(vf_Binormal);
  vec3  l_Nor             = normalize(vf_Normal);

  if (!gl_FrontFacing) {
    l_Bin                *= -1.0;
    l_Nor                *= -1.0;
  }

  vec3  l_WorldPos        = vf_WorldPos;

  vec3  l_Normal          = l_Nor;
  vec3  l_Parallax        = vec3(0.0);

  if (u_Material.is_map_normal && u_Material.is_bump) {
    l_Pos                 = tangentSpace(l_Pos, l_Tan, l_Bin, l_Nor);
    l_View                = normalize(l_Pos);
    if (u_Material.is_parallax) {
      l_Parallax          = makeParallax(u_Texture_Normal, l_Coo,
                                         max(0.0, u_Material.parallax.height),
                                         u_Material.parallax.loop,
                                         l_View);
      l_Pos              += l_Parallax;
      l_View              = normalize(l_Pos);
      l_Coo              += l_Parallax.xy;
    }
    l_Normal              = texture2D(u_Texture_Normal, l_Coo).rgb;
    l_Normal             *= 2.0;
    l_Normal             -= 1.0;
  }

  float l_Alpha           = 1.0; min(pow(-l_Pos.z * 0.1, 2.0), 1.0);
  vec4  l_DifMap          = vec4(vec3(1.0), l_Alpha * u_Material.alpha);
  if (u_Material.is_map_diffuse) {
    l_DifMap              = texture2D(u_Texture_Diffuse, l_Coo);
    l_DifMap.a           *= l_Alpha * u_Material.alpha;
  }

  float l_BayerThreshold  = bayer_threshold(gl_FragCoord.xy);
  if (l_DifMap.a < l_BayerThreshold) {
    discard;
  }

  vec3  l_Dif             = vec3(0.0);
  if (u_Ambient.is_enable) {  // ambient
    l_Dif                += u_Ambient.color;
  }

  vec3  l_Spe             = vec3(0.0);

  for (int i = 0; i < LIGHT_MAX_NUM; ++i) {
    if (!u_Lights[i].is_enable) { continue; }
    if (EPSILON > dot(u_Lights[i].color, u_Lights[i].color)) { continue; }
    // ------------------------------------------------------------------------
    float l_LightFactor   = 1.0;
    vec3  l_LightDir      = u_Lights[i].direction;
    if (u_Material.is_map_normal && u_Material.is_bump) {
      l_LightDir          = normalize(tangentSpace(l_LightDir,
                                                   l_Tan, l_Bin, l_Nor));
    }
    // ------------------------------------------------------------------------
    vec3  l_LightPos;
    float l_LightDot;
    // ========================================================================
    // ------------------------------------------------------------------------
    if (u_Lights[i].is_point) {
      // POINT or SPOT
      l_LightPos          = u_Lights[i].position;
      if (u_Material.is_map_normal && u_Material.is_bump) {
        l_LightPos        = tangentSpace(l_LightPos, l_Tan, l_Bin, l_Nor);
      }
      l_LightPos         -= l_Pos;
      float l_LightLen    = length(l_LightPos);
      l_LightPos         /= l_LightLen;
      l_LightDot          = dot(l_LightPos, l_Normal);
      if (0.0 > l_LightDot) { continue; }

      if (u_Lights[i].cutoff > -1.0) {
        // SPOT
        l_LightFactor    *= dot(l_LightDir, -l_LightPos);
        if (u_Lights[i].cutoff > l_LightFactor) { continue; }
        l_LightFactor     = pow(l_LightFactor, u_Lights[i].exponent);
        if (EPSILON > l_LightFactor) { continue; }
      }

      l_LightFactor      *= shadowDepthFactor(l_WorldPos, i, l_LightDot);
      if (EPSILON > l_LightFactor) { continue; }

      l_LightFactor      /=
          (u_Lights[i].kcklkq.x +
           ((u_Lights[i].kcklkq.y +
             (u_Lights[i].kcklkq.z * l_LightLen)) * l_LightLen));
      if (EPSILON > l_LightFactor) { continue; }
    } else {
      // SUN
      l_LightPos          = -l_LightDir;
      l_LightDot          = dot(l_LightPos, l_Normal);
      if (0.0 > l_LightDot) { continue; }
      l_LightFactor      *= shadowDepthFactor(l_WorldPos, i, l_LightDot);
      if (EPSILON > l_LightFactor) { continue; }
    }

    if (u_Material.is_map_normal &&
        u_Material.is_bump && u_Material.is_parallax) {
      l_LightFactor      *=
          makeParallaxSoftShadow(u_Texture_Normal, l_Coo, l_Parallax,
                                 u_Material.parallax.shadow_loop,
                                 l_LightPos);
      if (EPSILON > l_LightFactor) { continue; }
    }

    vec3  l_Int           = u_Lights[i].color;
    l_Int                *= l_LightFactor;

    l_Dif                += l_Int * l_LightDot;

    vec3 l_Half           = normalize(l_LightPos - l_View);
    float l_Shininess     = u_Material.shininess;
    if (u_Material.is_anisotropic) {
      l_Shininess        *= pow(l_Half.y, 20.0);  // TODO(hanepjiv): exponent
    }
    l_Spe                +=
        l_Int * pow(max(0.0, dot(l_Half, l_Normal)), l_Shininess);
  }

  {  // diffuse
    l_Dif                *= u_Material.diffuse;
    l_Dif                *= l_DifMap.rgb;
    gl_FragData[0].rgb    = l_Dif;
    gl_FragData[0].a      = l_DifMap.a;
  }

  {  // speculer
    l_Spe                *= u_Material.specular;
    if  (EPSILON < dot(l_Spe, l_Spe)) {
      if (u_Material.is_map_specular) {
        vec4  l_SpeMap    = texture2D(u_Texture_Speculer, l_Coo);
        l_Spe            *= l_SpeMap.rgb;
        l_Spe            *= l_SpeMap.a;
      }
      gl_FragData[0].rgb += l_Spe;
    }
  }

  {  // emissive
    vec3  l_Emi           = u_Material.emissive;
    if  (EPSILON < dot(l_Emi, l_Emi)) {
      if (u_Material.is_map_emissive) {
        vec4  l_EmiMap    = texture2D(u_Texture_Emissive, l_Coo);
        l_Emi            *= l_EmiMap.rgb;
        l_Emi            *= l_EmiMap.a;
      }
      gl_FragData[0].rgb += l_Emi;
    }
  }

  {  // edge
    float l_EdgeFactor    = dot(-l_View, l_Normal);
    if (0.0 < u_Ink.exponent) {  // ink
      vec3 l_Ink          = u_Ink.color;
      l_Ink              *= pow(max( 0.0, 1.0 - l_EdgeFactor), u_Ink.exponent);
      gl_FragData[0].rgb -= l_Ink;
    }
    if (0.0 < u_Rim.exponent) {  // rim
      vec3 l_Rim          = u_Rim.color;
      l_Rim              *= pow(max(0.0, 1.0 - l_EdgeFactor), u_Rim.exponent);
      gl_FragData[0].rgb += l_Rim;
    }
  }

  /*
    float threshold       = bayer_threshold(gl_FragCoord.xy);
    gl_FragData[0].rgb    =
    (length(gl_FragData[0].rgb) < threshold) ? vec3(0.0) : vec3(1.0);
  */

  // gl_PointCoord * gl_FragCoord.w

  // gamma  ===================================================================
  // see. OpenGL4.0 Shading Language (ISBN978-4-86246-189-6 Japan) P.155
  /*
    gl_FragData[0]        = vec4(pow(gl_FragData[0].rgb, vec3(1.0 / GAMMA)),
    gl_FragData[0].a);
  */
}
