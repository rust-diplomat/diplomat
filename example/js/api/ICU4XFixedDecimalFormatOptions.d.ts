import { ICU4XFixedDecimalGroupingStrategy } from "./ICU4XFixedDecimalGroupingStrategy";
import { ICU4XFixedDecimalSignDisplay } from "./ICU4XFixedDecimalSignDisplay";

/**
 */
export class ICU4XFixedDecimalFormatOptions {
  grouping_strategy: ICU4XFixedDecimalGroupingStrategy;
  sign_display: ICU4XFixedDecimalSignDisplay;

  /**
   */
  static default(): ICU4XFixedDecimalFormatOptions;
}
