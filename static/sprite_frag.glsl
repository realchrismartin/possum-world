#version 300 es
precision highp float;

in vec2 vertex_texture_coordinates;
in float vertex_texture_index;

out vec4 outColor;
uniform sampler2D u_texture_0;
uniform sampler2D u_texture_1;

void main() 
{
    if( int(vertex_texture_index) == 0)
    {
        outColor = texture(u_texture_0, vertex_texture_coordinates);
    } else
    {
        outColor = texture(u_texture_1, vertex_texture_coordinates);
    }
}