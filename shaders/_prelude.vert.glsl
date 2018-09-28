#version 300 es
precision mediump float;

in float feature;
out float v_feature;



#define PASS_FEATURE_IDX v_feature = feature;