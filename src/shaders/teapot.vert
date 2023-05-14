#version 450

in vec3 position;
in vec3 normal;

uniform mat4 matrix;

void main() { gl_Position = vec4(position, 1.0) * matrix; }
