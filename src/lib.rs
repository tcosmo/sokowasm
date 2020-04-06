mod utils;

extern crate web_sys;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, sokowasm!");
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BackgroundElementType {
    Nothing = 0,
    Wall = 1,
    Goal = 2
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ForegroundElementType {
    Player = 0,
    Crate = 1,
}

#[wasm_bindgen]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ForegroundElement {
    x: i32,
    y: i32,
    element_type: ForegroundElementType
}

#[wasm_bindgen]
impl ForegroundElement {
    pub fn element_type(&self) -> ForegroundElementType {
        self.element_type
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    background: Vec<BackgroundElementType>,
    foreground: Vec<ForegroundElement>,
    player_id: usize,
    number_crates_ok: i32
}

struct Level {
    width: i32,
    height: i32,
    content: Vec<String>,
    player_id: usize
}

impl Level {

    pub fn new(level_string: &str) -> Level {
        let split: Vec<String> = level_string.split("\n").map(|s| s.to_string()).collect(); 
        let width = split[0].len() as i32;
        let height = split.len() as i32;

        Level {
            width,
            height,
            content: split,
            player_id: 0
        }
    }

    fn convert_background_char(char_elem: char) -> BackgroundElementType {
        match char_elem {
            ' ' => BackgroundElementType::Nothing,
            '#' => BackgroundElementType::Wall,
            '.' => BackgroundElementType::Goal,
            _ => BackgroundElementType::Nothing //Todo error handling
        }
    }

    pub fn get_background(&self) -> Vec<BackgroundElementType> {
        (0..(self.width*self.height))
                .map(|i| {
                    Level::convert_background_char(self.get_char_1D(i as usize))
                })
                .collect()
    }

    pub fn get_foreground(&mut self) -> Vec<ForegroundElement> {
        let mut foreground: Vec<ForegroundElement> = Vec::new();
        for i in 0..(self.width*self.height) {
            let (x,y) = self.get_2D_from_1D(i as usize);
            
            if self.get_char_1D(i as usize) == '@' {
                self.player_id = foreground.len();

                foreground.push(ForegroundElement {
                    x: x as i32,
                    y: y as i32,
                    element_type: ForegroundElementType::Player
                });
            }

            if self.get_char_1D(i as usize) == '$' {
                foreground.push(ForegroundElement {
                    x: x as i32,
                    y: y as i32,
                    element_type: ForegroundElementType::Crate
                });
            }
        }
        foreground
    }

    fn get_char_2D(&self, x: usize, y: usize) -> char {
        self.content[y].as_bytes()[x] as char
    }

    fn get_2D_from_1D(&self, i: usize) -> (usize, usize) {
        (i%(self.width as usize),i/(self.width as usize))
    }

    fn get_char_1D(&self, i: usize) -> char {
        let (x,y) = self.get_2D_from_1D(i);
        self.get_char_2D(x,y)
    }
}

const LEVEL3: &str = "#########\n##  #   #\n#.$.  $ #\n# #  ## #\n# @$.$. #\n#########";

impl Universe {

    fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn get_1D_from_2D(&self, x: i32, y: i32) -> usize {
        (y*self.width + x) as usize
    }

    fn crate_at_pos(&self, x: i32, y: i32, i_crate: &mut usize ) -> bool {
        for i in 0..self.foreground.len() {
            if (self.foreground[i].x, self.foreground[i].y ) == (x,y) {
                *i_crate = i;
                return true;
            }
        }
        return false;
    }

}


#[wasm_bindgen]
impl Universe {

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn background(&self) -> *const BackgroundElementType {
        self.background.as_ptr()
    }

    pub fn foreground_size(&self) -> usize {
        self.foreground.len()
    }

    pub fn get_foreground_elem(&self, i: usize) -> ForegroundElement {
        self.foreground[i]
    }

    pub fn number_crates_ok(&self) -> i32 {
        self.number_crates_ok
    }

    pub fn from_level_const() -> Universe {
        Universe::from_level(LEVEL3)
    }

    pub fn from_level(level_string: &str) -> Universe {
        
        let mut level = Level::new(level_string);
        let width = level.width;
        let height = level.height;
        //web_sys::console::log_1(&format!("{} {}", width, height).into());
        let background: Vec<BackgroundElementType> = level.get_background();

        let foreground: Vec<ForegroundElement> = level.get_foreground();
        let player_id = level.player_id;


        Universe {
            width,
            height,
            background,
            foreground,
            player_id,
            number_crates_ok: 0
        }
    }

    pub fn get_background_2D(&self, x: i32, y: i32) -> BackgroundElementType {
        //web_sys::console::log_1(&format!("{}", self.get_1D_from_2D(x,y)).into());
        self.background[self.get_1D_from_2D(x,y)]
        //self.background[0]
    }

    pub fn move_player(& mut self, dx: i32, dy: i32) -> () {
        let player = self.foreground[self.player_id].clone();

        let (new_x, new_y) = (player.x + dx, player.y + dy);
    
        if !self.is_valid(new_x,new_y) {
            return ();
        }

        let next_type = self.get_background_2D(new_x,new_y);

        if next_type == BackgroundElementType::Wall {
            return ();
        }

        let i_crate: &mut usize = &mut 0; 
        if self.crate_at_pos(new_x, new_y, i_crate) {
            
            let (new_new_x, new_new_y) = (new_x+dx, new_y+dy);
            if !self.is_valid(new_new_x,new_new_y) {
                return ();
            }

            let next_next_type = self.get_background_2D(new_new_x,new_new_y);

            if next_next_type == BackgroundElementType::Wall {
                return ();
            }

            let i_crate_crate: &mut usize = &mut 0;
            if self.crate_at_pos(new_new_x,new_new_y,i_crate_crate) {
                return ();
            }
            
            web_sys::console::log_1(&format!("HERE {}", next_type as u32).into());
            if next_type == BackgroundElementType::Goal {
                web_sys::console::log_1(&format!("HEREE {}", next_type as u32).into());
                self.number_crates_ok -= 1;
            }

            if next_next_type == BackgroundElementType::Goal {
                self.number_crates_ok += 1;
            }

            self.foreground[*i_crate].x += dx;
            self.foreground[*i_crate].y += dy;
        }

        self.foreground[self.player_id].x += dx;
        self.foreground[self.player_id].y += dy;
    }

    pub fn has_won(&self) -> bool {
        self.number_crates_ok == (self.foreground_size() as i32 - 1)
    }

}