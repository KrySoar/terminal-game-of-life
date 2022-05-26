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

        let mut cell_count = 0;
        for dy in [-1,0,1] {
            for dx in [-1,0,1] {
                if self.get_state(cell_x + dx, cell_y + dy) == State::Alive {
                        cell_count += 1;
                        if dx == 0 && dy == 0 {
                            cell_count -= 1;
                        }
                }
            }
        }
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

    let first_cell_x = map.width/2;
    let first_cell_y= map.height/2;

    let starting_pattern: &[(i16,i16)] = &[
        (0,0),
        (2,0),
        (2,1),
        (4,2),
        (4,3),
        (4,4),
        (6,3),
        (6,4),
        (6,5),
        (7,4),
    ];
    for &(dx,dy) in starting_pattern {
        map.set_state(first_cell_x+dx,first_cell_y-dy,State::Alive);
    }

    loop {
        clear_screen();
        map.next_gen();
        map.draw();

        //delay between each generation in milliseconds
        thread::sleep(time::Duration::from_millis(70));
    }
}

