use nalgebra::{ SMatrix };
use std::{time::{Duration, Instant}, io};
// use std::env; <- Will be important later
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::thread::sleep;
use crossterm::{
	event::{
		read, poll, Event, KeyCode, KeyEventKind, KeyEvent,
	},
	terminal::{
		size,
	},
	execute,
	cursor,
};
use rand::prelude::*;



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
	let mut cur_obj : CurrentObject = CurrentObject {
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
	reset_obj(&mut cur_obj)?;
	

	// Main loop
	while running {

		// Frame time
		let now = Instant::now();

		// Input
		let input = poll_input(&mut input_obj);

		// Player object
		tick_obj(&mut map, &mut cur_obj,
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
		render_all(&cur_obj, map, level, score, lines)?;

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

fn render_all(obj : &CurrentObject, map : SMatrix<u8, 10, 18>, level : u8, score : u32, lines : u32) -> io::Result<()> {
	let (cols, rows) = size().unwrap();
	let x_offset = (cols/2) as u8 -18;
	let y_offset = (rows/2) as u8 -9;
	
	render(obj, map, x_offset, y_offset)?;
	update_border(x_offset, y_offset)?;
	update_display(x_offset, y_offset, level, score, lines)?;
	update_piece_preview(x_offset, y_offset, obj);

	Ok(())
}



//
// SINGLE INSTANCE OBJECTS
// |
// |-> CurrentObject (manages player input)
// |-> Random (automatic randomizer)
//



struct CurrentObject {
	// Center piece (everything rotates around this point)
	cx : u8,
	cy : u8,
	
	// Other pieces
	x1 : i8,
	y1 : i8,
	x2 : i8,
	y2 : i8,
	x3 : i8,
	y3 : i8,

	tick_delay : i8, // Ticks to wait to move down once
	exists : bool,
	exist_delay : i8,
	otype : u8,
	move_delay : u8,
	dead : bool,

	pieces : Vec<u8>,
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



//
// PLAYER INPUT
// |
// |-> Player object related
//



fn tick_obj(
	matrix : &mut SMatrix<u8, 10, 18>,
	obj : &mut CurrentObject,
	input : (i8, i8, bool, bool),
	score_vars : (&mut u8, &mut u32, &mut u32)) -> io::Result<()>
{

	// Object reset
	if !obj.exists {
		if obj.exist_delay <= 0 {
			reset_obj(obj)?;
			if !try_move(matrix, obj, 0, 1) &&
				!try_move(matrix, obj, -1, 0) &&
				!try_move(matrix, obj, 1, 0) {
				obj.dead = true;
			}
		}
		else {
			obj.exist_delay -= 1;
			return Ok(());
		}
	}
	
	// Vertical movement
	if obj.tick_delay <= 0 {
		if try_move(matrix, obj, 0, 1) {
			obj.cy += 1;
			obj.tick_delay = match score_vars.0 {
				0 => 53,
				1 => 49,
				2 => 45,
				3 => 41,
				4 => 37,
				5 => 33,
				6 => 28,
				7 => 22,
				8 => 17,
				9 => 11,
				10 => 10,
				11 => 9,
				12 => 8,
				13 => 7,
				14 => 6,
				15 => 6,
				16 => 5,
				17 => 5,
				18 => 4,
				19 => 4,
				_ => 3
			}
		}
		else {
			matrix[(obj.cx as usize, obj.cy as usize)] = obj.otype+1;
			matrix[((obj.x1+obj.cx as i8) as usize, (obj.y1+obj.cy as i8) as usize)] = obj.otype+1;
			matrix[((obj.x2+obj.cx as i8) as usize, (obj.y2+obj.cy as i8) as usize)] = obj.otype+1;
			matrix[((obj.x3+obj.cx as i8) as usize, (obj.y3+obj.cy as i8) as usize)] = obj.otype+1;
			obj.exists = false;
			obj.exist_delay = 10;
			check_rows(matrix, score_vars.0, score_vars.1, score_vars.2);
		}
	}
	else if input.2 {
		obj.tick_delay -= 3;
	}
	else {
		obj.tick_delay -= 1;
	}

	// Player input
	if obj.move_delay == 0 {
		if input.0 != 0 {
			if input.0 < 0 { // Right
				if try_move(matrix, obj, 1, 0) {
					obj.cx += 1;
					obj.move_delay = 15;
				}
			}
			else { // Left
				if try_move(matrix, obj, -1, 0) {
					obj.cx -= 1;
					obj.move_delay = 15;
				}
			}
		}
	}
	else {
		obj.move_delay -= 1;
	}



	if input.1 != 0 && try_rotate(matrix, obj, input.1) {
		if input.1 > 0 { // Clockwise
			let mut temp : i8;

			temp = obj.x1;
			obj.x1 = obj.y1;
			obj.y1 = -temp;

			temp = obj.x2;
			obj.x2 = obj.y2;
			obj.y2 = -temp;

			temp = obj.x3;
			obj.x3 = obj.y3;
			obj.y3 = -temp;
		}
		else { // Counter clockwise
			let mut temp : i8;

			temp = obj.x1;
			obj.x1 = -obj.y1;
			obj.y1 = temp;

			temp = obj.x2;
			obj.x2 = -obj.y2;
			obj.y2 = temp;

			temp = obj.x3;
			obj.x3 = -obj.y3;
			obj.y3 = temp;
		}
	}
	Ok(())
}

fn try_move(matrix : &SMatrix<u8, 10, 18>, obj : &CurrentObject, x : i8, y : i8) -> bool {

	let target_cx = obj.cx as i8 +x;
	let target_cy = obj.cy as i8 +y;
	let target_x1 = obj.x1 +x+obj.cx as i8;
	let target_y1 = obj.y1 +y+obj.cy as i8;
	let target_x2 = obj.x2 +x+obj.cx as i8;
	let target_y2 = obj.y2 +y+obj.cy as i8;
	let target_x3 = obj.x3 +x+obj.cx as i8;
	let target_y3 = obj.y3 +y+obj.cy as i8;


	// Out of bounds check
	if check_out_of_bounds(target_cx, target_cy) ||
		check_out_of_bounds(target_x1, target_y1) ||
		check_out_of_bounds(target_x2, target_y2) ||
		check_out_of_bounds(target_x3, target_y3)
	{
		return false;
	}

	// Check matrix
	if matrix[(target_cx as usize, target_cy as usize)] != 0 ||
		matrix[(target_x1 as usize, target_y1 as usize)] != 0 ||
		matrix[(target_x2 as usize, target_y2 as usize)] != 0 ||
		matrix[(target_x3 as usize, target_y3 as usize)] != 0
	{
		return false;
	}
	true
}

fn try_rotate(matrix : &SMatrix<u8, 10, 18>, obj : &CurrentObject, r : i8) -> bool {

	let target_cx = obj.cx as i8;
	let target_cy = obj.cy as i8;
	// TODO: Use expression based stuff thingy	
	let mut target_x1 = (-obj.y1) +obj.cx as i8;
	let mut target_y1 = (obj.x1) +obj.cy as i8;
	let mut target_x2 = (-obj.y2) +obj.cx as i8;
	let mut target_y2 = (obj.x2) +obj.cy as i8;
	let mut target_x3 = (-obj.y3) +obj.cx as i8;
	let mut target_y3 = (obj.y3) +obj.cy as i8;

	if r == 1 {
		target_x1 = (obj.y1) +obj.cx as i8;
		target_y1 = (-obj.x1) +obj.cy as i8;
		target_x2 = (obj.y2) +obj.cx as i8;
		target_y2 = (-obj.x2) +obj.cy as i8;
		target_x3 = (obj.y3) +obj.cx as i8;
		target_y3 = (-obj.x3) +obj.cy as i8;
	}



	// Out of bounds check
	if check_out_of_bounds(target_cx, target_cy) ||
		check_out_of_bounds(target_x1, target_y1) ||
		check_out_of_bounds(target_x2, target_y2) ||
		check_out_of_bounds(target_x3, target_y3)
	{
		return false;
	}

	// Check matrix
	if matrix[(target_cx as usize, target_cy as usize)] != 0 ||
		matrix[(target_x1 as usize, target_y1 as usize)] != 0 ||
		matrix[(target_x2 as usize, target_y2 as usize)] != 0 ||
		matrix[(target_x3 as usize, target_y3 as usize)] != 0
	{
		return false;
	}
	true
}

fn check_out_of_bounds(x : i8, y : i8) -> bool {
	if !(0..10).contains(&x) {
		return true;
	}
	if !(0..18).contains(&y) {
		return true;
	}
	false
}

fn reset_obj(obj : &mut CurrentObject) -> io::Result<()> {

	let mut rng = rand::rng();
	let obj_type: usize = rng.random_range(0..7);
	obj.pieces.push(obj_type as u8);
	obj.pieces.remove(0);

	obj.cx = 4;
	obj.cy = 1;
	obj.otype = obj.pieces[0];
	obj.exists = true;

	match obj.pieces[0] {
		0 => { // L : 000
			set_positions(obj, -1, 0, -1, 1, 1, 0);
			Ok(())
		}
		1 => { // J : 001
			set_positions(obj, -1, 0, 1, 1, 1, 0);
			Ok(())
		}
		2 => { // I : 010
			set_positions(obj, -1, 0, 2, 0, 1, 0);
			Ok(())
		}
		3 => { // O : 011
			set_positions(obj, 0, 1, 1, 1, 1, 0);
			Ok(())
		}
		4 => { // Z : 100
			set_positions(obj, -1, 0, 0, 1, 1, 1);
			Ok(())
		}
		5 => { // S : 101
			set_positions(obj, 1, 0, 0, 1, -1, 1);
			Ok(())
		}
		6 => { // T : 110
			set_positions(obj, -1, 0, 1, 0, 0, 1);
			Ok(())
		}
		_ => {
			Err(io::Error::other("how did you mess up this badly broo"))
		}
	}
}

fn set_positions(obj : &mut CurrentObject, x1 : i8, y1 : i8, x2 : i8, y2 : i8, x3 : i8, y3 : i8) {
	obj.x1 = x1;
	obj.y1 = y1;
	obj.x2 = x2;
	obj.y2 = y2;
	obj.x3 = x3;
	obj.y3 = y3;
}

fn check_rows(map : &mut SMatrix<u8, 10, 18>, level : &mut u8, score : &mut u32, lines : &mut u32) {

	let mut lines_this_frame : u8 = 0;
	
	for i in 0..18 {
		let mut mark_row : bool = true;
		for j in 0..10 {
			if map[(j as usize, i as usize)] == 0 {
				mark_row = false;
			}
		}
		if mark_row {
			clear_row(map, i);
			lines_this_frame += 1;
		}
	}
	*score += match lines_this_frame {
		1 => 40 * (*level+1) as u32,
		2 => 100 * (*level+1) as u32,
		3 => 300 * (*level+1) as u32,
		4 => 1200 * (*level+1) as u32,
		_ => 0
	};
	*lines += lines_this_frame as u32;
	*level = (*lines / 10) as u8;
}

fn clear_row(map : &mut SMatrix<u8, 10, 18>, row : u8) {
	for i in 0..10 {
		map[(i as usize, row as usize)] = 0;
	}

	for i in 0..10 {
		for j in 0..row {
			map[(i as usize, (row-j) as usize)] = map[(i as usize, (row-j-1) as usize)];
		}
	}
}


//
// RENDERER
// |
// |-> Graphics related
//



fn render(player_obj : &CurrentObject, map : SMatrix<u8, 10, 18>, x_offset : u8, y_offset : u8) -> io::Result<()> {

	let mut stdout = stdout();
	execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();

	// Matrix
	for i in 0..10 {
		for j in 0..18 {
			execute!(stdout, cursor::MoveTo(2+2*i +x_offset as u16, j +y_offset as u16)).unwrap();
			write!(stdout, "\x1b[38;5;{}m", map[(i as usize, j as usize)])?;
			write!(stdout, "██").unwrap();
		}
	}
	write!(stdout, "\x1b[38;5;{}m", player_obj.otype+1)?;

	// Player object
	if player_obj.exists {
		execute!(stdout, cursor::MoveTo((2+2*(player_obj.x1+player_obj.cx as i8) +x_offset as i8) as u16, ((player_obj.y1+player_obj.cy as i8) +y_offset as i8) as u16)).unwrap();
		write!(stdout, "██").unwrap();
		execute!(stdout, cursor::MoveTo((2+2*(player_obj.x2+player_obj.cx as i8) +x_offset as i8) as u16, ((player_obj.y2+player_obj.cy as i8) +y_offset as i8) as u16)).unwrap();
		write!(stdout, "██").unwrap();
		execute!(stdout, cursor::MoveTo((2+2*(player_obj.x3+player_obj.cx as i8) +x_offset as i8) as u16, ((player_obj.y3+player_obj.cy as i8) +y_offset as i8) as u16)).unwrap();
		write!(stdout, "██").unwrap();
		execute!(stdout, cursor::MoveTo((2+2*(player_obj.cx) +x_offset) as u16, (player_obj.cy +y_offset) as u16)).unwrap();
		write!(stdout, "██").unwrap();
	}

	stdout.flush().unwrap(); // flush manually
	Ok(())
}

fn update_display(
	x_offset : u8,
	y_offset : u8,
	level : u8,
	score : u32,
	lines : u32) -> io::Result<()> {

	let mut stdout = stdout();

	write!(stdout, "\x1b[38;5;7m")?;
	
	execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 3 +y_offset as u16)).unwrap();
	write!(stdout, "Score").unwrap();
	execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 4 +y_offset as u16)).unwrap();
	write!(stdout, "{}", score).unwrap();
	
	execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 6 +y_offset as u16)).unwrap();
	write!(stdout, "Lines").unwrap();
	execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 7 +y_offset as u16)).unwrap();
	write!(stdout, "{}", lines).unwrap();
	
	execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 9 +y_offset as u16)).unwrap();
	write!(stdout, "Level").unwrap();
	execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 10 +y_offset as u16)).unwrap();
	write!(stdout, "{}", level).unwrap();
	
	stdout.flush().unwrap(); // flush manually
	Ok(())
}

