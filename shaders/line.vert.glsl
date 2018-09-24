uniform mat4 u_p_matrix;
uniform mat4 u_mv_matrix;


in vec2 pos;
in vec2 normal;

out vec2 v_normal;

#pragma property : define vec4 color
#pragma property : define float opacity
#pragma property : define float width
#pragma property : define float gap_width

void main() {
    PASS_FEATURE_IDX;

    #pragma property : init vec4 color
    #pragma property : init float opacity
    #pragma property : init float width
    #pragma property : init float gap_width

    float actual_width = width + gap_width;

    // Invert y component, view matrix inverts this component in positions
    vec2 tangent = normal * vec2(actual_width, -actual_width) / 2048.;

    vec4 world_pos = u_mv_matrix *  vec4(pos + tangent,0.,1.);
    vec4 extruded = world_pos - vec4(tangent, 0., 0.);

    gl_Position = u_p_matrix * extruded;
}