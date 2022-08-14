#version 330 core

layout (location=0) in vec3 aPos;
layout (location=1) in vec3 aColor;

out vec3 ourColor;

void main() {
    vec3 bPos = vec3(aPos.x, -aPos.y, aPos.z);
    gl_Position = vec4(bPos, 1.0);
    ourColor = aColor;
}
