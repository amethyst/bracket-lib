use std::convert::TryInto;
use crate::prelude::*;
use std::collections::HashMap;

struct BuilderFont {
    path : String,
    dimensions : (u32, u32)
}

enum ConsoleType {
    SimpleConsole{ width: u32, height: u32, font: String },
    SparseConsole{ width: u32, height: u32, font: String }
}

pub struct RltkBuilder {
    width: u32,
    height: u32,
    title : Option<String>,
    resource_path : String,
    fonts : Vec<BuilderFont>,
    consoles : Vec<ConsoleType>,
    tile_width: u32,
    tile_height: u32,
    vsync : bool
}

impl RltkBuilder {
    pub fn new() -> Self {
        RltkBuilder{ 
            width : 80,
            height : 50,
            title : None,
            resource_path : "resources".to_string(),
            fonts : Vec::new(),
            consoles : Vec::new(),
            tile_height: 8,
            tile_width: 8,
            vsync: true
        }
    }

    pub fn simple80x50() -> Self {
        let mut cb = RltkBuilder{ 
            width : 80,
            height : 50,
            title : None,
            resource_path : "resources".to_string(),
            fonts : Vec::new(),
            consoles : Vec::new(),
            tile_height: 8,
            tile_width: 8,
            vsync: true
        };
        cb.fonts.push(
            BuilderFont{ path: "terminal8x8.png".to_string(), dimensions: (8, 8) }
        );
        cb.consoles.push(ConsoleType::SimpleConsole{
            width : 80,
            height : 50,
            font : "terminal8x8.png".to_string()
        });
        cb
    }

    pub fn simple<T>(width: T, height: T) -> Self 
    where T: TryInto<u32>
    {
        let w : u32 = width.try_into().ok().unwrap();
        let h : u32 = height.try_into().ok().unwrap();
        let mut cb = RltkBuilder{ 
            width : w,
            height : h,
            title : None,
            resource_path : "resources".to_string(),
            fonts : Vec::new(),
            consoles : Vec::new(),
            tile_height: 8,
            tile_width: 8,
            vsync: true
        };
        cb.fonts.push(
            BuilderFont{ path: "terminal8x8.png".to_string(), dimensions: (8, 8) }
        );
        cb.consoles.push(ConsoleType::SimpleConsole{
            width : w,
            height : h,
            font : "terminal8x8.png".to_string()
        });
        cb
    }

    pub fn vga80x50() -> Self {
        let mut cb = RltkBuilder{ 
            width : 80,
            height : 50,
            title : None,
            resource_path : "resources".to_string(),
            fonts : Vec::new(),
            consoles : Vec::new(),
            tile_height: 16,
            tile_width: 8,
            vsync: true
        };
        cb.fonts.push(
            BuilderFont{ path: "vga8x16.png".to_string(), dimensions: (8, 8) }
        );
        cb.consoles.push(ConsoleType::SimpleConsole{
            width : 80,
            height : 50,
            font : "vga8x16.png".to_string()
        });
        cb
    }

    pub fn vga<T>(width: T, height: T) -> Self 
    where T: TryInto<u32>
    {
        let w : u32 = width.try_into().ok().unwrap();
        let h : u32 = height.try_into().ok().unwrap();
        let mut cb = RltkBuilder{ 
            width : w,
            height : h,
            title : None,
            resource_path : "resources".to_string(),
            fonts : Vec::new(),
            consoles : Vec::new(),
            tile_height: 16,
            tile_width: 8,
            vsync: true
        };
        cb.fonts.push(
            BuilderFont{ path: "vga8x16.png".to_string(), dimensions: (8, 8) }
        );
        cb.consoles.push(ConsoleType::SimpleConsole{
            width : w,
            height : h,
            font : "vga8x16.png".to_string()
        });
        cb
    }

    pub fn with_dimensions<T>(mut self, width: T, height: T) -> Self
    where
        T: TryInto<u32>,
    {
        self.width = width.try_into().ok().unwrap();
        self.height = height.try_into().ok().unwrap();
        self
    }

    pub fn with_title<S: ToString>(mut self, title : S) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_resource_path<S: ToString>(mut self, path: S) -> Self {
        self.resource_path = path.to_string();
        self
    }

    pub fn with_font<S: ToString, T>(mut self, font_path: S, width: T, height: T) -> Self
    where T:TryInto<u32>
    {
        self.fonts.push(BuilderFont{
            path : font_path.to_string(),
            dimensions: ( width.try_into().ok().unwrap(), height.try_into().ok().unwrap() )
        });
        self
    }

    pub fn with_simple_console<S:ToString, T>(mut self, width: T, height: T, font: S) -> Self
    where T : TryInto<u32>
    {
        self.consoles.push(ConsoleType::SimpleConsole{
            width : width.try_into().ok().unwrap(),
            height : height.try_into().ok().unwrap(),
            font : font.to_string()
        });
        self
    }

    pub fn with_simple8x8(mut self) -> Self {
        self.consoles.push(ConsoleType::SimpleConsole{
            width : self.width,
            height : self.height,
            font : "resources/terminal8x8.png".to_string()
        });
        self
    }

    pub fn with_sparse_console<S:ToString, T>(mut self, width: T, height: T, font: S) -> Self
    where T: TryInto<u32>
    {
        self.consoles.push(ConsoleType::SparseConsole{
            width : width.try_into().ok().unwrap(),
            height : height.try_into().ok().unwrap(),
            font : font.to_string()
        });
        self
    }

    pub fn with_vsync(mut self, vsync : bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn build(self) -> Rltk {
        let mut context = init_raw(
            self.width * self.tile_width,
            self.height * self.tile_height,
            self.title.unwrap_or("RLTK Window".to_string()),
            self.vsync
        );

        let mut font_map : HashMap<String, usize> = HashMap::new();
        for font in &self.fonts {
            let font_path = format!("{}/{}", self.resource_path, font.path);
            let font_id = context.register_font(Font::load(font_path.clone(), font.dimensions));
            println!("Registered font: {} as {}", font_path, font_id);
            font_map.insert(font_path, font_id);
        }

        for console in &self.consoles {
            match console {
                ConsoleType::SimpleConsole{width, height, font} => {
                    let font_path = format!("{}/{}", self.resource_path, font);
                    println!("Looking for font: {}", font_path);
                    let font_id = font_map[&font_path];
                    context.register_console(SimpleConsole::init(*width, *height, &context.backend), font_id);
                }
                ConsoleType::SparseConsole{width, height, font} => {
                    let font_path = format!("{}/{}", self.resource_path, font);
                    println!("Looking for font: {}", font_path);
                    let font_id = font_map[&font_path];
                    context.register_console(SparseConsole::init(*width, *height, &context.backend), font_id);
                }
            }
        }

        context
    }
}