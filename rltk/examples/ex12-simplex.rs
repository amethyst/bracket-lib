rltk::add_wasm_support!();
use rltk::prelude::*;

struct State {
    colors: Vec<RGB>,
    counter: u64,
    timer: f32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.timer += ctx.frame_time_ms;
        if self.timer > 500.0 {
            self.timer = 0.0;
            self.rebuild_noise();
        }
        ctx.cls();

        for y in 0..50 {
            for x in 0..80 {
                let idx = ((y * 80) + x) as usize;
                ctx.set(x, y, self.colors[idx], RGB::from_f32(0.0, 0.0, 0.0), 219);
            }
        }
    }
}

impl State {
    pub fn rebuild_noise(&mut self) {
        let mut noise = FastNoise::seeded(self.counter);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);

        for y in 0..50 {
            for x in 0..80 {
                let n = noise.get_noise((x as f32) / 160.0, (y as f32) / 100.0);
                let idx = ((y * 80) + x) as usize;
                if n < 0.0 {
                    self.colors[idx] = RGB::from_f32(0.0, 0.0, 1.0 - (0.0 - n));
                } else {
                    self.colors[idx] = RGB::from_f32(0.0, n, 0.0);
                }
            }
        }
        self.counter += 1;
    }
}

fn main() -> RltkError {
    let mut gs: State = State {
        colors: vec![RGB::from_f32(0., 0., 0.); 80 * 50],
        counter: 0,
        timer: 0.0,
    };
    gs.rebuild_noise();

    let context = RltkBuilder::simple80x50()
        .with_title("RLTK Example 12 - Perlin Noise")
        .build()?;
    rltk::main_loop(context, gs)
}
