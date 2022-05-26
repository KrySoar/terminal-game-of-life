#![allow(unused_variables)]
#![allow(dead_code)]

use std::process::Command;
use std::{thread,time};

#[derive(PartialEq,Eq)]
#[derive(Copy, Clone)]
enum State {
    Dead,
    Alive,
}

struct Grid {
    width: i16,
    height: i16,
    blocks: (char,char),
    spacing: String,
    cells: Vec<State>,
}

impl Grid {
    fn new(width: i16,height: i16,) -> Grid {
        Grid {
            width,
            height,
            blocks: ('░','█'), // ('DEAD_CELL','ALIVE_CELL')
            //blocks: ('□','■'),//if you use these blocks, set the spacing to 1: (" ") instead of 0: ("") 
            spacing: String::from(""),
            cells: vec![State::Dead; (height*width) as usize],
        }
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index =  y * self.width + x;

                match self.cells[index as usize] {
                    State::Dead => print!("{0}{1}",self.blocks.0,self.spacing),
                    State::Alive => print!("{0}{1}",self.blocks.1,self.spacing),
                }
            }
            print!("\n");
        }
    }

    fn set_state(&mut self,x: i16,y: i16,input_state: State) {
        let index =  y * self.width + x;//convert 2D positioning to 1D index
        self.cells[index as usize] = input_state;
    }

    fn get_state(&self,x: i16,y: i16) -> State {
        if (x < 0 || y < 0) || (x >= self.width as i16 || y >= self.height as i16) {
            return State::Dead;
        } else {
            let index =  y * self.width + x;//convert 2D positioning to 1D index
            self.cells[index as usize]
        }
    }

    fn count_neighbors(&self,cell_x: i16,cell_y: i16) -> i16 {
        let x = cell_x;
        let y = cell_y;
        let cell_state = self.get_state(x as i16  ,y as i16);

        let upper_left = self.get_state(x-1,y-1);
        let upper = self.get_state(x,y-1);
        let upper_right = self.get_state(x+1,y-1);

        let middle_left = self.get_state(x-1,y);
        let middle_right = self.get_state(x+1,y);

        let bottom_left = self.get_state(x-1,y+1);
        let bottom = self.get_state(x,y+1);
        let bottom_right = self.get_state(x+1,y+1);

        let mut cell_count = 0;

        if upper_left == State::Alive { cell_count += 1; }
        if upper == State::Alive { cell_count += 1; }
        if upper_right == State::Alive { cell_count += 1; }

        if middle_left == State::Alive { cell_count += 1; }
        if middle_right == State::Alive { cell_count += 1; }

        if bottom_left == State::Alive { cell_count += 1; }
        if bottom == State::Alive { cell_count += 1; }
        if bottom_right == State::Alive { cell_count += 1; }

        cell_count
    }

    fn next_gen(&mut self) {
        let mut newvec: Vec<State> = Vec::new();
        for z in 0..(self.height*self.width) {

            if self.cells[z as usize] == State::Alive{
                newvec.push(State::Alive);
            } else if self.cells[z as usize] == State::Dead {
                newvec.push(State::Dead);
            }

        }

        for y in 0..self.height as i16 {
            for x in 0..self.width as i16 {
                let nb_count = self.count_neighbors(x,y);
                let index = y *self.width + x;

                if self.get_state(x, y) == State::Alive {
                    if nb_count != 2 && nb_count != 3 {
                        newvec[index as usize] = State::Dead;
                    } else {
                        newvec[index as usize] = State::Alive;
                    }
                } else if  self.get_state(x, y) == State::Dead {
                    if nb_count == 3 {
                        newvec[index as usize] = State::Alive;
                    }
                }
            }
        }
        self.cells = newvec;
    }
}

fn clear_screen() {
    if cfg!(unix) {
        Command::new("clear").status().unwrap();
    } else if cfg!(windows) {
        Command::new("cls").status().unwrap();
    }
}

fn main() {
    let mut map = Grid::new(100,50); //default is (100,150) you can do more if you zoom out the terminal ([ctrl]'-') or ([ctrl][shift]'+')
    map.draw();

    let first_cell_x = map.width/2;
    let first_cell_y= map.height/2;

    //setting the first generation to an "infinite" pattern
    map.set_state(first_cell_x,first_cell_y,State::Alive);
    map.set_state(first_cell_x+2,first_cell_y,State::Alive);
    map.set_state(first_cell_x+2,first_cell_y-1,State::Alive);
    
    map.set_state(first_cell_x+4,first_cell_y-2,State::Alive);
    map.set_state(first_cell_x+4,first_cell_y-3,State::Alive);
    map.set_state(first_cell_x+4,first_cell_y-4,State::Alive);
    
    map.set_state(first_cell_x+6,first_cell_y-3,State::Alive);
    map.set_state(first_cell_x+6,first_cell_y-4,State::Alive);
    map.set_state(first_cell_x+6,first_cell_y-5,State::Alive);

    map.set_state(first_cell_x+7,first_cell_y-4,State::Alive);

    loop {
        clear_screen();
        map.next_gen();
        map.draw();

        //delay between each generation in milliseconds
        thread::sleep(time::Duration::from_millis(500));
    }
}

