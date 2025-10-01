#version 330 core

layout (location = 0) in vec3 position;

out vec3 tex_coord;

uniform mat4 view;
uniform mat4 perspective;


void main() {
    tex_coord = position;
    gl_Position = perspective * view * vec4(position, 1.0);
}

