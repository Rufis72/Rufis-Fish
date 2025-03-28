/** the following code is the class for how we store piece's positions.
the chess board is 64 squares, so we use a 64 bit number to represent each of those square
then, we have one of these 64 bit numbers for each piece. Meaning if a bit of the number is 0, there is none of that piece there. Otherwise, there is a piece there**/
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

pub struct Position {
    dark_pawn: BitBoard,
    dark_night: BitBoard,
    dark_bishop: BitBoard,
    dark_queen: BitBoard,
    dark_king: BitBoard,
    light_pawn: BitBoard,
    light_night: BitBoard,
    light_bishop: BitBoard,
    light_queen: BitBoard,
    light_king: BitBoard,

}

fn main() {
    println!("Hello, world!");
}
