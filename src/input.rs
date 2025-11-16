use std::{
	time::{
		Duration,
		Instant
	}
};
use std::{
	// env <- Will be important later
	collections::HashSet,
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
};


pub struct InputState {
	pressed: HashSet<KeyCode>,
	just_pressed: HashSet<KeyCode>,
	last_press_time: std::collections::HashMap<KeyCode, Instant>,
}

impl InputState {
	pub fn new() -> Self {
		Self {
			pressed: HashSet::new(),
			just_pressed: HashSet::new(),
			last_press_time: std::collections::HashMap::new(),
		}
	}

	pub fn update(&mut self) {
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

	pub fn is_pressed(&self, key: KeyCode) -> bool {
		self.pressed.contains(&key)
	}

	pub fn just_pressed(&self, key: KeyCode) -> bool {
		self.just_pressed.contains(&key)
	}
}


pub fn poll_input(input_obj : &mut InputState) -> (i8, i8, bool, bool, bool, bool) {

	// Polling
	input_obj.update();

	// Input variables
	let left_pressed : bool = input_obj.is_pressed(KeyCode::Left);
	let right_pressed : bool = input_obj.is_pressed(KeyCode::Right);
	let rotate_left_pressed : bool = input_obj.just_pressed(KeyCode::Char('y')) || input_obj.is_pressed(KeyCode::Char('z')); // QUERTY & QUERTZ keyboard support
	let rotate_right_pressed : bool = input_obj.just_pressed(KeyCode::Char('x'));
	let soft_drop : bool = input_obj.is_pressed(KeyCode::Down) || input_obj.just_pressed(KeyCode::Down);
	let hard_drop : bool = input_obj.just_pressed(KeyCode::Up);
	let pause_pressed : bool = input_obj.just_pressed(KeyCode::Tab);
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
