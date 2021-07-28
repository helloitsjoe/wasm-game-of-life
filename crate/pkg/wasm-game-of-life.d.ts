/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Cell {
  Dead,
  Alive,
}
/**
*/
export class Universe {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @param {boolean | undefined} empty_cells
* @returns {Universe}
*/
  static new(width: number, height: number, empty_cells?: boolean): Universe;
/**
* @returns {number}
*/
  width(): number;
/**
* @returns {number}
*/
  height(): number;
/**
* @returns {number}
*/
  cells(): number;
/**
* @returns {string}
*/
  render(): string;
/**
* @param {number} col
* @param {number} row
*/
  set_live(col: number, row: number): void;
/**
* @param {number} col
* @param {number} row
* @returns {number}
*/
  get_live(col: number, row: number): number;
/**
* @param {number} col
* @param {number} row
* @returns {number}
*/
  live_neighbor_count(col: number, row: number): number;
/**
*/
  tick(): void;
}
