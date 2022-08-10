//#version version_number
#version 330 core

//in type in_variable_name;
//in type in_variable_name:
layout (location=0) in vec3 aPos;

out vec4 vertexColor;

//out type out_variable_name;

//uniform type uniform_name;

void main() {

//    process input(s) and do some weird graphics stuff

//    output processed stuff to output variable
//    out_variable_name = werid_stuff_we_processed;

    // Data Types
    float a;
    bool b;
    int c;
    uint d;
    // double not allowed on macos
//    double e;

    vec2 f32_vec_2;
    vec3 f32_vec_3;
    vec4 f32_vec_4;

    bvec2 bool_vec_2;
    bvec3 bool_vec_3;
    bvec4 bool_vec_4;

    ivec2 int_vec_2;
    ivec3 int_vec_3;
    ivec4 int_vec_4;

    uvec2 uint_vec_2;
    uvec3 uint_vec_3;
    uvec4 uint_vec_4;

    // error: illegal use of reserved word `dvecn'
//    dvec2 f64_vec_2;
//    dvec3 f64_vec_3;
//    dvec4 f64_vec_4;

    vec3 tmp1 = vec3(0, 0, 0);
    tmp1.x = 1;
    tmp1.y = 1;
    tmp1.z = 1;

    vec3 tmp2 = tmp1.zyy;
    vec3 tmp3 = vec3(tmp2.xy, 1);

    gl_Position = vec4(aPos, 1.0);
    vertexColor = vec4(0.5, 0.0, 0.0, 1.0);
}
