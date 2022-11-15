use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    let bracket = BTermBuilder::empty()
        .with_font("vga8x16-color-alpha.png", 16, 16, (8.0, 16.0))
        .with_simple_console(0, 80, 25);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bracket)
        .add_system(tick)
        .run();
}

struct TickResource(VirtualConsole);

impl Default for TickResource {
    fn default() -> Self {
        Self(VirtualConsole::from_text(TALE_OF_TWO_CITIES_INTRO, 79))
    }
}

fn tick(ctx: Res<BracketContext>, tale_of_two_cities: Local<TickResource>) {
    ctx.cls();
    tale_of_two_cities.0.print_sub_rect(
        bracket_bevy::prelude::Rect::with_size(0, 0, 79, 25),
        bracket_bevy::prelude::Rect::with_exact(0, 0, 79, 25),
        0,
        &ctx,
    );
}

const TALE_OF_TWO_CITIES_INTRO : &str = "I. The Period
It was the best of times,
it was the worst of times,
it was the age of wisdom,
it was the age of foolishness,
it was the epoch of belief,
it was the epoch of incredulity,
it was the season of Light,
it was the season of Darkness,
it was the spring of hope,
it was the winter of despair,

we had everything before us, we had nothing before us, we were all going direct to Heaven, we were all going direct the other way— in short, the period was so far like the present period, that some of its noisiest authorities insisted on its being received, for good or for evil, in the superlative degree of comparison only.

There were a king with a large jaw and a queen with a plain face, on the throne of England; there were a king with a large jaw and a queen with a fair face, on the throne of France. In both countries it was clearer than crystal to the lords of the State preserves of loaves and fishes, that things in general were settled for ever.

It was the year of Our Lord one thousand seven hundred and seventy-five. Spiritual revelations were conceded to England at that favoured period, as at this. Mrs. Southcott had recently attained her five-and-twentieth blessed birthday, of whom a prophetic private in the Life Guards had heralded the sublime appearance by announcing that arrangements were made for the swallowing up of London and Westminster. Even the Cock-lane ghost had been laid only a round dozen of years, after rapping out its messages, as the spirits of this very year last past (supernaturally deficient in originality) rapped out theirs. Mere messages in the earthly order of events had lately come to the English Crown and People, from a congress of British subjects in America: which, strange to relate, have proved more important to the human race than any communications yet received through any of the chickens of the Cock-lane brood.";
