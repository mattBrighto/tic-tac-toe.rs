use tic_tac_toe::{ask_for_input, GameState};

fn main() {

    let mut gm = GameState::new();

    loop{
        gm.draw_board();
        print!("row: ");
        let row : u8 = match ask_for_input(){
            Err(e) => {println!("parsing error : {e}");continue;}
            Ok(n) => n,
        };
        print!("column: ");
        let column : u8 = match ask_for_input(){
            Err(e) => {println!("parsing error : {e}");continue;}
            Ok(n) => n,
        };
        match gm.put_mark(row.into(), column.into()){
            Err(e) => {println!("parsing error : {e}");continue;}
            Ok(_) => (),
        }
        gm.check_for_winner();
        match gm.winner{
            Some(c) => {
                gm.draw_board();
                if c != ' '{
                    println!("ladies and gentleman. the winner iss {}", c);
                }else{
                    println!("draw");
                }
                break;
            }
            None => continue,
        }
    }
}
