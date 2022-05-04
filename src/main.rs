use std::{thread, time};


fn main() {
    let mut world = Game::new();
    world.set_cell_state(3, 3, State::Alive);
    world.set_cell_state(4, 3, State::Alive);
    world.set_cell_state(5, 3, State::Alive);
    world.set_cell_state(3, 6, State::Alive);
    world.set_cell_state(4, 6, State::Alive);
    world.set_cell_state(5, 6, State::Alive);
    world.set_cell_state(7, 8, State::Alive);
    world.set_cell_state(8, 8, State::Alive);
    world.set_cell_state(9, 8, State::Alive);
    world.set_cell_state(7, 11, State::Alive);
    world.set_cell_state(8, 11, State::Alive);
    world.set_cell_state(9, 11, State::Alive);
    for _ in 0..200{
        print!("\x1B[2J\x1B[1;1H");
        println!("{}",world.draw());
        world.generation();
        thread::sleep(time::Duration::from_millis(500));
    }
}

#[derive(Clone, Copy, Debug)]
enum State{
    Dead,
    Alive
}

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
        for _ in 1..16{
            let mut construction :Vec<Cell> = Vec::new();
            for _ in 1..16{
                let mut x = Vec::from([Cell::new()]);
                construction.append(&mut x);
            }
            cells.append(&mut Vec::from([construction]));
        }
        Game{cells}
    }

    fn from(width:u8, height:u8) -> Game{
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
