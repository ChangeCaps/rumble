#version 450

in vec2 position;
in vec2 tex_coord;
out vec2 vtex_coord;

void main() {
	gl_Position = vec4(position, 0.0, 1.0);
	vtex_coord = tex_coord;
}
