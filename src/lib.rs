use std::io::{stdin, stdout, Write};

#[derive(Debug)]
pub struct GameState{
    board: [[char; 3];3],
    pub winner: Option<char>,
    current_player: bool,
    move_count: u8,
}

impl GameState{
    pub fn new() -> GameState{
        return GameState{
            board: [[' '; 3];3],
            winner: None,
            current_player: true,
            move_count: 0,
        }
    }

    pub fn draw_board(&self){
        println!("    1.  2.  3.");
        for i in 0..3{
            println!("{}.  {} | {} | {} ",i+1, self.board[i][0], self.board[i][1], self.board[i][2]);
            if i == 2{
                break;
            }
            println!("   ---+---+---");
        }
    }

    pub fn put_mark(&mut self, w: usize, h: usize) -> Result<(), &'static str>{
        if w < 1 || h < 1 || w > 3 || h > 3{
            return Err("index out of bounds");
        }

        let w = w-1;
        let h = h-1;

        if self.board[w][h] == ' '{
            self.board[w][h] = if self.current_player {'X'} else {'O'};
            self.current_player = !self.current_player;
            self.move_count += 1;
            return Ok(());
        }
        Err("the spot is not free")
    }

    pub fn check_for_winner(&mut self){
        if self.move_count >= 9 {self.winner = Some(' ')}
        for i in 0..3{
            if self.board[0][i] == self.board[1][i] &&
                self.board[0][i] == self.board[2][i] &&
                self.board[0][i] != ' ' {
                self.winner = Some(self.board[0][i]);
            }
            if self.board[i][0] == self.board[i][1] &&
                self.board[i][0] == self.board[i][2] &&
                self.board[i][0] != ' ' {
                self.winner = Some(self.board[i][0]);
            }
            if self.board[0][0] == self.board[1][1] &&
                self.board[0][0] == self.board[2][2] &&
                self.board[0][0] != ' ' {
                self.winner = Some(self.board[0][0]);
            }
            if self.board[0][2] == self.board[1][1] &&
                self.board[0][2] == self.board[2][0] &&
                self.board[0][2] != ' ' {
                self.winner = Some(self.board[0][2]);
            }
        }
    }
}

pub fn ask_for_input() -> Result<u8, core::num::ParseIntError>{
    let mut s = String::new();
    stdout().flush().expect("flush failed!");
    stdin().read_line(&mut s).expect("did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    s.parse::<u8>()
}
