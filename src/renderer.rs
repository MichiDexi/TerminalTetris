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
	
	render(obj, map, x_offset, y_offset)?;
	update_border(x_offset, y_offset)?;
	update_display(x_offset, y_offset, level, score, lines)?;
	update_piece_preview(x_offset, y_offset, obj);

	Ok(())
}

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
