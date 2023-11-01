typedef AttrEnumFfi = int;

enum AttrEnum {
  A._(0),
  B._(1),
  C._(2);

  const AttrEnum._(this._id);

  final int _id;
}

// These are not methods because we want to keep them package-private, and methods are either private or public
AttrEnum AttrEnumFromFfi(int id) =>
    AttrEnum.values.firstWhere((value) => value._id == id);
AttrEnumFfi AttrEnumAsFfi(AttrEnum t) => t._id;
