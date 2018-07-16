#version 300 es

// Only used on older version
//#define in attribute
//#define out varying
precision highp float;

in float feature;
out float v_feature;



#define PASS_FEATURE_IDX v_feature = feature;