#version 450

in vec2 vtex_coord;
out vec4 color;

uniform sampler2D map_texture;

void main() {
	vec2 size = textureSize(map_texture, 0);

	color = texelFetch(map_texture, ivec2(round(vtex_coord * size)), 0);
}
