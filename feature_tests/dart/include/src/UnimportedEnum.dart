typedef UnimportedEnumFfi = int;

enum UnimportedEnum {
  A._(0),
  B._(1),
  C._(2);

  const UnimportedEnum._(this._id);

  final int _id;
}

// These are not methods because we want to keep them package-private, and methods are either private or public
UnimportedEnum UnimportedEnumFromFfi(int id) =>
    UnimportedEnum.values.firstWhere((value) => value._id == id);
UnimportedEnumFfi UnimportedEnumAsFfi(UnimportedEnum t) => t._id;
