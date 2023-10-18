#[derive(Debug)] //Enable printing out complex data structures
struct TicTacToeGrid {
    cells: [[State; 3]; 3]
}

impl TicTacToeGrid {
    fn new() -> Self {
        // Create a new 3x3 grid with all squares set to Empty
        Self {
            cells: [[State::Empty; 3]; 3],
        }
    }
}

impl TicTacToeGrid {
    fn pick_space() -> bool{
        true
    }
}

#[derive(Debug, Clone, Copy)] //Enable printing out complex data structures
enum State {
    Empty,
    X,
    O,
}

enum GameProgress {
    Finished,
    OnGoing,
}

fn main() {
    let mut grid: TicTacToeGrid = TicTacToeGrid::new();
    grid.cells[1][1] = State::O


    //player 1 starter
        //Velger en node ved å enten skrive inn kordinater eller rutenr (1-9)
        //Sjekker om noden er ledig
        //Sjekker om win
    //player 2 fortsetter
        //Repeat until win
}

fn draw_grid(grid: TicTacToeGrid) {

}