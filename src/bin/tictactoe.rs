use ndarray::Array2;

#[derive(Debug)] //Enable printing out complex data structures
struct TicTacToeGrid {
    cells: Array2<State>
}

impl TicTacToeGrid {
    fn new() -> Self {
        // Create a new 3x3 grid with all squares set to Empty
        Self {
            cells: Array2::from_elem((3, 3), State::Empty),
        }
    }
}

#[derive(Debug, Clone)] //Enable printing out complex data structures
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
    //player 1 starter
        //Velger en node ved å enten skrive inn kordinater eller rutenr (1-9)
        //Sjekker om noden er ledig
        //Sjekker om win
    //player 2 fortsetter
        //Repeat until win
}
