extern crate pleco;

use pleco::board::{Board, RandBoard};
use pleco::bot_prelude::*;
use pleco::tools::Searcher;

#[test]
fn test_all_bot() {
    for _x in 0..3 {
        let board: Board = RandBoard::default().one();
        RandomBot::best_move(board.shallow_clone(), 4);
        MiniMaxSearcher::best_move(board.shallow_clone(), 4);
        AlphaBetaSearcher::best_move(board.shallow_clone(), 4);
        ParallelMiniMaxSearcher::best_move(board.shallow_clone(), 4);
        JamboreeSearcher::best_move(board.shallow_clone(), 4);
    }
}

#[test]
fn test_bot_en_passant() {
    // taking with rook is fine (but brick on pipi), taking en passant leads to mate in 1
    let board = Board::from_fen("2r3k1/5ppp/8/1pP5/8/8/1R3PPP/6K1 w - b6 0 2").unwrap();
    
    let _move = AlphaBetaSearcher::best_move(board.shallow_clone(), 4);
    assert_eq!(_move.flag(), pleco::core::piece_move::BitMove::FLAG_EP);
    
    let _move = JamboreeSearcher::best_move(board.shallow_clone(), 4);
    assert_eq!(_move.flag(), pleco::core::piece_move::BitMove::FLAG_EP);
    
    let _move = MiniMaxSearcher::best_move(board.shallow_clone(), 4);
    assert_eq!(_move.flag(), pleco::core::piece_move::BitMove::FLAG_EP);
    
    let _move = ParallelMiniMaxSearcher::best_move(board.shallow_clone(), 4);
    assert_eq!(_move.flag(), pleco::core::piece_move::BitMove::FLAG_EP);
    
    let _move = RandomBot::best_move(board.shallow_clone(), 4);
    assert_eq!(_move.flag(), pleco::core::piece_move::BitMove::FLAG_EP);
}
