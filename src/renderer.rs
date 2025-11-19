use crossterm::{
	terminal::size,
	execute,
	cursor
};
use std::io;
use std::io::{stdout, Write};
use nalgebra::SMatrix;

use crate::current_piece::CurrentObject;


pub fn render_all(obj : &CurrentObject, map : SMatrix<u8, 10, 18>, level : u8, score : u32, lines : u32) -> io::Result<()> {
	let (cols, rows) = size().unwrap();
	let x_offset = (cols/2) as u8 -18;
	let y_offset = (rows/2) as u8 -9;

	let mut playfield_buffer : SMatrix<u8, 12, 19> = SMatrix::zeros(); // x0/11 & y19 are borders

	playfield(&mut playfield_buffer, &map);
	player_object(&mut playfield_buffer, obj);
	border(&mut playfield_buffer);
	render_buffer(&playfield_buffer, x_offset, y_offset)?;
	// render(obj, map, x_offset, y_offset)?;
	// update_border(x_offset, y_offset)?;
	update_display(x_offset, y_offset, level, score, lines)?;
	update_piece_preview(x_offset, y_offset, obj);

	
	
	Ok(())
}

fn player_object(buffer : &mut SMatrix<u8, 12, 19>, player_obj : &CurrentObject) {

	// Set positions
	
	let x : i8 = player_obj.cx as i8;
	let x1 : i8 = player_obj.cx as i8 + player_obj.x1;
	let x2 : i8 = player_obj.cx as i8 + player_obj.x2;
	let x3 : i8 = player_obj.cx as i8 + player_obj.x3;
	let y : i8 = player_obj.cy as i8;
	let y1 : i8 = player_obj.cy as i8 + player_obj.y1;
	let y2 : i8 = player_obj.cy as i8 + player_obj.y2;
	let y3 : i8 = player_obj.cy as i8 + player_obj.y3;
	
	// Set object positions in buffer
	if check_out_of_bounds(x, y) {
		buffer[(1+x as usize, y as usize)] = player_obj.otype+1;
	}
	if check_out_of_bounds(x1, y1) {
		buffer[(1+x1 as usize, y1 as usize)] = player_obj.otype+1;
	}
	if check_out_of_bounds(x2, y2) {
		buffer[(1+x2 as usize, y2 as usize)] = player_obj.otype+1;
	}
	if check_out_of_bounds(x3, y3) {
		buffer[(1+x3 as usize, y3 as usize)] = player_obj.otype+1;
	}
}

fn check_out_of_bounds(x : i8, y : i8) -> bool {
	if x > -1 && x < 12 &&
		y > -1 && y < 19 {

		return true;
	}

	false
}

fn playfield(buffer : &mut SMatrix<u8, 12, 19>, map : &SMatrix<u8, 10, 18>) {
	// Write map (with x_offset of 1) into buffer
	for x in 0..10 {
		for y in 0..18 {
			buffer[(x+1, y)] = map[(x, y)];
		}
	}
}

fn border(buffer : &mut SMatrix<u8, 12, 19>) {
	// Walls
	for y in 0..19 {
		buffer[(0,  y)] = 7;
		buffer[(11, y)] = 7;
	}

	// Floor
	for x in 1..11 {
		buffer[(x, 18)] = 7;
	}
}

fn render_buffer(buffer : &SMatrix<u8, 12, 19>, x_offset : u8, y_offset : u8) -> io::Result<()> {

	let mut stdout = stdout();
	
	for y in 0..19 {
		execute!(stdout, cursor::MoveTo(x_offset as u16, y as u16 + y_offset as u16)).unwrap();
		for x in 0..12 {
			write!(stdout, "\x1b[38;5;{}m██", buffer[(x, y)]).unwrap(); // Reads colored pixel from buffer
		}
	}
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
