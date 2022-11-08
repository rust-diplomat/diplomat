import { Two } from "./Two";

/**
 */
export class One {

  /**
   */
  static transitivity(hold: One, nohold: One): One;

  /**
   */
  static cycle(hold: Two, nohold: One): One;

  /**
   */
  static many_dependents(a: One, b: One, c: Two, d: Two, nohold: Two): One;

  /**
   */
  static return_outlives_param(hold: Two, nohold: One): One;

  /**
   */
  static diamond_top(top: One, left: One, right: One, bottom: One): One;

  /**
   */
  static diamond_left(top: One, left: One, right: One, bottom: One): One;

  /**
   */
  static diamond_right(top: One, left: One, right: One, bottom: One): One;

  /**
   */
  static diamond_bottom(top: One, left: One, right: One, bottom: One): One;

  /**
   */
  static diamond_and_nested_types(a: One, b: One, c: One, d: One, nohold: One): One;

  /**
   */
  static implicit_bounds(explicit_hold: One, implicit_hold: One, nohold: One): One;

  /**
   */
  static implicit_bounds_deep(explicit_: One, implicit_1: One, implicit_2: One, nohold: One): One;
}
