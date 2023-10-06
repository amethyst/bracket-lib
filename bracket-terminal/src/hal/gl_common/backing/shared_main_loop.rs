use crate::hal::scaler::FontScaler;
use crate::hal::{
    ConsoleBacking, FancyConsoleBackend, SimpleConsoleBackend, SparseConsoleBackend,
    SpriteConsoleBackend, BACKEND, CONSOLE_BACKING,
};
use crate::prelude::{FlexiConsole, SimpleConsole, SparseConsole, SpriteConsole, BACKEND_INTERNAL};
use crate::BResult;

pub(crate) fn check_console_backing() {
    let mut be = BACKEND.lock();
    let mut consoles = CONSOLE_BACKING.lock();
    if consoles.is_empty() {
        // Easy case: there are no consoles so we need to make them all.
        for cons in &BACKEND_INTERNAL.lock().consoles {
            let cons_any = cons.console.as_any();
            if let Some(st) = cons_any.downcast_ref::<SimpleConsole>() {
                consoles.push(ConsoleBacking::Simple {
                    backing: SimpleConsoleBackend::new(
                        st.width as usize,
                        st.height as usize,
                        be.gl.as_mut().unwrap(),
                    ),
                });
            } else if let Some(sp) = cons_any.downcast_ref::<SparseConsole>() {
                consoles.push(ConsoleBacking::Sparse {
                    backing: SparseConsoleBackend::new(
                        sp.width as usize,
                        sp.height as usize,
                        be.gl.as_ref().unwrap(),
                    ),
                });
            } else if let Some(sp) = cons_any.downcast_ref::<FlexiConsole>() {
                consoles.push(ConsoleBacking::Fancy {
                    backing: FancyConsoleBackend::new(
                        sp.width as usize,
                        sp.height as usize,
                        be.gl.as_ref().unwrap(),
                    ),
                });
            } else if let Some(sp) = cons_any.downcast_ref::<SpriteConsole>() {
                consoles.push(ConsoleBacking::Sprite {
                    backing: SpriteConsoleBackend::new(
                        sp.width as usize,
                        sp.height as usize,
                        be.gl.as_ref().unwrap(),
                    ),
                });
            } else {
                panic!("Unknown console type.");
            }
        }
    }
}

pub(crate) fn rebuild_consoles() {
    let must_resize = BACKEND.lock().screen_scaler.get_resized_and_reset();
    let mut consoles = CONSOLE_BACKING.lock();
    let mut bi = BACKEND_INTERNAL.lock();
    let ss = bi.sprite_sheets.clone();
    for (i, c) in consoles.iter_mut().enumerate() {
        let font_index = bi.consoles[i].font_index;
        let glyph_dimensions = bi.fonts[font_index].font_dimensions_glyphs;
        let tex_dimensions = bi.fonts[font_index].font_dimensions_texture;
        let cons = &mut bi.consoles[i];
        match c {
            ConsoleBacking::Simple { backing } => {
                let sc = cons
                    .console
                    .as_any_mut()
                    .downcast_mut::<SimpleConsole>()
                    .unwrap();
                if sc.is_dirty {
                    backing.rebuild_vertices(
                        sc.height,
                        sc.width,
                        &sc.tiles,
                        sc.offset_x,
                        sc.offset_y,
                        sc.scale,
                        sc.scale_center,
                        sc.needs_resize_internal || must_resize,
                        FontScaler::new(glyph_dimensions, tex_dimensions),
                    );
                    sc.needs_resize_internal = false;
                }
            }
            ConsoleBacking::Sparse { backing } => {
                let sc = bi.consoles[i]
                    .console
                    .as_any_mut()
                    .downcast_mut::<SparseConsole>()
                    .unwrap();
                if sc.is_dirty {
                    backing.rebuild_vertices(
                        sc.height,
                        sc.width,
                        sc.offset_x,
                        sc.offset_y,
                        sc.scale,
                        sc.scale_center,
                        &sc.tiles,
                        FontScaler::new(glyph_dimensions, tex_dimensions),
                        must_resize,
                    );
                    sc.needs_resize_internal = false;
                }
            }
            ConsoleBacking::Fancy { backing } => {
                let fc = bi.consoles[i]
                    .console
                    .as_any_mut()
                    .downcast_mut::<FlexiConsole>()
                    .unwrap();
                if fc.is_dirty {
                    fc.tiles.sort_by(|a, b| a.z_order.cmp(&b.z_order));
                    backing.rebuild_vertices(
                        fc.height,
                        fc.width,
                        fc.offset_x,
                        fc.offset_y,
                        fc.scale,
                        fc.scale_center,
                        &fc.tiles,
                        FontScaler::new(glyph_dimensions, tex_dimensions),
                    );
                    fc.needs_resize_internal = false;
                }
            }
            ConsoleBacking::Sprite { backing } => {
                let sc = bi.consoles[i]
                    .console
                    .as_any_mut()
                    .downcast_mut::<SpriteConsole>()
                    .unwrap();
                if sc.is_dirty {
                    sc.sprites.sort_by(|a, b| a.z_order.cmp(&b.z_order));
                    backing.rebuild_vertices(
                        sc.height,
                        sc.width,
                        &sc.sprites,
                        &ss[sc.sprite_sheet],
                    );
                    sc.needs_resize_internal = false;
                }
            }
        }
    }
}

pub(crate) fn render_consoles() -> BResult<()> {
    let bi = BACKEND_INTERNAL.lock();
    let mut consoles = CONSOLE_BACKING.lock();
    for (i, c) in consoles.iter_mut().enumerate() {
        let cons = &bi.consoles[i];
        let font = &bi.fonts[cons.font_index];
        let shader = &bi.shaders[cons.shader_index];
        match c {
            ConsoleBacking::Simple { backing } => {
                backing.gl_draw(font, shader)?;
            }
            ConsoleBacking::Sparse { backing } => {
                backing.gl_draw(font, shader)?;
            }
            ConsoleBacking::Fancy { backing } => {
                backing.gl_draw(font, shader)?;
            }
            ConsoleBacking::Sprite { backing } => {
                let sprite_sheet = cons
                    .console
                    .as_any()
                    .downcast_ref::<SpriteConsole>()
                    .unwrap()
                    .sprite_sheet;
                backing.gl_draw(
                    bi.sprite_sheets[sprite_sheet].backing.as_ref().unwrap(),
                    shader,
                )?;
            }
        }
    }
    Ok(())
}
