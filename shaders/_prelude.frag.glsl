#version 300 es
precision mediump float;


out vec4 frag_out;

in float v_feature;

#define PASS_FEATURE_IDX float feature = v_feature;