use std::cmp;

pub struct Ai {}

impl Ai {
    pub fn new() -> Self {
        Self {}
    }

    // alphabeta(origin, depth, −∞, +∞, TRUE)

    pub fn best_move(&self, board: &crate::Board) -> crate::Cell {
        let player = board.player();

        let maximizing_player = player == crate::Player::X;

        self.alphabeta(
            &board,
            &mut Box::from(std::i32::MIN),
            &mut Box::from(std::i32::MAX),
            maximizing_player,
        )
        .1
        .unwrap()
        .latest_move
        .unwrap()
    }

    // function alphabeta(node, depth, α, β, maximizingPlayer) is
    //     if depth = 0 or node is a terminal node then
    //         return the heuristic value of node
    //     if maximizingPlayer then
    //         value := −∞
    //         for each child of node do
    //             value := max(value, alphabeta(child, depth − 1, α, β, FALSE))
    //             if value ≥ β then
    //                 break (* β cutoff *)
    //             α := max(α, value)
    //         return value
    //     else
    //         value := +∞
    //         for each child of node do
    //             value := min(value, alphabeta(child, depth − 1, α, β, TRUE))
    //             if value ≤ α then
    //                 break (* α cutoff *)
    //             β := min(β, value)
    //         return value

    fn alphabeta(
        &self,
        node: &crate::Board,
        alpha: &mut Box<i32>,
        beta: &mut Box<i32>,
        maximizing_player: bool,
    ) -> (i32, Option<crate::Board>) {
        if node.terminal() {
            return match node.winner() {
                crate::Player::X => (1, None),
                crate::Player::O => (-1, None),
                crate::Player::EMPTY => (0, None),
            };
        }

        let mut board = None;

        if maximizing_player {
            let mut value = std::i32::MIN;

            for child in node.possible_moves(crate::Player::X) {
                let test_value = self.alphabeta(&child, alpha, beta, false).0;

                if test_value > value {
                    value = test_value;
                    board = Some(child);
                }

                // if value >= **beta {
                //     break;
                // }

                **alpha = cmp::max_by(**alpha, value, |a, b| a.cmp(b));
            }

            return (value, board);
        } else {
            let mut value = std::i32::MAX;

            for child in node.possible_moves(crate::Player::O) {
                let test_value = self.alphabeta(&child, alpha, beta, true).0;

                if test_value < value {
                    value = test_value;
                    board = Some(child);
                }

                // if value <= **alpha {
                //     break;
                // }

                **beta = cmp::min_by(**beta, value, |a, b| a.cmp(b));
            }

            println!("{} => {}", **alpha, **beta);

            return (value, board);
        }
    }
}
