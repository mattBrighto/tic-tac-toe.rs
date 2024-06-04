use std::io::{stdin, stdout, Write};

pub const MARKS :  [&str;8] = [
"X",
"O",
"M",
"A",
"T",
"R",
"I",
"G",
];

#[derive(PartialEq)]
pub struct Player{
    pub name: String,
    symbol: &'static str,
}

impl Player{
    pub fn new(n : String, m : &'static str) -> Player{
        Player{
            name: n,
            symbol: m,
        }
    }

    pub fn empty() -> Player{
        Player{
            name: "{empty}".to_string(),
            symbol: "O",
        }
    }
}

pub enum GameEnd<'a>{
    Draw,
    None,
    Win(&'a Player),
}

pub struct GameState<'a>{
    pub winner: GameEnd<'a>,
    pub current_player: usize,
    pub win_line_size: usize,
    move_count: u32,
    board_height: usize,
    board_width: usize,
    board: Vec<Vec<Option<&'a Player>>>,
}

impl<'a> GameState<'a>{
    pub fn new(w:usize, h:usize) -> GameState<'a>{
        return GameState{
            move_count: 0,
            winner: GameEnd::None,
            current_player: 0,
            board_width: w,
            board_height: h,
            board: vec![vec![None; w];h],
            win_line_size: 3,
        }
    }

    pub fn draw_board(&self){
        print!("    ");
        for c in 1..=self.board_width {
            print!(" {}. ", c);
        }
        print!("\n");

        for r in 0..self.board_height{
            let mut row = format!("{}. |", r+1);
            let mut divider = "   +".to_string();
            for c in 0..self.board_width{
                divider = divider + "---+";
                row = row + &format!(" {} ", match self.board[c][r]{
                    None => " ",
                    Some(ref p) => &p.symbol,
                })[..];

                if c == self.board_width-1{
                    row = row + "|\n";   
                    divider = divider + "\n";
                }else{
                    row = row + "|"
                }
            }
            print!("{divider}");
            print!("{row}");
            if r == self.board_height-1{
                print!("{divider}");
            }
        }
    }

    pub fn put_mark(&mut self, w: usize, h: usize, players : &'a Vec<Player>) -> Result<(), &'static str>{
        if w < 1 || h < 1 || w > self.board_width || h > self.board_height{
            return Err("index out of bounds");
        }

        let w = w-1;
        let h = h-1;

        if self.board[w][h] != None{
            return Err("The spot is not free");
        }
        self.board[w][h] = Some(&players[self.current_player]);

        if self.current_player+1 >= players.len(){
            self.current_player = 0;
        }else{
            self.current_player +=1;
        }

        self.move_count += 1;
        Ok(())
    }


    pub fn set_winner(&mut self){
        if self.move_count as usize >= self.board_height * self.board_width {
            self.winner = GameEnd::Draw;
        }

        for c in 0..self.board_width{
            for r in 0..self.board_height-self.win_line_size+1{
                
                let mut statement = self.board[c][r] != None;
                for i in 1..self.win_line_size{
                    statement = statement && self.board[c][r] == self.board[c][r+i as usize];
                }
                if statement {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                }
 
            }
        }

        for r in 0..self.board_height{
            for c in 0..self.board_width-self.win_line_size+1{
                
                let mut statement = self.board[c][r] != None;
                for i in 1..self.win_line_size{
                    statement = statement && self.board[c][r] == self.board[c+i as usize][r];
                }
                if statement {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                }
 
            }
        }

        //----------- diagonals -----------------------
        for row in 0..self.board_height-self.win_line_size+1{
            let mut c = 0;
            for r in row..self.board_height-self.win_line_size+1{
                //self.visualize_left(c, r);
                
                let mut statement = self.board[c][r] != None;
                for i in 1..self.win_line_size{
                    statement = statement && self.board[c][r] == self.board[c+i as usize][r+i as usize];
                }
                if statement {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                }

                c+=1;
                if c+self.win_line_size-1 >= self.board_width{
                    break;
                }
            }
        }

        for column in 1..self.board_width-self.win_line_size+1{
            let mut r = 0;
            for c in column..self.board_width-self.win_line_size+1{
                //self.visualize_left(c,r);
                
                let mut statement = self.board[c][r] != None;
                for i in 1..self.win_line_size{
                    statement = statement && self.board[c][r] == self.board[c+i as usize][r+i as usize];
                }
                if statement {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                }

                r+=1;
            }
        }

        for row in 0..self.board_height-self.win_line_size+1{
            let mut c = self.board_width-1;
            for r in row..self.board_height-self.win_line_size+1{
                //self.visualize_right(c,r);
                let mut statement = self.board[c][r] != None;
                for i in 1..self.win_line_size{
                    statement = statement && self.board[c][r] == self.board[c-i as usize][r+i as usize];
                }
                if statement {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                }
                c-=1;
                if c as i8 - self.win_line_size as i8 +2 <= 0{
                    break;
                }
            }
        }

        for column in (self.win_line_size-1..self.board_width-1).rev(){
            let mut r = 0;
            for c in (self.win_line_size-1..=column).rev(){
                //self.visualize_right(c,r);
                
                let mut statement = self.board[c][r] != None;
                for i in 1..self.win_line_size{
                    statement = statement && self.board[c][r] == self.board[c-i as usize][r+i as usize];
                }
                if statement {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                }

                r+=1;
            }
        }
    }

    fn  visualize_left(&self, c: usize, r: usize){
        let mut vis = GameState::new(self.board_width, self.board_height);
        let empty = Player::empty();
        vis.board[c][r] = Some(&empty);
        vis.board[c+1][r+1] = Some(&empty);
        vis.board[c+2][r+2] = Some(&empty);
        vis.draw_board();
    }
    fn  visualize_right(&self, c: usize, r: usize){
        let mut vis = GameState::new(self.board_width, self.board_height);
        let empty = Player::empty();
        vis.board[c][r] = Some(&empty);
        vis.board[c-1][r+1] = Some(&empty);
        vis.board[c-2][r+2] = Some(&empty);
        vis.draw_board();
    }

}

pub fn ask_for_input() -> String{
    let mut s = String::new();
    stdout().flush().expect("flush failed!");
    stdin().read_line(&mut s).expect("did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}
