#version 450

// in vec3 vColor;
// out vec4 fColor;

// void main() { fColor = vec4(vColor, 1.0); }

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() { color = texture(tex, v_tex_coords); }
