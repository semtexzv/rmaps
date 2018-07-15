#version 300 es

#extension GL_EXT_texture_buffer : require
#extension GL_EXT_texture_buffer : enable


//#define in varying

precision highp float;
uniform samplerBuffer feature_data;

layout(location = 7) in float feature;