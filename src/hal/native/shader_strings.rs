pub static BACKING_FS: &str = r#"#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D screenTexture;

void main()
{
    vec3 col = texture(screenTexture, TexCoords).rgb;
    FragColor = vec4(col, 1.0);
}"#;

pub static BACKING_VS: &str = r#"#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoords;

out vec2 TexCoords;

void main()
{
    TexCoords = aTexCoords;
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
}
"#;

pub static CONSOLE_NO_BG_FS: &str = r#"#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;
in vec3 ourBackground;

// texture sampler
uniform sampler2D texture1;

void main()
{
    vec4 original = texture(texture1, TexCoord);
    if (original.r < 0.1f || original.g < 0.1f || original.b < 0.1f) discard;
    vec4 fg = original * vec4(ourColor, 1.f);
	FragColor = fg;
}
"#;

pub static CONSOLE_NO_BG_VS: &str = r#"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec3 bColor;
layout (location = 3) in vec2 aTexCoord;

out vec3 ourColor;
out vec3 ourBackground;
out vec2 TexCoord;

void main()
{
	gl_Position = vec4(aPos, 1.0);
	ourColor = aColor;
	ourBackground = bColor;
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}"#;

pub static CONSOLE_WITH_BG_FS : &str = r#"#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;
in vec3 ourBackground;

// texture sampler
uniform sampler2D texture1;

void main()
{
    vec4 original = texture(texture1, TexCoord);
    vec4 fg = original.r > 0.1f || original.g > 0.1f || original.b > 0.1f ? original * vec4(ourColor, 1.f) : vec4(ourBackground, 1.f);
	FragColor = fg;
}
"#;

pub static CONSOLE_WITH_BG_VS: &str = r#"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec3 bColor;
layout (location = 3) in vec2 aTexCoord;

out vec3 ourColor;
out vec3 ourBackground;
out vec2 TexCoord;

void main()
{
	gl_Position = vec4(aPos, 1.0);
	ourColor = aColor;
	ourBackground = bColor;
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}"#;

pub static SCANLINES_FS : &str = r#"#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D screenTexture;
uniform vec3 screenSize;
uniform bool screenBurn;

void main()
{
    vec3 col = texture(screenTexture, TexCoords).rgb;
    float scanLine = mod(gl_FragCoord.y, 2.0) * 0.25;
    vec3 scanColor = col.rgb - scanLine;

    if (col.r < 0.1f && col.g < 0.1f && col.b < 0.1f) {
        if (screenBurn) {
            float dist = (1.0 - distance(vec2(gl_FragCoord.x / screenSize.x, gl_FragCoord.y / screenSize.y), vec2(0.5,0.5))) * 0.2;
            FragColor = vec4(0.0, dist, dist, 1.0);
        } else {
            FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
    } else {
        FragColor = vec4(scanColor, 1.0);
    }
}"#;

pub static SCANLINES_VS: &str = r#"#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoords;

out vec2 TexCoords;

void main()
{
    TexCoords = aTexCoords;
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
}
"#;

pub static TEST_VS : &str = r#"#version 330 core
precision mediump float;
layout(location = 0) in vec2 vertexPosition_modelspace;
layout(location = 1) in vec2 aTex;

out vec2 TexCoords;
uniform vec3 offset;
uniform vec3 font;

void main(){
    gl_Position = vec4(vertexPosition_modelspace + floor(offset.rg * font.rg), 0.0, 1.0);
    TexCoords = aTex;
}
"#;

pub static TEST_FS : &str = r#"#version 330 core
precision mediump float;

in vec2 TexCoords;
out vec4 FragColor;
uniform sampler2D texture1;
uniform sampler2D glyphBuffer;
uniform sampler2D bgBuffer;
uniform vec3 font;
uniform vec3 screenSize;
uniform bool screenBurn;
uniform bool showScanLines;
uniform bool hasBackground;

void main(){
    float consoleX = gl_FragCoord.x / font.r;
    float consoleY = gl_FragCoord.y / font.g;
    float conX = floor(consoleX);
    float conY = floor(consoleY);

    // Obtain the 0..1 range within each character
    float spriteX = fract(consoleX);
    float spriteY = fract(consoleY);

    // Figure out where we are on the sprite sheet
    mediump float glyphSizeX = 1.0f / 16.0f;
    mediump float glyphSizeY = 1.0f / 16.0f;

    vec4 glyphLookup = texture(glyphBuffer, TexCoords);

    mediump float glyph = glyphLookup.r * 255.0f;
    mediump float glyphX = mod(glyph, 16.0f);
    mediump float glyphY = 16.0f - floor(glyph / 16.0f);
    // So now we have it in 0..16 , 0..16.

    vec3 rgb = glyphLookup.gba;

    vec2 fontcoords;
    fontcoords.r = (glyphX + spriteX) * glyphSizeX;
    fontcoords.g = (glyphY - (1.0f - spriteY)) * glyphSizeY;

    vec3 color = texture(texture1, fontcoords).rgb * rgb;

    // Set the background color
    bool applyScan = true;
    if (color.r < 0.1 && color.g < 0.1 && color.b < 0.1) {
        if (!hasBackground) discard;
        vec3 bg = texture(bgBuffer, TexCoords).rgb;
        if (bg.r < 0.1 && bg.g < 0.1 && bg.b < 0.1) {
            if (screenBurn) {
                float dist = (1.0 - distance(vec2(gl_FragCoord.x / screenSize.x, gl_FragCoord.y / screenSize.y), vec2(0.5,0.5))) * 0.2;
                color = vec3(0.0, dist, dist);
                applyScan = false;
            } else {
                color = vec3(0.0, 0.0, 0.0);
            }
        } else {
            color = bg;
        }
    }

    // Scan lines
    float scanLine = mod(gl_FragCoord.y, 2.0) * 0.25;
    vec3 scanColor = showScanLines ? (color.rgb - scanLine) : color;

    FragColor = applyScan ? vec4(scanColor.rgb, 1.0) : vec4(color, 1.0);
}
"#;