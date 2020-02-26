use crate::prelude::{font::Font, init_raw, BTerm, InitHints, SimpleConsole, SparseConsole};
use crate::Result;
use std::collections::HashMap;
use std::convert::TryInto;

/// Internal structure defining a font to be loaded.
struct BuilderFont {
    path: String,
    dimensions: (u32, u32),
}

/// Internal enum defining a console to be loaded.
enum ConsoleType {
    SimpleConsole {
        width: u32,
        height: u32,
        font: String,
    },
    SparseConsole {
        width: u32,
        height: u32,
        font: String,
    },
    SparseConsoleNoBg {
        width: u32,
        height: u32,
        font: String,
    },
}

/// Provides a builder mechanism for initializing BTerm. You can chain builders together,
/// and and with a call to `.build()`. This allows you to provide settings if you want to,
/// or just use a simple initializer if you are in a hurry.
pub struct BTermBuilder {
    width: u32,
    height: u32,
    title: Option<String>,
    resource_path: String,
    fonts: Vec<BuilderFont>,
    consoles: Vec<ConsoleType>,
    tile_width: u32,
    tile_height: u32,
    platform_hints: InitHints,
}

impl BTermBuilder {
    /// Provides a new, unconfigured, starting point for an BTerm session. You'll have to
    /// specify everything manually.
    pub fn new() -> Self {
        BTermBuilder {
            width: 80,
            height: 50,
            title: None,
            resource_path: "resources".to_string(),
            fonts: Vec::new(),
            consoles: Vec::new(),
            tile_height: 8,
            tile_width: 8,
            platform_hints: InitHints::new(),
        }
    }

    /// Provides an 80x50 console in the baked-in 8x8 terminal font as your starting point.
    pub fn simple80x50() -> Self {
        let mut cb = BTermBuilder {
            width: 80,
            height: 50,
            title: None,
            resource_path: "resources".to_string(),
            fonts: Vec::new(),
            consoles: Vec::new(),
            tile_height: 8,
            tile_width: 8,
            platform_hints: InitHints::new(),
        };
        cb.fonts.push(BuilderFont {
            path: "terminal8x8.png".to_string(),
            dimensions: (8, 8),
        });
        cb.consoles.push(ConsoleType::SimpleConsole {
            width: 80,
            height: 50,
            font: "terminal8x8.png".to_string(),
        });
        cb
    }

    /// Provides an 8x8 terminal font simple console, with the specified dimensions as your starting point.
    pub fn simple<T>(width: T, height: T) -> Result<Self>
    where
        T: TryInto<u32>,
    {
        let w: u32 = width.try_into().or(Err("Must be convertible to a u32"))?;
        let h: u32 = height.try_into().or(Err("Must be convertible to a u32"))?;
        let mut cb = BTermBuilder {
            width: w,
            height: h,
            title: None,
            resource_path: "resources".to_string(),
            fonts: Vec::new(),
            consoles: Vec::new(),
            tile_height: 8,
            tile_width: 8,
            platform_hints: InitHints::new(),
        };
        cb.fonts.push(BuilderFont {
            path: "terminal8x8.png".to_string(),
            dimensions: (8, 8),
        });
        cb.consoles.push(ConsoleType::SimpleConsole {
            width: w,
            height: h,
            font: "terminal8x8.png".to_string(),
        });
        Ok(cb)
    }

    /// Provides an 80x50 terminal, in the VGA font as your starting point.
    pub fn vga80x50() -> Self {
        let mut cb = BTermBuilder {
            width: 80,
            height: 50,
            title: None,
            resource_path: "resources".to_string(),
            fonts: Vec::new(),
            consoles: Vec::new(),
            tile_height: 16,
            tile_width: 8,
            platform_hints: InitHints::new(),
        };
        cb.fonts.push(BuilderFont {
            path: "vga8x16.png".to_string(),
            dimensions: (8, 8),
        });
        cb.consoles.push(ConsoleType::SimpleConsole {
            width: 80,
            height: 50,
            font: "vga8x16.png".to_string(),
        });
        cb
    }

    /// Provides a VGA-font simple terminal with the specified dimensions as your starting point.
    pub fn vga<T>(width: T, height: T) -> Self
    where
        T: TryInto<u32>,
    {
        let w: u32 = width.try_into().ok().expect("Must be convertible to a u32");
        let h: u32 = height
            .try_into()
            .ok()
            .expect("Must be convertible to a u32");
        let mut cb = BTermBuilder {
            width: w,
            height: h,
            title: None,
            resource_path: "resources".to_string(),
            fonts: Vec::new(),
            consoles: Vec::new(),
            tile_height: 16,
            tile_width: 8,
            platform_hints: InitHints::new(),
        };
        cb.fonts.push(BuilderFont {
            path: "vga8x16.png".to_string(),
            dimensions: (8, 8),
        });
        cb.consoles.push(ConsoleType::SimpleConsole {
            width: w,
            height: h,
            font: "vga8x16.png".to_string(),
        });
        cb
    }

    /// Adds width/height dimensions to the BTerm builder.
    pub fn with_dimensions<T>(mut self, width: T, height: T) -> Self
    where
        T: TryInto<u32>,
    {
        self.width = width.try_into().ok().expect("Must be convertible to a u32");
        self.height = height
            .try_into()
            .ok()
            .expect("Must be convertible to a u32");
        self
    }

