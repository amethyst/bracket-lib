pub static CONSOLE_NO_BG_FS: &str = r#"#version 300 es
// Console No Background Fragment
precision mediump float;
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoords;
in vec3 ourBackground;

// texture sampler
uniform sampler2D texture1;

// Screen effects
uniform vec3 screenSize;
uniform bool screenBurn;
uniform bool showScanLines;

void main()
{
    vec3 col = texture(texture1, TexCoords).rgb;
    float scanLine = mod(gl_FragCoord.y, 2.0) * 0.25;
    vec3 scanColor = showScanLines ? (col.rgb - scanLine) * ourColor : ourColor;

    if (col.r < 0.1 && col.g < 0.1 && col.b < 0.1) discard;
    FragColor = vec4(scanColor, 1.0);
}
"#;

pub static CONSOLE_NO_BG_VS: &str = r#"#version 300 es
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec3 bColor;
layout (location = 3) in vec2 aTexCoord;

out vec3 ourColor;
out vec3 ourBackground;
out vec2 TexCoords;

void main()
{
	gl_Position = vec4(aPos, 1.0);
	ourColor = aColor;
	ourBackground = bColor;
	TexCoords = vec2(aTexCoord.x, aTexCoord.y);
}"#;

pub static CONSOLE_WITH_BG_FS : &str = r#"#version 300 es
precision mediump float;
in vec3 ourColor;
in vec3 ourBackground;
in vec2 TexCoords;

uniform vec3 screenSize;
uniform bool screenBurn;
uniform bool showScanLines;
uniform sampler2D texture1;

out vec4 FragColor;

void main()
{
    vec3 col = texture(texture1, TexCoords).rgb;
    float scanLine = mod(gl_FragCoord.y, 2.0) * 0.25;
    vec3 scanColor = showScanLines ? (col.rgb - scanLine) * ourColor : ourColor;

    if (col.r < 0.1 && col.g < 0.1 && col.b < 0.1) {
        if (screenBurn) {
            float dist = (1.0 - distance(vec2(gl_FragCoord.x / screenSize.x, gl_FragCoord.y / screenSize.y), vec2(0.5,0.5))) * 0.2;
            FragColor = vec4(0.0, dist, dist, 1.0);
        } else {
            FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
    } else {
        FragColor = vec4(scanColor, 1.0);
    }
}
"#;

pub static CONSOLE_WITH_BG_VS: &str = r#"#version 300 es
in vec3 aPos;
in vec3 aColor;
in vec3 bColor;
in vec2 aTexCoord;

out vec3 ourColor;
out vec3 ourBackground;
out vec2 TexCoords;

void main()
{
	gl_Position = vec4(aPos, 1.0);
	ourColor = aColor;
	ourBackground = bColor;
	TexCoords = vec2(aTexCoord.x, aTexCoord.y);
}"#;
