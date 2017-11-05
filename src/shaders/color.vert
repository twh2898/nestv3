#version 140

in vec2 position;
in vec4 color;
in vec2 uvw;

out vec4 v_color;
out vec2 v_uvw;

uniform mat4 perspective;

void main() {
	gl_Position = perspective * vec4(position, 0.0, 1.0);
	v_color = color;
	v_uvw = uvw;
}
