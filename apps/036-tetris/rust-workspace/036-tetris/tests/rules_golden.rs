//! Plan 005: exercise the generated Rust implementation directly.
//!
//! The fixture includes the generated component, so these assertions cover
//! the same `TetrisStore::on` code used by the Rust renderer.  They provide a
//! deterministic golden for spawn, movement, locking, compaction and all line
//! clear score tiers without duplicating the rules in a second language.

#[allow(dead_code)]
mod generated {
    include!("../src/main.rs");
}

use generated::{TetrisStore, TetrisStoreMsg};
use auto_lang::ui::Component;

fn fresh_playing() -> TetrisStore {
    let mut store = TetrisStore::new();
    store.on(TetrisStoreMsg::Start);
    store
}

#[test]
fn opening_and_lock_golden() {
    let mut store = fresh_playing();
    assert_eq!((store.piece, store.next_piece, store.px, store.py), (2, 0, 3, 0));
    assert_eq!((store.score, store.lines, store.level), (0, 0, 1));

    store.on(TetrisStoreMsg::MoveLeft);
    store.on(TetrisStoreMsg::Rotate);
    store.on(TetrisStoreMsg::SoftDrop);
    assert_eq!((store.rotation, store.px, store.py, store.score), (1, 2, 1, 1));

    store.on(TetrisStoreMsg::HardDrop);
    assert_eq!((store.py, store.score, store.pending_lock), (17, 33, true));
    store.on(TetrisStoreMsg::Tick);
    assert_eq!((store.piece, store.next_piece, store.lines, store.score), (0, 2, 0, 33));
    assert_eq!(store.board.iter().filter(|cell| **cell != 0).count(), 4);
}

#[test]
fn line_clear_score_and_compaction_golden() {
    for cleared in 1_i32..=4 {
        let mut store = fresh_playing();
        store.board = vec![0; 200];
        store.piece = 0;
        store.rotation = 1; // vertical I: four cells in one column
        store.px = 7; // rotation 1 uses x = 2, so x = 9 completes columns 0..8
        store.py = 16;
        store.pending_lock = true;
        for row in (20 - cleared)..20 {
            for col in 0..9 {
                store.board[(row * 10 + col) as usize] = 1;
            }
        }

        store.on(TetrisStoreMsg::Tick);
        let expected_score = [0, 100, 300, 500, 800][cleared as usize] as i32;
        assert_eq!(store.lines, cleared, "line count for {cleared}");
        assert_eq!(store.score, expected_score, "score for {cleared}");
        assert_eq!(store.feedback, format!("消除 {} 行", cleared));
        assert!(store.board[..(cleared as usize * 10)].iter().all(|cell| *cell == 0));
        assert_eq!(store.board.len(), 200);
    }
}
