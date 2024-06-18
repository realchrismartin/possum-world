#version 300 es
 
layout(location = 0) in vec3 position;
layout(location = 1) in float model_matrix_index;
layout(location = 2) in vec2 texture_coordinates;
layout(location = 3) in float texture_index;

uniform mat4 vp_matrix;
uniform mat4 m_matrices[64];

out vec2 vertex_texture_coordinates;
out float vertex_texture_index;

void main() 
{
    gl_Position = m_matrices[int(model_matrix_index)] * vp_matrix * vec4(position,1.0);
    vertex_texture_coordinates = texture_coordinates;
    vertex_texture_index = texture_index;
}