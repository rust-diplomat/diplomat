typedef ICU4XFixedDecimalGroupingStrategyFfi = int;

enum ICU4XFixedDecimalGroupingStrategy {
  /// Auto grouping
  Auto._(0),

  /// No grouping
  Never._(1),

  /// Always group
  Always._(2),

  /// At least 2 groups
  Min2._(3);

  const ICU4XFixedDecimalGroupingStrategy._(this._id);

  final int _id;
}

// These are not methods because we want to keep them package-private, and methods are either private or public
ICU4XFixedDecimalGroupingStrategy ICU4XFixedDecimalGroupingStrategyFromFfi(
        int id) =>
    ICU4XFixedDecimalGroupingStrategy.values
        .firstWhere((value) => value._id == id);
ICU4XFixedDecimalGroupingStrategyFfi ICU4XFixedDecimalGroupingStrategyAsFfi(
        ICU4XFixedDecimalGroupingStrategy t) =>
    t._id;
