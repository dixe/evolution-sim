#version 330 core


in VS_OUTPUT {
    vec2 FragPos;
    vec2 Pos;
} IN;

out vec4 FragColor;

uniform float width;
uniform float height;

uniform vec3 u_color;

float border(vec2 fragCoord, float gridWidth)
{
    if(fragCoord.x < gridWidth || fragCoord.x > (width - gridWidth) ||
    fragCoord.y < gridWidth  || fragCoord.y > (height - gridWidth ))
    {
        return 0.0;
    }
    return 1.0;
}


void main()
{

    // maybe look at https://www.shadertoy.com/view/WtdSDs

    // Square is defined with corners in 0.5 and -0.5 on both x and y axis.
    // add 0.5 to get range of 0..1
    float u = IN.FragPos.x + 0.5;
    float v = IN.FragPos.y + 0.5;

    vec2 fragCoord = vec2(u * width, v* height);


    vec3 col = u_color;
    float space_x = width / 9.;
    float space_y = height / 9.;
    float grid_width = 2.0;


    // TODO: don't draw on borders
    //float alpha = border(fragCoord, grid_width);
    FragColor = vec4(col, 1.0);
}
