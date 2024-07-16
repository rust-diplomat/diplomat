import { FixedDecimalGroupingStrategy } from "./FixedDecimalGroupingStrategy";

/**
 */
export class FixedDecimalFormatterOptions {
  grouping_strategy: FixedDecimalGroupingStrategy;
  some_other_config: boolean;

  /**
   */
  static default(): FixedDecimalFormatterOptions;
}
