use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, Plugins};
use notan::log;
use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    notan::init().draw(draw).build();

    Ok(())
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();

    draw.begin(Some(&Color::new(0.1, 0.2, 0.3, 1.0)));
    draw.color = Color::GREEN;
    draw.triangle(400.0, 100.0, 100.0, 500.0, 700.0, 500.0);
    draw.end();

    gfx.render(&draw);
}
