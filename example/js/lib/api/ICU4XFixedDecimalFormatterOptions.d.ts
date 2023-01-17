import { ICU4XFixedDecimalGroupingStrategy } from "./ICU4XFixedDecimalGroupingStrategy";

/**
 */
export class ICU4XFixedDecimalFormatterOptions {
  grouping_strategy: ICU4XFixedDecimalGroupingStrategy;
  some_other_config: boolean;

  /**
   */
  static default(): ICU4XFixedDecimalFormatterOptions;
}
