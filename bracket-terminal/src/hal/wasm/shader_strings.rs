pub static UBERSHADER_VS: &str = r#"#version 300 es
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

pub static UBERSHADER_FS: &str = r#"#version 300 es
precision mediump float;

in vec2 TexCoords;
out vec4 FragColor;
uniform sampler2D texture1;
uniform sampler2D glyphBuffer;
uniform sampler2D bgBuffer;
uniform sampler2D fgBuffer;
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

    vec4 rgb = texture(fgBuffer, TexCoords);

    vec2 fontcoords;
    fontcoords.r = (glyphX + spriteX) * glyphSizeX;
    fontcoords.g = (glyphY - (1.0f - spriteY)) * glyphSizeY;

    vec4 raw_color = texture(texture1, fontcoords);
    vec4 color = raw_color * rgb;
    vec4 raw_bg = texture(bgBuffer, TexCoords);
    vec4 bg = raw_bg;

    // Set the background color
    bool applyScan = true;
    if (raw_color.r < 0.1 && raw_color.g < 0.1 && raw_color.b < 0.1 || raw_color.a < 0.1) {
        if (!hasBackground) discard;
        if (bg.r < 0.1 && bg.g < 0.1 && bg.b < 0.1 || bg.a < 0.1) {
            if (screenBurn) {
                float dist = (1.0 - distance(vec2(gl_FragCoord.x / screenSize.x, gl_FragCoord.y / screenSize.y), vec2(0.5,0.5))) * 0.2;
                color = vec4(0.0, dist, dist, 1.0);
                applyScan = false;
            } else {
                if (raw_bg.a < 0.1) discard;
                color = vec4(0.0, 0.0, 0.0, 1.0);
            }
        } else {
            color = bg;
        }
    }

    // Scan lines
    float scanLine = mod(gl_FragCoord.y, 2.0) * 0.25;
    vec4 scanColor = showScanLines ? (color - scanLine) : color;

    FragColor = applyScan ? scanColor : color;
}
"#;
