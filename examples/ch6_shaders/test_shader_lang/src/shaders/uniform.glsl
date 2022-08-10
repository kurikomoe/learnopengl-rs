#version 330 core

out vec4 fragColor;

// Uniform is some kind of global varialbe shared across the shaders
uniform vec4 ourColor;  // We set this variable in the OpenGL code.

void main() {
    fragColor = ourColor;
//    fragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
