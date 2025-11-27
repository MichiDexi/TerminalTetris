use crossterm::{
	execute,
	cursor
};
use std::io;
use std::io::{stdout, Write};
use nalgebra::SMatrix;

use crate::current_piece::CurrentObject;

pub fn inject_buffers(
	playfield_buffer : &mut SMatrix<u8, 12, 19>,
	obj : &CurrentObject, map : SMatrix<u8, 10, 18>) {

	playfield(playfield_buffer, &map);
	if obj.exists {
		player_object(playfield_buffer, obj);
	}
}

pub fn player_object(buffer : &mut SMatrix<u8, 12, 19>, player_obj : &CurrentObject) {

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

pub fn border(buffer : &mut SMatrix<u8, 12, 19>) {
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

pub fn render_buffer(buffer : &SMatrix<u8, 12, 19>, x_offset : u8, y_offset : u8) -> io::Result<()> {

	let mut stdout = stdout();
	
	for y in 0..19 {
		execute!(stdout, cursor::MoveTo(x_offset as u16, y as u16 + y_offset as u16)).unwrap();
		for x in 0..12 {
			write!(stdout, "\x1b[38;5;{}m██", buffer[(x, y)]).unwrap(); // Reads colored pixel from buffer
		}
	}
	Ok(())
}

pub fn render_piece_preview(preview : &mut SMatrix<u8, 6, 6>, player_obj : &CurrentObject, x_offset : u8, y_offset : u8) -> io::Result<()> {
	let mut stdout = stdout();

	set_next_piece(preview, player_obj);
	
	for y in 0..6 {
		execute!(stdout, cursor::MoveTo(x_offset as u16, y as u16 + y_offset as u16)).unwrap();
		for x in 0..6 {
			write!(stdout, "\x1b[38;5;{}m██", preview[(x, y)]).unwrap(); // Reads colored pixel from buffer
		}
	}
	Ok(())
}

pub fn render_text(level : &u8, score : &u32, lines : &u32, x_offset : u8, y_offset : u8) -> io::Result<()> {
	let mut stdout = stdout();

	// Score
	execute!(stdout, cursor::MoveTo(x_offset as u16, y_offset as u16)).unwrap();
	write!(stdout, "\x1b[38;5;7mScore:").unwrap();
	execute!(stdout, cursor::MoveTo(x_offset as u16, y_offset as u16+1)).unwrap();
	write!(stdout, "{}", score).unwrap();

	// Lines
	execute!(stdout, cursor::MoveTo(x_offset as u16, y_offset as u16+3)).unwrap();
	write!(stdout, "Lines:").unwrap();
	execute!(stdout, cursor::MoveTo(x_offset as u16, y_offset as u16+4)).unwrap();
	write!(stdout, "{}", lines).unwrap();

	// Level
	execute!(stdout, cursor::MoveTo(x_offset as u16, y_offset as u16+6)).unwrap();
	write!(stdout, "Level:").unwrap();
	execute!(stdout, cursor::MoveTo(x_offset as u16, y_offset as u16+7)).unwrap();
	write!(stdout, "{}", level).unwrap();

	// Next
	execute!(stdout, cursor::MoveTo(4+x_offset as u16, y_offset as u16+11)).unwrap();
	write!(stdout, "Next").unwrap();
	Ok(())
}

pub fn set_next_piece(preview : &mut SMatrix<u8, 6, 6>, player_obj : &CurrentObject) {

	clear_piece_preview(preview);

	let piece = player_obj.pieces[1]+1;

	if piece-1 == 0 || piece-1 == 1 {
		preview[(2, 2)] = piece;
	}
	else {
		preview[(2, 3)] = piece;
	}
	

	let mut x1 : u8 = 0;
	let mut x2 : u8 = 0;
	let mut x3 : u8 = 0;
	let mut y1 : u8 = 0;
	let mut y2 : u8 = 0;
	let mut y3 : u8 = 0;

	match piece-1 { 
		0 => { // L
			x1 = 1;
			y1 = 2;
			x2 = 3;
			y2 = 2;
			x3 = 1;
			y3 = 3;
		}
		1 => { // J
			x1 = 1;
			y1 = 2;
			x2 = 3;
			y2 = 2;
			x3 = 3;
			y3 = 3;
		}
		2 => { // I
			x1 = 1;
			y1 = 3;
			x2 = 3;
			y2 = 3;
			x3 = 4;
			y3 = 3;
		}
		3 => { // O
			x1 = 3;
			y1 = 3;
			x2 = 3;
			y2 = 2;
			x3 = 2;
			y3 = 2;
		}
		4 => { // Z
			x1 = 1;
			y1 = 2;
			x2 = 2;
			y2 = 2;
			x3 = 3;
			y3 = 3;
		}
		5 => { // S
			x1 = 1;
			y1 = 3;
			x2 = 2;
			y2 = 2;
			x3 = 3;
			y3 = 2;
		}
		6 => { // T
			x1 = 1;
			y1 = 2;
			x2 = 3;
			y2 = 2;
			x3 = 2;
			y3 = 2;
		}
		_ => {  }
	}
	preview[(x1 as usize, y1 as usize)] = piece;
	preview[(x2 as usize, y2 as usize)] = piece;
	preview[(x3 as usize, y3 as usize)] = piece;
}

fn clear_piece_preview(preview : &mut SMatrix<u8, 6, 6>) {
	for i in 0..6 {
		preview[(i, 0)] = 7;
		preview[(i, 5)] = 7;
		for j in 1..5 {
			preview[(i, j)] = 0;
		}
	}
	for i in 1..5 {
		preview[(0, i)] = 7;
		preview[(5, i)] = 7;
	}
}
