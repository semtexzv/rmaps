#version 300 es
//#define in varying
precision highp float;


out vec4 frag_out;

in float v_feature;

#define PASS_FEATURE_IDX float feature = v_feature;