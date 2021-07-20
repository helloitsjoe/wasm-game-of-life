//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_game_of_life;
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_game_of_life::{Universe, Cell};

// wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn set_live() {
  let mut u = Universe::new(5, 6);
  assert_eq!(u.get_live(0, 0), Cell::Dead);
  u.set_live(0, 0);
  assert_eq!(u.get_live(0, 0), Cell::Alive);
}

#[wasm_bindgen_test]
fn count_neighbors() {
  let mut u = Universe::new(5, 6);
  u.set_live(2, 2);
  u.set_live(3, 2);
  assert_eq!(u.live_neighbor_count(2, 2), 1);
  u.set_live(1, 2);
  assert_eq!(u.live_neighbor_count(2, 2), 2);
  u.set_live(2, 1);
  u.set_live(2, 3);
  assert_eq!(u.live_neighbor_count(2, 2), 4);
  u.set_live(1, 1);
  u.set_live(3, 1);
  u.set_live(3, 3);
  u.set_live(1, 3);
  assert_eq!(u.live_neighbor_count(2, 2), 8);
}

#[wasm_bindgen_test]
fn count_neighbors_edge() {
  let mut u = Universe::new(5, 5);
  assert_eq!(u.live_neighbor_count(0, 0), 0);
  u.set_live(1, 0);
  u.set_live(0, 1);
  u.set_live(1, 1);
  assert_eq!(u.live_neighbor_count(0, 0), 3);
  // Lower right
  assert_eq!(u.live_neighbor_count(4, 4), 0);
  // Surround lower right
  u.set_live(4, 3);
  u.set_live(3, 4);
  u.set_live(3, 3);
  assert_eq!(u.live_neighbor_count(4, 4), 3);
}

#[wasm_bindgen_test]
fn get_next_generation() {
  let mut u = Universe::new(5, 5);
  u.set_live(1, 2);
  u.set_live(2, 2);
  u.set_live(3, 2);
  u.tick();
  assert_eq!(u.get_live(2, 1), Cell::Alive);
  assert_eq!(u.get_live(2, 2), Cell::Alive);
  assert_eq!(u.get_live(2, 3), Cell::Alive);

  assert_eq!(u.get_live(1, 2), Cell::Dead);
  assert_eq!(u.get_live(3, 2), Cell::Dead);
}
