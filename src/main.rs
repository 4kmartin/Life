use std::{thread, time};
use clap::Parser;

///Conway's Game of Life implemented in Rust! 🎺🎺🎺
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///a hexadecimal string where every two digits represent the coords of a living cell.
    ///for example, ff would mean that the cell at column 15, row 15 is alive
    #[clap(default_value = "030c131c232c434c535c636c838c939ca3acc3ccd3dce3ec")]
    seed: String,

    ///specify how many frames per second
    #[clap(short,long,default_value_t=1)]
    framerate:u8,

    ///specify the number of generations to simulate
    #[clap(short,long,default_value_t = 10)]
    generations: u8
}


fn main() {
    let args = Args::parse();
    let pause = 1000 / args.framerate as u16;
    let mut world = Game::new();
    world.initialize(args.seed);    
    for _ in 0..args.generations{
        print!("\x1B[2J\x1B[1;1H");
        println!("{}",world.draw());
        world.generation();
        thread::sleep(time::Duration::from_millis(pause as u64));
    }
}

///this enum allows a cell to know its state (ie. dead or alive)
#[derive(Clone, Copy, Debug)]
enum State{
    Dead,
    Alive
}

///a Cell is a single point that can be dead or alive.
/// 
/// the rules to determine if a living cell dies is if
/// 1. the cell has more than three live neighbors, or 
/// 2. the cell has less than two living neighbors
/// 
/// a dead cell can come to life if it has three living neighbors
#[derive(Clone, Copy)]
struct Cell {
    state:State,
    living_neighbors:u8
}

struct Game{
    cells: Vec<Vec<Cell>>
}

impl Cell{
    fn new() -> Cell{
        Cell{state:State::Dead, living_neighbors:0 }
    }
    
    fn get_state(&self)->State{
        self.state.clone()
    }

    fn set_state(&mut self, new_state:State){
        self.state = new_state;
    }

    fn add_live_neighbor(&mut self){
        self.living_neighbors += 1
    }

    fn clear_live_neighbors(&mut self){
        self.living_neighbors = 0
    }
}

impl Game {

    fn new() -> Game {
        let mut cells = Vec::new();
        for _ in 0..16{
            let mut construction :Vec<Cell> = Vec::new();
            for _ in 0..16{
                let mut x = Vec::from([Cell::new()]);
                construction.append(&mut x);
            }
            cells.append(&mut Vec::from([construction]));
        }
        Game{cells}
    }

    /* fn from(width:u8, height:u8) -> Game{
        let mut cells = Vec::new();
        for _ in 1..height{
            let mut construction :Vec<Cell> = Vec::new();
            for _ in 1..width{
                let mut x = Vec::from([Cell::new()]);
                construction.append(&mut x);
            }
            cells.append(&mut Vec::from([construction]));
        }
        Game{cells}
    }*/

    fn initialize(&mut self,seed:String){
        let mut coord:Vec<usize> = Vec::new();
        let a: String = seed.chars()
            .map(|i| "0".to_string() + i.to_string().as_str())
            .collect();
        let slice =  hex::decode(a).expect("Not Hexadecimal");
        for i in slice{
            coord.append(&mut vec!(i as usize));
            if coord.len() == 2{
                self.set_cell_state(coord[0], coord[1], State::Alive);
                coord = Vec::new();
            }
        }
    }

    fn draw(&self) -> String{
        let mut drawing = String::new();
        for row in &self.cells{
            for cell in row{
                match cell.get_state(){
                    State::Alive => drawing += "@ ",
                    State::Dead => drawing += ". "
                }
            }
            drawing += "\n"
        }
        drawing
    }

