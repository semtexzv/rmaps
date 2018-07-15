#version 300 es

#extension GL_EXT_texture_buffer : require
#extension GL_EXT_texture_buffer : enable

// Only used on older version
//#define in attribute
//#define out varying

precision highp float;

uniform samplerBuffer feature_data;

in float feature;
layout(location = 7) out float v_feature;

#define PASS_FEATUREE_IDX v_feature = feature;