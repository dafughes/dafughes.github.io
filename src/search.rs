use std::{
    cmp::Ordering,
    sync::{atomic::AtomicBool, Arc},
    vec,
};

use crate::{
    board::Board,
    color::Color,
    eval::{self, evaluate, mvv_lva, Score, DRAW, INF, MATE},
    moves::{generate_moves, Move, MoveKind, Movelist},
};

#[derive(Debug)]
pub struct SearchParams {
    pub wtime: u32,
    pub btime: u32,
    pub winc: u32,
    pub binc: u32,
    pub movestogo: u32,
    pub depth: u32,
    pub nodes: u32,
    pub mate: u32,
    pub movetime: u32,
    pub infinite: bool,
}

pub fn negamax(board: &Board, depth: usize, depth_left: usize) -> Score {
    let moves = generate_moves(board);

    if depth_left == 0 {
        if moves.is_empty() {
            if board.is_in_check() {
                return -MATE + (depth as i32);
            } else {
                return DRAW;
            }
        } else {
            return evaluate(board);
        }
    } else {
        if moves.is_empty() {
            if board.is_in_check() {
                return -MATE + (depth as i32);
            } else {
                return DRAW;
            }
        }
    }

    let mut max = -INF;

    for mv in &moves {
        let score = -negamax(&board.do_move(mv), depth + 1, depth_left - 1);
        if score > max {
            max = score;
        }
    }

    max
}

pub fn negamax_alphabeta(
    board: &Board,
    mut alpha: Score,
    beta: Score,
    depth: usize,
    depth_left: usize,
) -> Score {
    let moves = generate_moves(board);

    if depth_left == 0 {
        if moves.is_empty() {
            if board.is_in_check() {
                return -MATE + (depth as i32);
            } else {
                return DRAW;
            }
        } else {
            // return evaluate(board);
            return quiescence_search(board, alpha, beta);
        }
    } else {
        if moves.is_empty() {
            if board.is_in_check() {
                return -MATE + (depth as i32);
            } else {
                return DRAW;
            }
        }
    }

    for mv in &moves {
        let score =
            -negamax_alphabeta(&board.do_move(mv), -beta, -alpha, depth + 1, depth_left - 1);
        if score >= beta {
            return beta;
        }

        if score > alpha {
            alpha = score;
        }
    }

    return alpha;
}

pub fn quiescence_search(board: &Board, mut alpha: Score, beta: Score) -> Score {
    let standing_pat = evaluate(board);

    if standing_pat >= beta {
        return beta;
    }
    if alpha < standing_pat {
        alpha = standing_pat;
    }

    let moves = generate_moves(board);

    for mv in &moves {
        match mv.kind() {
            MoveKind::Cap | MoveKind::EnPassant | MoveKind::PromCap(_) => {
                let score = -quiescence_search(&board.do_move(mv), -beta, -alpha);

                if score >= beta {
                    return beta;
                }
                if score > alpha {
                    alpha = score;
                }
            }
            _ => continue,
        }
    }

    return alpha;
}

pub fn search(params: SearchParams, board: &Board, stopflag: Arc<AtomicBool>) -> Move {
    let moves = generate_moves(board);

    if params.depth == 0 {
        return Move::null();
    }

    let mut max = -INF;
    let mut bestmove = Move::null();

    for mv in &moves {
        if stopflag.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }

        let score = -negamax_alphabeta(&board.do_move(mv), -INF, INF, 1, params.depth as usize - 1);
        println!("{}: {}", mv, score);
        if score > max {
            max = score;
            bestmove = mv;
        }
    }

    bestmove
}