    fn get_neighbors(&mut self, row_number:usize, column_number:usize) ->Vec<Cell>{

        if row_number == 0{
            //case 1
            if column_number == 0{
                let neighbors = [
                    self.cells.get(row_number).unwrap().get(column_number+1).unwrap().clone(),
                    self.cells.get(row_number+1).unwrap().get(column_number).unwrap().clone(),
                    self.cells.get(row_number+1).unwrap().get(column_number+1).unwrap().clone()
                ];
                return Vec::from(neighbors)
            }
            //case 3
            else if column_number == self.cells[0].len()-1 {
                let neighbors = [
                    self.cells.get(row_number).unwrap().get(column_number-1).unwrap().clone(),
                    self.cells.get(row_number+1).unwrap().get(column_number-1).unwrap().clone(),
                    self.cells.get(row_number+1).unwrap().get(column_number).unwrap().clone()
                ];
                return Vec::from(neighbors)
            }
            //case 2
            else{
                let neighbors = [
                    self.cells.get(row_number).unwrap().get(column_number-1).unwrap().clone(),
                    self.cells.get(row_number).unwrap().get(column_number+1).unwrap().clone(),
                    self.cells.get(row_number+1).unwrap().get(column_number-1).unwrap().clone(),
                    self.cells.get(row_number+1).unwrap().get(column_number).unwrap().clone(),
                    self.cells.get(row_number+1).unwrap().get(column_number+1).unwrap().clone()
                ];
                return Vec::from(neighbors)
            }
        }
        else if row_number == self.cells.len()-1{
            //case 6
            if column_number == 0{
                let neighbors = [
                    self.cells.get(row_number-1).unwrap().get(column_number).unwrap().clone(),
                    self.cells.get(row_number-1).unwrap().get(column_number+1).unwrap().clone(),
                    self.cells.get(row_number).unwrap().get(column_number+1).unwrap().clone()
                ];
                return Vec::from(neighbors)
            }
            //case 8
            else if column_number == self.cells[0].len()-1{
                let neighbors = [
                    self.cells.get(row_number-1).unwrap().get(column_number-1).unwrap().clone(),
                    self.cells.get(row_number-1).unwrap().get(column_number).unwrap().clone(),
                    self.cells.get(row_number).unwrap().get(column_number-1).unwrap().clone()
                ];
                return Vec::from(neighbors)
            }
            //case 7
            else{
                let neighbors = [
                    self.cells.get(row_number-1).unwrap().get(column_number-1).unwrap().clone(),
                    self.cells.get(row_number-1).unwrap().get(column_number).unwrap().clone(),
                    self.cells.get(row_number-1).unwrap().get(column_number+1).unwrap().clone(),
                    self.cells.get(row_number).unwrap().get(column_number-1).unwrap().clone(),
                    self.cells.get(row_number).unwrap().get(column_number+1).unwrap().clone()
                ];
                return Vec::from(neighbors)
            }
        }
        // case 4
        else if column_number == 0{
            let neighbors = [
                self.cells.get(row_number-1).unwrap().get(column_number).unwrap().clone(),
                self.cells.get(row_number-1).unwrap().get(column_number+1).unwrap().clone(),
                self.cells.get(row_number).unwrap().get(column_number+1).unwrap().clone(), 
                self.cells.get(row_number+1).unwrap().get(column_number).unwrap().clone(),
                self.cells.get(row_number+1).unwrap().get(column_number+1).unwrap().clone()
            ];
            return Vec::from(neighbors)
        }
        // case 5
        else if column_number == self.cells[0].len()-1{
            let neighbors = [
                self.cells.get(row_number-1).unwrap().get(column_number-1).unwrap().clone(),
                self.cells.get(row_number-1).unwrap().get(column_number).unwrap().clone(),
                self.cells.get(row_number).unwrap().get(column_number-1).unwrap().clone(), 
                self.cells.get(row_number+1).unwrap().get(column_number-1).unwrap().clone(),
                self.cells.get(row_number+1).unwrap().get(column_number).unwrap().clone()
            ];
            return Vec::from(neighbors)
        }
        // case 9
        else{
            let neighbors = [
                self.cells.get(row_number-1).unwrap().get(column_number-1).unwrap().clone(),
                self.cells.get(row_number-1).unwrap().get(column_number).unwrap().clone(),
                self.cells.get(row_number-1).unwrap().get(column_number+1).unwrap().clone(),
                self.cells.get(row_number).unwrap().get(column_number-1).unwrap().clone(),
                self.cells.get(row_number).unwrap().get(column_number+1).unwrap().clone(),
                self.cells.get(row_number+1).unwrap().get(column_number-1).unwrap().clone(), 
                self.cells.get(row_number+1).unwrap().get(column_number).unwrap().clone(),
                self.cells.get(row_number+1).unwrap().get(column_number+1).unwrap().clone()
            ];
            return Vec::from(neighbors)
        }
        
    }

    fn generation(&mut self){
        let mut next_generation = self.cells.clone();
        for row_number in 0..self.cells.len(){
            for column_number in 0..self.cells[0].len(){
                let neighbors = self.get_neighbors(row_number, column_number);
                let current_cell = self.cells.get_mut(row_number).unwrap().get_mut(column_number).unwrap();
                let next_gen_cell = next_generation.get_mut(row_number).unwrap().get_mut(column_number).unwrap();
                for neighbor in neighbors{
                    match neighbor.get_state() {
                        State::Alive => {current_cell.add_live_neighbor();},
                        _ => ()
                    }
                }
                match current_cell.get_state() {
                    State::Alive => {
                        if current_cell.living_neighbors > 3{
                            next_gen_cell.set_state(State::Dead)
                        }
                        else if current_cell.living_neighbors < 2 {
                            next_gen_cell.set_state(State::Dead)
                        }
                    }
                    State::Dead => {
                        if current_cell.living_neighbors == 3 {
                            next_gen_cell.set_state(State::Alive)
                        }
                    }
                }
                current_cell.clear_live_neighbors()
            }
        }
        self.cells = next_generation
    }

    fn set_cell_state(&mut self, row:usize, column:usize, new_state:State){
        let target =self.cells.get_mut(row).unwrap().get_mut(column).unwrap();
        target.set_state(new_state);
       
    }
}