    /// Overrides the default assumption for tile sizes. Needed for a raw initialization.
    pub fn with_tile_dimensions<T>(mut self, width: T, height: T) -> Self
    where
        T: TryInto<u32>,
    {
        self.tile_width = width.try_into().ok().expect("Must be convertible to a u32");
        self.tile_height = height
            .try_into()
            .ok()
            .expect("Must be convertible to a u32");
        self
    }

    /// Adds a window title to the BTerm builder.
    pub fn with_title<S: ToString>(mut self, title: S) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Adds a resource path to the BTerm builder. You only need to specify this if you aren't
    /// embedding your resources.
    pub fn with_resource_path<S: ToString>(mut self, path: S) -> Self {
        self.resource_path = path.to_string();
        self
    }

    /// Adds a font registration to the BTerm builder.
    pub fn with_font<S: ToString, T>(mut self, font_path: S, width: T, height: T) -> Self
    where
        T: TryInto<u32>,
    {
        self.fonts.push(BuilderFont {
            path: font_path.to_string(),
            dimensions: (
                width.try_into().ok().expect("Must be convertible to a u32"),
                height
                    .try_into()
                    .ok()
                    .expect("Must be convertible to a u32"),
            ),
        });
        self
    }

    /// Adds a simple console layer to the BTerm builder.
    pub fn with_simple_console<S: ToString, T>(mut self, width: T, height: T, font: S) -> Self
    where
        T: TryInto<u32>,
    {
        self.consoles.push(ConsoleType::SimpleConsole {
            width: width.try_into().ok().expect("Must be convertible to a u32"),
            height: height
                .try_into()
                .ok()
                .expect("Must be convertible to a u32"),
            font: font.to_string(),
        });
        self
    }

    /// Adds a simple console, hard-coded to the baked-in 8x8 terminal font. This does NOT register the font.
    pub fn with_simple8x8(mut self) -> Self {
        self.consoles.push(ConsoleType::SimpleConsole {
            width: self.width,
            height: self.height,
            font: "terminal8x8.png".to_string(),
        });
        self
    }

    /// Adds a sparse console layer to the BTerm builder.
    pub fn with_sparse_console<S: ToString, T>(mut self, width: T, height: T, font: S) -> Self
    where
        T: TryInto<u32>,
    {
        self.consoles.push(ConsoleType::SparseConsole {
            width: width.try_into().ok().expect("Must be convertible to a u32"),
            height: height
                .try_into()
                .ok()
                .expect("Must be convertible to a u32"),
            font: font.to_string(),
        });
        self
    }

    /// Adds a sparse console with no bg rendering layer to the BTerm builder.
    pub fn with_sparse_console_no_bg<S: ToString, T>(mut self, width: T, height: T, font: S) -> Self
    where
        T: TryInto<u32>,
    {
        self.consoles.push(ConsoleType::SparseConsoleNoBg {
            width: width.try_into().ok().expect("Must be convertible to a u32"),
            height: height
                .try_into()
                .ok()
                .expect("Must be convertible to a u32"),
            font: font.to_string(),
        });
        self
    }

    /// Enables you to override the vsync default for native rendering.
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.platform_hints.vsync = vsync;
        self
    }

    /// Enables you to override the full screen setting for native rendering.
    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.platform_hints.fullscreen = fullscreen;
        self
    }

    /// Push platform-specific initialization hints to the builder. THIS REMOVES CROSS-PLATFORM COMPATIBILITY
    pub fn with_platform_specific(mut self, hints: InitHints) -> Self {
        self.platform_hints = hints;
        self
    }

    /// Instructs the back-end (not all of them honor it; WASM and Amethyst do their own thing) to try to limit frame-rate and CPU utilization.
    pub fn with_fps_cap(mut self, fps: f32) -> Self {
        self.platform_hints.frame_sleep_time = Some(1.0 / fps);
        self
    }

    /// Combine all of the builder parameters, and return an BTerm context ready to go.
    pub fn build(self) -> Result<BTerm> {
        let mut context = init_raw(
            self.width * self.tile_width,
            self.height * self.tile_height,
            self.title.unwrap_or("BTerm Window".to_string()),
            self.platform_hints,
        )?;

        let mut font_map: HashMap<String, usize> = HashMap::new();
        for font in &self.fonts {
            let font_path = format!("{}/{}", self.resource_path, font.path);
            let font_id = context.register_font(Font::load(font_path.clone(), font.dimensions));
            font_map.insert(font_path, font_id?);
        }

        for console in &self.consoles {
            match console {
                ConsoleType::SimpleConsole {
                    width,
                    height,
                    font,
                } => {
                    let font_path = format!("{}/{}", self.resource_path, font);
                    let font_id = font_map[&font_path];
                    context.register_console(SimpleConsole::init(*width, *height), font_id);
                }
                ConsoleType::SparseConsole {
                    width,
                    height,
                    font,
                } => {
                    let font_path = format!("{}/{}", self.resource_path, font);
                    let font_id = font_map[&font_path];
                    context.register_console(SparseConsole::init(*width, *height), font_id);
                }
                ConsoleType::SparseConsoleNoBg {
                    width,
                    height,
                    font,
                } => {
                    let font_path = format!("{}/{}", self.resource_path, font);
                    let font_id = font_map[&font_path];
                    context.register_console_no_bg(SparseConsole::init(*width, *height), font_id);
                }
            }
        }

        Ok(context)
    }
}
