mod display_driver;
use crate::display_driver::DisplayDriver;
use crate::life::Life;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use std::time::Duration;
mod life;

fn main() -> Result<(), String> {
    let mut display_driver = DisplayDriver::build(10);
    let (width, height) = display_driver.get_size();
    let mut life = Life::build(width as usize, height as usize, 10)?;
    life.pulsar();
    life.glider();
    display_driver.draw(&mut life.get_board(), false);
    let mut paused = false;
    let mut event_pump = display_driver.ctx.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => paused = !paused,
                _ => {}
            }
        }
        if event_pump
            .mouse_state()
            .is_mouse_button_pressed(MouseButton::Left)
        {
            let state = event_pump.mouse_state();
            let x = state.x() as usize / 10;
            let y = state.y() as usize / 10;
            display_driver.turn_on_pixel(life.get_board(), x, y, paused)?;
        }

        display_driver.draw(life.get_board(), paused)?;
        if !paused {
            life.apply_rules();
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 40));
    }
    Ok(())
}
