use nalgebra::SMatrix;
use std::{
	time::{
		Duration,
		Instant
	},
	io
};
use std::{
	// env <- Will be important later
	io::{
		stdout,
		Write
	},
	thread::sleep
};
use crossterm::{
	execute,
	cursor,
	event::KeyCode
};


mod renderer;
mod current_piece;
mod input;


//
// MAIN FUNCTION
// |
// |-> Entry point
//



fn main() -> io::Result<()> {
	
	let fps: f32 = 59.73;
	let frame_time: Duration = Duration::from_secs_f32(1.0 / fps);
	print!("\x1B[?25l"); // hide cursor
	
	// let args: Vec<String> = env::args().collect(); // Gameboard size, start level, visual size // TODO: Add thing behavior
	let mut input_obj = input::InputState::new();
	crossterm::terminal::enable_raw_mode().unwrap();
	
	let mut level : u8 = 0;
	let mut score : u32 = 0;
	let mut lines : u32 = 0;
	
	let mut map : SMatrix<u8, 10, 18> = SMatrix::zeros(); // matrix stuff 10x18
	let mut cur_obj : current_piece::CurrentObject = current_piece::CurrentObject {
		cx: 0,
		cy: 0,
		x1: 0,
		y1: 0,
		x2: 0,
		y2: 0,
		x3: 0,
		y3: 0,
		tick_delay: 0,
		exists: false,
		exist_delay: 100,
		otype: 0,
		move_delay: 15,
		dead : false,
		pieces : vec!(0, 0),
	};
	
	let mut running : bool = true;
	current_piece::reset_obj(&mut cur_obj)?;
	

	// Main loop
	while running {

		// Frame time
		let now = Instant::now();

		// Input
		let input = input::poll_input(&mut input_obj);

		// Player object
		current_piece::tick_obj(&mut map, &mut cur_obj,
				(input.0, input.1, input.2, input.3),
				(&mut level, &mut score, &mut lines)
				)?;
		
		if cur_obj.dead {
			running = false;
		}

		// Other
		if input.4 { // Paused
			while !input_obj.just_pressed(KeyCode::Char('p')) {
				input_obj.update();
				sleep(Duration::from_millis(100));
			}
		}
		if input.5 { // Quit
			running = false;
		}

		// Render
		renderer::render_all(&cur_obj, map, level, score, lines)?;

		// Frame time II
		let frame_duration = Instant::now().duration_since(now);
		if frame_duration < frame_time {
			sleep(frame_time - frame_duration);
		}
	}
	crossterm::terminal::disable_raw_mode().unwrap();
	let mut stdout = stdout();
	execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
	execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
	write!(stdout, "Score: {}\nLevel: {}\nLines: {}\n", score, level, lines)?;
	stdout.flush()?;
	print!("\x1B[?25h"); // show cursor
	Ok(())
}
