#version 450

in vec2 position;
in vec2 tex_coords;
// in vec3 color;

// out vec3 vColor;
out vec2 v_tex_coords;

uniform mat4 matrix;

void main() {
  gl_Position = vec4(position, 0.0, 1.0) * matrix;
  v_tex_coords = tex_coords;
  // vColor = color;
}
