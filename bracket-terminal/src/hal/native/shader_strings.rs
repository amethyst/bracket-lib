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

in vec4 ourColor;
in vec2 TexCoord;
in vec4 ourBackground;

// texture sampler
uniform sampler2D texture1;

void main()
{
    vec4 original = texture(texture1, TexCoord);
    if (original.r < 0.1f || original.g < 0.1f || original.b < 0.1f) discard;
    vec4 fg = original * ourColor;
	FragColor = fg;
}
"#;

pub static CONSOLE_NO_BG_VS: &str = r#"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec4 bColor;
layout (location = 3) in vec2 aTexCoord;

out vec4 ourColor;
out vec4 ourBackground;
out vec2 TexCoord;

void main()
{
	gl_Position = vec4(aPos, 1.0);
	ourColor = aColor;
	ourBackground = bColor;
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}"#;

pub static CONSOLE_WITH_BG_FS: &str = r#"#version 330 core
out vec4 FragColor;

in vec4 ourColor;
in vec2 TexCoord;
in vec4 ourBackground;

// texture sampler
uniform sampler2D texture1;

void main()
{
    vec4 original = texture(texture1, TexCoord);
    vec4 fg = (original.r > 0.1f || original.g > 0.1f || original.b > 0.1f) && original.a > 0.1f ? original * ourColor : ourBackground;
	FragColor = fg;
}
"#;

pub static CONSOLE_WITH_BG_VS: &str = r#"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec4 bColor;
layout (location = 3) in vec2 aTexCoord;

out vec4 ourColor;
out vec4 ourBackground;
out vec2 TexCoord;

void main()
{
	gl_Position = vec4(aPos, 1.0);
	ourColor = aColor;
	ourBackground = bColor;
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}"#;

pub static SCANLINES_FS: &str = r#"#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D screenTexture;
uniform vec3 screenSize;
uniform bool screenBurn;
uniform vec3 screenBurnColor;

void main()
{
    vec3 col = texture(screenTexture, TexCoords).rgb;
    float scanLine = mod(gl_FragCoord.y, 2.0) * 0.25;
    vec3 scanColor = col.rgb - scanLine;

    if (col.r < 0.1f && col.g < 0.1f && col.b < 0.1f) {
        if (screenBurn) {
            float dist = (1.0 - distance(vec2(gl_FragCoord.x / screenSize.x, gl_FragCoord.y / screenSize.y), vec2(0.5,0.5))) * 0.2;
            FragColor = vec4(dist * screenBurnColor.r, dist * screenBurnColor.g, dist * screenBurnColor.b, 1.0);
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

pub static FANCY_CONSOLE_FS: &str = r#"#version 330 core
out vec4 FragColor;

in vec4 ourColor;
in vec2 TexCoord;
in vec4 ourBackground;

// texture sampler
uniform sampler2D texture1;

void main()
{
    vec4 original = texture(texture1, TexCoord);
    vec4 fg = (original.r > 0.1f || original.g > 0.1f || original.b > 0.1f) && original.a > 0.1f ? original * ourColor : ourBackground;
	FragColor = fg;
}
"#;

pub static FANCY_CONSOLE_VS: &str = r#"#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec4 bColor;
layout (location = 3) in vec2 aTexCoord;
layout (location = 4) in vec3 aRotate; // Angle, base X, base Y
layout (location = 5) in vec2 aScale;

out vec4 ourColor;
out vec4 ourBackground;
out vec2 TexCoord;

mat2 r2d(float a) {
	float c = cos(a), s = sin(a);
    return mat2(
        c, s,
        -s, c
    );
}

void main()
{
    float rot = aRotate.x;
    vec2 center_pos = aRotate.yz;
    vec2 base_pos = aPos.xy - center_pos;
    base_pos *= r2d(rot);
    base_pos *= aScale;
    base_pos += center_pos;

	gl_Position = vec4(base_pos, 0.0, 1.0);
	ourColor = aColor;
	ourBackground = bColor;
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}"#;

pub static SPRITE_CONSOLE_FS: &str = r#"#version 330 core
out vec4 FragColor;

in vec4 ourColor;
in vec2 TexCoord;

// texture sampler
uniform sampler2D texture1;

void main()
{
    vec4 original = texture(texture1, TexCoord);
    vec4 fg = original * ourColor;
	FragColor = fg;
}
"#;

pub static SPRITE_CONSOLE_VS: &str = r#"#version 330 core
layout (location = 0) in vec2 aRelativePos;
layout (location = 1) in vec2 aTransform;
layout (location = 2) in vec4 aColor;
layout (location = 3) in vec2 aTexCoord;
layout (location = 4) in vec2 aScale;

out vec4 ourColor;
out vec2 TexCoord;

void main()
{
    vec2 base_pos = aRelativePos;
    vec2 scaled = base_pos * aScale;
    vec2 translated = scaled + aTransform.xy;

	gl_Position = vec4(translated, 1.0, 1.0);
	ourColor = aColor;
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}"#;
