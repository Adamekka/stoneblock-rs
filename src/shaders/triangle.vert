#version 450

in vec2 position;
in vec3 color;

out vec3 vColor;

uniform mat4 matrix;

void main() {
  gl_Position = matrix * vec4(position, 0.0, 1.0);
  vColor = color;
}
