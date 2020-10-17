use std::fmt::Debug;
#[derive(Debug)]
pub struct Player(TTTShape, String);
impl Default for Player {
    fn default() -> Self {
        Player(TTTShape::Blank, "None".into())
    }
}
impl Player {
    pub fn shape(&self) -> &TTTShape {
        &self.0
    }
    pub fn name(&self) -> &String {
        &self.1
    }
}
impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name(), self.shape())
    }
}

#[derive(Debug)]
pub enum TTTError {
    IndexOutOfRange,
    ShapeAlreadyPlaced,
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TTTShape {
    X = 0,
    O = 1,
    Blank = 2,
}
impl From<usize> for TTTShape {
    fn from(num: usize) -> Self {
        match num {
            0 => Self::X,
            1 => Self::O,
            _ => Self::Blank,
        }
    }
}
impl std::fmt::Display for TTTShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::X => "X",
            Self::O => "O",
            Self::Blank => " ",
        };

        write!(f, "{}", name)
    }
}
impl Default for TTTShape {
    fn default() -> Self {
        Self::Blank
    }
}
pub struct TTTEngine {
    added_player_index: u8,
    current_player_turn: u8,
    players: [Player; 2],
    board: Vec<Vec<TTTShape>>,
}
impl TTTEngine {
    pub fn new() -> Self {
        TTTEngine {
            current_player_turn: 0,
            added_player_index: 0,
            players: Default::default(),
            board: vec![vec![Default::default(); 3]; 3],
        }
    }
    pub fn get_players(&self) -> &[Player] {
        &self.players
    }
    pub fn reset_game(&mut self) {
        self.board = vec![vec![Default::default(); 3]; 3];
        self.current_player_turn = 0;
    }
    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player_turn as usize]
    }
    pub fn reset_players(&mut self) {
        self.players = Default::default();
        self.added_player_index = 0;
    }
    pub fn get_board(&self) -> &Vec<Vec<TTTShape>> {
        &self.board
    }

    pub fn next_turn(&mut self, x: usize, y: usize) -> Result<(), TTTError> {
        let board_len = self.board.len();
        //Before placing shape, check if it is valid.
        if x >= board_len && y >= self.board[y].len() {
            return Err(TTTError::IndexOutOfRange);
        } else if self.board[y][x] != TTTShape::Blank {
            return Err(TTTError::ShapeAlreadyPlaced);
        }
        //Set board at coordinates [y][x] to the shape of the current players turn
        self.board[y][x] = *self.get_current_player().shape();

        self.current_player_turn = if self.current_player_turn == 1 { 0 } else { 1 };
        Ok(())
    }
    pub fn board_to_string(&self) -> String {
        let mut output = String::from(" 0|1|2 \n");
        let divider = "|";
        for (index, row) in (&self.board).iter().enumerate() {
            let mut line = String::from("|");
            for shape in row {
                line += &format!("{}", shape);
                line += divider;
            }
            line += &format!("{}\n", index);
            output += &line;
        }
        output
    }
    pub fn complete(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|shape| *shape != TTTShape::Blank))
    }
    fn eq_diagonal<T: PartialEq + Copy>(
        &self,
        predicate: impl Fn(T) -> bool,
        collection: &Vec<Vec<T>>,
    ) -> bool {
        let mut i = 0;
        collection
            .iter()
            .all(|collection_val| predicate((collection_val[i], i += 1).0))
    }
    fn get_player_from_shape(&self, shape: &TTTShape) -> &Player {
        &self.players[*shape as usize]
    }
    fn check_rows(&self) -> Option<TTTShape> {
        let mut temp: TTTShape;
        for row in &self.board {
            temp = row[0];
            let row_matches = temp != TTTShape::Blank && row.iter().all(|e| *e == temp);
            if row_matches {
                return Some(temp);
            }
        }
        None
    }
    fn check_columns(&self) -> Option<TTTShape> {
        let board_len = self.board.len();
        //Can use same length as grid is symmetrical
        for i in 0..board_len {
            let temp = self.board[0][i];
            if temp != TTTShape::Blank && (0..board_len).all(|j| self.board[j][i] == temp) {
                return Some(temp);
            }
        }

        None
    }
    fn check_diagonal(&self) -> Option<TTTShape> {
        let temp = self.board[0][0];
        if temp != TTTShape::Blank && self.eq_diagonal(|shape| shape == temp, &self.board) {
            Some(temp)
        } else {
            None
        }
    }
    fn check_diagonal_rev(&self) -> Option<TTTShape> {
        let temp = &self.board[0][self.board.len() - 1];
        if *temp != TTTShape::Blank && {
            let mut board = self.board.to_vec();
            board.reverse();
            self.eq_diagonal(|shape| shape == *temp, &board)
        } {
            Some(temp.clone())
        } else {
            None
        }
    }

    pub fn check_winner(&self) -> Option<&Player> {
        let winner = self
            .check_rows()
            .or_else(|| self.check_columns())
            .or_else(|| self.check_diagonal())
            .or_else(|| self.check_diagonal_rev());

        if let Some(winner) = winner {
            Some(self.get_player_from_shape(&winner))
        } else {
            None
        }
    }
    pub fn add_player(&mut self, name: &str) -> Result<(), TTTError> {
        //if less than 2 we can increment to next enumeration.
        // else we return and index out of rang error.
        if self.added_player_index < 2 {
            let shape = if self.added_player_index == 0 {
                TTTShape::X
            } else {
                TTTShape::O
            };

            //Set new player to new name
            self.players[self.added_player_index as usize] = Player(shape, name.into());

            //Increment next index
            self.added_player_index += 1;
        } else {
            return Err(TTTError::IndexOutOfRange);
        }
        Ok(())
    }
}
