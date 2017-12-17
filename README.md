# Next V3

Version 3 of the nest API https://github.com/nest-rs/nest

## Usage

Cargo.toml
```toml
[dependencies]
nest = { git = "https://github,com/twh2898/nestv3.git" }
```

main.rs
```rust
extern crate glium;
use glium::glutin;

extern crate nestv3 as nest;
use nest::*;

fn main() {
	let mut event_loop = glutin::EventsLoop::new();
	let mut app = Window::new("Window Example", 640, 480, &event_loop).unwrap();

	let mut running = true;
	
	while running {
		let mut frame = app.next_frame();
		{
			frame.clear_color([0.0, 0.0, 0.0, 1.0]);
			/* Draw Code Here */
		}
		frame.finish().unwrap();

		event_loop.poll_events(|ev| {
			use glium::glutin::{Event, WindowEvent};

			match ev {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::Closed => running = false,
					_ => (),
				}
				_ => (),
			}
		});
	}
}
```

## Licence

nestv3 uses the [MIT](LICENCE) licence
