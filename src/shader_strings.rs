#[cfg(not(target_arch = "wasm32"))]
pub static BACKING_FS: &str = r#"#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D screenTexture;

void main()
{
    vec3 col = texture(screenTexture, TexCoords).rgb;
    FragColor = vec4(col, 1.0);
}"#;

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
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

/////////////////////////// WASM WebGL2 Versions

#[cfg(target_arch = "wasm32")]
pub static BACKING_FS: &str = r#"#version 300 es
// Backing FS
precision highp float;
out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D screenTexture;

void main()
{
    vec3 col = texture(screenTexture, TexCoords).rgb;
    FragColor = vec4(col, 1.0);
}"#;

#[cfg(target_arch = "wasm32")]
pub static BACKING_VS: &str = r#"#version 300 es
// Backing VS
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoords;

out vec2 TexCoords;

void main()
{
    TexCoords = aTexCoords;
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
}
"#;

#[cfg(target_arch = "wasm32")]
pub static CONSOLE_NO_BG_FS: &str = r#"#version 300 es
// Console No Background Fragment
precision highp float;
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;
in vec3 ourBackground;

// texture sampler
uniform sampler2D texture1;

void main()
{
    vec4 original = texture(texture1, TexCoord);
    if (original.r < 0.1 || original.g < 0.1 || original.b < 0.1) discard;
    vec4 fg = original * vec4(ourColor, 1.0);
	FragColor = fg;
}
"#;

#[cfg(target_arch = "wasm32")]
pub static CONSOLE_NO_BG_VS: &str = r#"#version 300 es
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

#[cfg(target_arch = "wasm32")]
pub static CONSOLE_WITH_BG_FS : &str = r#"#version 300 es
precision highp float;
in vec3 ourColor;
in vec3 ourBackground;
in vec2 TexCoord;

// texture sampler
uniform sampler2D texture1;

out vec4 FragColor;

void main()
{
    vec4 original = texture(texture1, TexCoord);
    vec4 fg = original.r > 0.1 || original.g > 0.1 || original.b > 0.1 ? original * vec4(ourColor, 1.0) : vec4(ourBackground, 1.0);
	FragColor = fg;
}
"#;

#[cfg(target_arch = "wasm32")]
pub static CONSOLE_WITH_BG_VS: &str = r#"#version 300 es
in vec3 aPos;
in vec3 aColor;
in vec3 bColor;
in vec2 aTexCoord;

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

#[cfg(target_arch = "wasm32")]
pub static SCANLINES_FS : &str = r#"#version 300 es
// Scanlines FS
precision highp float;
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
}"#;

#[cfg(target_arch = "wasm32")]
pub static SCANLINES_VS: &str = r#"#version 300 es
// Scanlines VS
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoords;

out vec2 TexCoords;

void main()
{
    TexCoords = aTexCoords;
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
}
"#;
