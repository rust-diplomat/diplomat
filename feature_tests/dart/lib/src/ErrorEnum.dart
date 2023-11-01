typedef ErrorEnumFfi = int;

enum ErrorEnum {
  Foo._(0),
  Bar._(1);

  const ErrorEnum._(this._id);

  final int _id;
}

// These are not methods because we want to keep them package-private, and methods are either private or public
ErrorEnum ErrorEnumFromFfi(int id) =>
    ErrorEnum.values.firstWhere((value) => value._id == id);
ErrorEnumFfi ErrorEnumAsFfi(ErrorEnum t) => t._id;
