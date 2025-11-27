use std::{
	io
};
use nalgebra::SMatrix;
use rand::prelude::*;

pub struct CurrentObject {
	// Center piece (everything rotates around this point)
	pub cx : u8,
	pub cy : u8,
	
	// Other pieces
	pub x1 : i8,
	pub y1 : i8,
	pub x2 : i8,
	pub y2 : i8,
	pub x3 : i8,
	pub y3 : i8,

	pub tick_delay : i8, // Ticks to wait to move down once
	pub exists : bool,
	pub exist_delay : i8,
	pub otype : u8,
	pub move_delay : u8,
	pub dead : bool,

	pub pieces : Vec<u8>,
}

impl CurrentObject {

	pub fn tick_obj(
		&mut self,
		matrix : &mut SMatrix<u8, 10, 18>,
		input : (i8, i8, bool, bool), // x, r, soft, hard
		score_vars : (&mut u8, &mut u32, &mut u32)) -> io::Result<bool>
	{

		// Object reset
		if !self.exists {
			if self.exist_delay <= 0 {
				self.reset_obj()?;
				if !self.try_move(matrix, 0, 1) &&
					!self.try_move(matrix, -1, 0) &&
					!self.try_move(matrix, 1, 0) {
					self.dead = true;
				}
				return Ok(true);
			}
			else {
				self.exist_delay -= 1;
				return Ok(false);
			}
		}
		
		// Vertical movement
		if input.3 {
			for _ in 0..20 {
				if self.try_move(matrix, 0, 1) {
					self.cy += 1;
				}
				else { break; }
			}
			self.tick_delay = 0;
		}
		if self.tick_delay <= 0 {
			if self.try_move(matrix, 0, 1) {
				self.cy += 1;
				self.tick_delay = match score_vars.0 {
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
				matrix[(self.cx as usize, self.cy as usize)] = self.otype+1;
				matrix[((self.x1+self.cx as i8) as usize, (self.y1+self.cy as i8) as usize)] = self.otype+1;
				matrix[((self.x2+self.cx as i8) as usize, (self.y2+self.cy as i8) as usize)] = self.otype+1;
				matrix[((self.x3+self.cx as i8) as usize, (self.y3+self.cy as i8) as usize)] = self.otype+1;
				self.exists = false;
				self.exist_delay = 10;
				CurrentObject::check_rows(matrix, score_vars.0, score_vars.1, score_vars.2);
			}
		}
		else if input.2 {
			self.tick_delay -= 3;
		}
		else {
			self.tick_delay -= 1;
		}

		// Player input
		if self.move_delay == 0 {
			if input.0 != 0 {
				if input.0 < 0 { // Right
					if self.try_move(matrix, 1, 0) {
						self.cx += 1;
						self.move_delay = 15;
					}
				}
				else { // Left
					if self.try_move(matrix, -1, 0) {
						self.cx -= 1;
						self.move_delay = 15;
					}
				}
			}
		}
		else {
			self.move_delay -= 1;
			if input.0 == 0 {
				self.move_delay = 0;
			}
		}

		if input.1 != 0 && self.try_rotate(matrix, input.1) {
			if input.1 > 0 { // Clockwise
				let mut temp : i8;

				temp = self.x1;
				self.x1 = self.y1;
				self.y1 = -temp;

				temp = self.x2;
				self.x2 = self.y2;
				self.y2 = -temp;

				temp = self.x3;
				self.x3 = self.y3;
				self.y3 = -temp;
			}
			else { // Counter clockwise
				let mut temp : i8;

				temp = self.x1;
				self.x1 = -self.y1;
				self.y1 = temp;

				temp = self.x2;
				self.x2 = -self.y2;
				self.y2 = temp;

				temp = self.x3;
				self.x3 = -self.y3;
				self.y3 = temp;
			}
		}
		Ok(false)
	}

	

	fn try_move(&self, matrix : &SMatrix<u8, 10, 18>, x : i8, y : i8) -> bool {

		let target_cx = self.cx as i8 +x;
		let target_cy = self.cy as i8 +y;
		let target_x1 = self.x1 +x+self.cx as i8;
		let target_y1 = self.y1 +y+self.cy as i8;
		let target_x2 = self.x2 +x+self.cx as i8;
		let target_y2 = self.y2 +y+self.cy as i8;
		let target_x3 = self.x3 +x+self.cx as i8;
		let target_y3 = self.y3 +y+self.cy as i8;


		// Out of bounds check
		if CurrentObject::check_out_of_bounds(target_cx, target_cy) ||
			CurrentObject::check_out_of_bounds(target_x1, target_y1) ||
			CurrentObject::check_out_of_bounds(target_x2, target_y2) ||
			CurrentObject::check_out_of_bounds(target_x3, target_y3)
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



	fn try_rotate(&self, matrix : &SMatrix<u8, 10, 18>, r : i8) -> bool {

		let target_cx = self.cx as i8;
		let target_cy = self.cy as i8;
		// TODO: Use expression based stuff thingy	
		let mut target_x1 = (-self.y1) +self.cx as i8;
		let mut target_y1 = (self.x1) +self.cy as i8;
		let mut target_x2 = (-self.y2) +self.cx as i8;
		let mut target_y2 = (self.x2) +self.cy as i8;
		let mut target_x3 = (-self.y3) +self.cx as i8;
		let mut target_y3 = (self.y3) +self.cy as i8;

		if r == 1 {
			target_x1 = (self.y1) +self.cx as i8;
			target_y1 = (-self.x1) +self.cy as i8;
			target_x2 = (self.y2) +self.cx as i8;
			target_y2 = (-self.x2) +self.cy as i8;
			target_x3 = (self.y3) +self.cx as i8;
			target_y3 = (-self.x3) +self.cy as i8;
		}

		// Out of bounds check
		if CurrentObject::check_out_of_bounds(target_cx, target_cy) ||
			CurrentObject::check_out_of_bounds(target_x1, target_y1) ||
			CurrentObject::check_out_of_bounds(target_x2, target_y2) ||
			CurrentObject::check_out_of_bounds(target_x3, target_y3)
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

	

	pub fn reset_obj(&mut self) -> io::Result<()> {

		let mut rng = rand::rng();
		let obj_type: usize = rng.random_range(0..7);
		self.pieces.push(obj_type as u8);
		self.pieces.remove(0);
	
		self.cx = 4;
		self.cy = 1;
		self.otype = self.pieces[0];
		self.exists = true;
	
		match self.pieces[0] {
			0 => { // L : 000
				self.set_positions(-1, 0, -1, 1, 1, 0);
				Ok(())
			}
			1 => { // J : 001
				self.set_positions(-1, 0, 1, 1, 1, 0);
				Ok(())
			}
			2 => { // I : 010
				self.set_positions(-1, 0, 2, 0, 1, 0);
				Ok(())
			}
			3 => { // O : 011
				self.set_positions(0, 1, 1, 1, 1, 0);
				Ok(())
			}
			4 => { // Z : 100
				self.set_positions(-1, 0, 0, 1, 1, 1);
				Ok(())
			}
			5 => { // S : 101
				self.set_positions(1, 0, 0, 1, -1, 1);
				Ok(())
			}
			6 => { // T : 110
				self.set_positions(-1, 0, 1, 0, 0, 1);
				Ok(())
			}
			_ => {
				Err(io::Error::other("how did you mess up this badly broo"))
			}
		}
	}



	fn set_positions(&mut self, x1 : i8, y1 : i8, x2 : i8, y2 : i8, x3 : i8, y3 : i8) {
		self.x1 = x1;
		self.y1 = y1;
		self.x2 = x2;
		self.y2 = y2;
		self.x3 = x3;
		self.y3 = y3;
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
				CurrentObject::clear_row(map, i);
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
}
