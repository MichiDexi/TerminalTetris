// Headers
use nalgebra::SMatrix;
use std::{
	time::{
		Duration,
		Instant
	},
	io
};
use std::{
	// env, <- Will be important later
	io::{
		stdout,
		Write
	},
	thread::sleep
};
use crossterm::{
	terminal::size,
	execute,
	cursor,
	event::KeyCode
};

// Files
mod renderer;
mod current_piece;
mod input;

// Constant values
const TARGET_FPS : f32 = 59.73;

// Entry point
fn main() -> io::Result<()> {

	// Setup
	let frame_time : Duration = Duration::from_secs_f32(1.0 / TARGET_FPS); // Calculates the time per frame
	print!("\x1B[?25l"); // hide cursor

	
	// TODO: Add Gameboard size, start level, visual size change with Arguments
	// |-> let args: Vec<String> = env::args().collect();
	let mut input_obj = input::InputState::new();
	crossterm::terminal::enable_raw_mode().unwrap();


	// The actual game
	let scores = game(&mut input_obj, frame_time);

	
	// Program end
	crossterm::terminal::disable_raw_mode().unwrap(); // Disables raw mode
	
	let mut stdout = stdout();
	execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap(); // Clears terminal
	execute!(stdout, cursor::MoveTo(0, 0)).unwrap(); // Moves cursor to 0,0
	write!(stdout, "Score: {}\nLevel: {}\nLines: {}\n", scores.1, scores.0, scores.2)?; // Endscreen
	stdout.flush()?;
	
	print!("\x1B[?25h"); // Shows cursor
	Ok(())
}

// Game function
fn game(
	input_obj : &mut input::InputState,
	framerate : Duration)
	-> (u8, u32, u32) // Returns scores
{
	// Return initialization
	let mut level : u8 = 0;
	let mut score : u32 = 0;
	let mut lines : u32 = 0;

	// Game setup
	
	let mut cur_obj = current_piece::CurrentObject {
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
	let _ = cur_obj.reset_obj(); // Piece gets reset

	let mut map : SMatrix<u8, 10, 18> = SMatrix::zeros(); // The playfield
	let mut playfield_buffer : SMatrix<u8, 12, 19> = SMatrix::zeros(); // Used to render playfield
	let mut piecepreview_buffer : SMatrix<u8, 6, 6> = SMatrix::zeros(); // Used to render piece preview
	renderer::border(&mut playfield_buffer);
	

	let (cols, rows) = size().unwrap(); // Gets size of terminal
	let x_offset = (cols/2) as u8 -18; // Render x offset
	let y_offset = (rows/2) as u8 -9; // Render y offset

	// Main loop
	loop {
	
		let now = Instant::now(); // Get frame time
		let input_check = input::poll_input(input_obj); // Poll input


		// Player object
		let piecepreview_render_flag = cur_obj.tick_obj(&mut map,
			(input_check.0, input_check.1, input_check.2, input_check.3),
			(&mut level, &mut score, &mut lines)
		);
		if cur_obj.dead { // Check to stop the game if the piece is stuck
			break;
		}

		// Other
		if input_check.4 { // Pause key


			// Wait for unpause
			loop {
				sleep(Duration::from_millis(100));
				input_obj.update();
				if input_obj.just_pressed(KeyCode::Esc) ||
					input_obj.just_pressed(KeyCode::Tab) 
				{
					break;
				}
				
			}
		}
		if input_check.5 { // Quit key
			break;
		}

		// Render
		renderer::inject_buffers(&mut playfield_buffer, &cur_obj, map);
		let _ = renderer::render_buffer(&playfield_buffer, x_offset, y_offset);
		if piecepreview_render_flag.is_ok() {
			let _ = renderer::render_piece_preview(&mut piecepreview_buffer, &cur_obj, x_offset+10, y_offset);
		}

		// Frame time management for consistent framerate
		let frame_duration = Instant::now().duration_since(now);
		if frame_duration < framerate {
			sleep(framerate - frame_duration);
		}
	}
	
	(level, score, lines)
}