fn update_border(
	x_offset : u8,
	y_offset : u8) -> io::Result<()> {

	let mut stdout = stdout();
	write!(stdout, "\x1b[38;5;7m")?;
	for i in 0..18 {
		execute!(stdout, cursor::MoveTo(x_offset as u16, i +y_offset as u16)).unwrap();
		write!(stdout, "██").unwrap();
		execute!(stdout, cursor::MoveTo(22 +x_offset as u16, i +y_offset as u16)).unwrap();
		write!(stdout, "██").unwrap();
	}
	execute!(stdout, cursor::MoveTo(x_offset as u16, 18 +y_offset as u16)).unwrap();
	write!(stdout, "████████████████████████").unwrap();
	
	stdout.flush().unwrap(); // flush manually
	Ok(())
}

fn update_piece_preview(
	x_offset : u8,
	y_offset : u8,
	obj : &CurrentObject) {

	let mut stdout = stdout();
	execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 12 +y_offset as u16)).unwrap();
	write!(stdout, "Next").unwrap();

	execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 13 +y_offset as u16)).unwrap();
	match obj.pieces[1] {
		0 => { // L : 000
			write!(stdout, "██████").unwrap();
			execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 14 +y_offset as u16)).unwrap();
			write!(stdout, "██").unwrap();
		}
		1 => { // J : 001
			write!(stdout, "██████").unwrap();
			execute!(stdout, cursor::MoveTo(32 +x_offset as u16, 14 +y_offset as u16)).unwrap();
			write!(stdout, "██").unwrap();
		}
		2 => { // I : 010
			write!(stdout, "████████").unwrap();
		}
		3 => { // O : 011
			write!(stdout, "████").unwrap();
			execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 14 +y_offset as u16)).unwrap();
			write!(stdout, "████").unwrap();
		}
		4 => { // Z : 100
			write!(stdout, "████").unwrap();
			execute!(stdout, cursor::MoveTo(30 +x_offset as u16, 14 +y_offset as u16)).unwrap();
			write!(stdout, "████").unwrap();
		}
		5 => { // S : 101
			execute!(stdout, cursor::MoveTo(30 +x_offset as u16, 13 +y_offset as u16)).unwrap();
			write!(stdout, "████").unwrap();
			execute!(stdout, cursor::MoveTo(28 +x_offset as u16, 14 +y_offset as u16)).unwrap();
			write!(stdout, "████").unwrap();
		}
		6 => { // T : 110
			write!(stdout, "██████").unwrap();
			execute!(stdout, cursor::MoveTo(30 +x_offset as u16, 14 +y_offset as u16)).unwrap();
			write!(stdout, "██").unwrap();
		}
		_ => { }
	}
	stdout.flush().unwrap(); // flush manually
}
