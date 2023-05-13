#version 450

in vec2 position;
in vec3 color;

out vec3 vColor;

uniform float step;

void main() {
  vec2 pos = position;
  pos.x += step;
  gl_Position = vec4(pos, 0.0, 1.0);
  vColor = color;
}
