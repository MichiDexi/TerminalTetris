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
	collections::HashSet,
	io::{
		stdout,
		Write
	},
	thread::sleep
};
use crossterm::{
	event::{
		read,
		poll,
		Event,
		KeyCode,
		KeyEventKind,
		KeyEvent,
	},
	execute,
	cursor,
};


mod renderer;
mod current_piece;


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
	let mut input_obj = InputState::new();
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
		let input = poll_input(&mut input_obj);

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



//
// INPUT
// |
// |-> Single instance object: Input helper
// |-> Polling
//



struct InputState {
	pressed: HashSet<KeyCode>,
	just_pressed: HashSet<KeyCode>,
	last_press_time: std::collections::HashMap<KeyCode, Instant>,
}

impl InputState {
	fn new() -> Self {
		Self {
			pressed: HashSet::new(),
			just_pressed: HashSet::new(),
			last_press_time: std::collections::HashMap::new(),
		}
	}

	fn update(&mut self) {
		self.just_pressed.clear();

		while poll(Duration::from_millis(0)).unwrap() {
			if let Event::Key(KeyEvent { code, kind, .. }) = read().unwrap() &&
					matches!(kind, KeyEventKind::Press | KeyEventKind::Repeat) {
				if !self.pressed.contains(&code) {
					self.just_pressed.insert(code);
				}
				self.pressed.insert(code);
				self.last_press_time.insert(code, Instant::now());
			}
		}
		let now = Instant::now();
		self.pressed.retain(|k| {
			if let Some(&t) = self.last_press_time.get(k) {
				now.duration_since(t) < Duration::from_millis(150)
			}
			else {
				false
			}
		});
	}

	fn is_pressed(&self, key: KeyCode) -> bool {
		self.pressed.contains(&key)
	}

	fn just_pressed(&self, key: KeyCode) -> bool {
		self.just_pressed.contains(&key)
	}
}


fn poll_input(input_obj : &mut InputState) -> (i8, i8, bool, bool, bool, bool) {

	// Polling
	input_obj.update();

	// Input variables
	let left_pressed : bool = input_obj.is_pressed(KeyCode::Left);
	let right_pressed : bool = input_obj.is_pressed(KeyCode::Right);
	let rotate_left_pressed : bool = input_obj.just_pressed(KeyCode::Char('y')) || input_obj.is_pressed(KeyCode::Char('z')); // QUERTY & QUERTZ keyboard support
	let rotate_right_pressed : bool = input_obj.just_pressed(KeyCode::Char('x'));
	let soft_drop : bool = input_obj.is_pressed(KeyCode::Down) || input_obj.just_pressed(KeyCode::Down);
	let hard_drop : bool = input_obj.just_pressed(KeyCode::Up);
	let pause_pressed : bool = input_obj.just_pressed(KeyCode::Char('o'));
	let quit_pressed : bool = input_obj.just_pressed(KeyCode::Esc);

	// Output setup
	let mut x : i8 = 0;
	let mut r : i8 = 0;
	
	
	// X-position related
	if left_pressed {
		x += 1;
	}
	if right_pressed {
		x -= 1;
	}
	
	// Rotation related
	if rotate_left_pressed {
		r += 1;
	}
	if rotate_right_pressed {
		r -= 1;
	}

	(x, r, soft_drop, hard_drop, pause_pressed, quit_pressed)
}
