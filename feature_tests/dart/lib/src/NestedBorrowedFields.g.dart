// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _NestedBorrowedFieldsFfi extends ffi.Struct {
  external _BorrowedFieldsFfi fields;
  external _BorrowedFieldsWithBoundsFfi bounds;
  external _BorrowedFieldsWithBoundsFfi bounds2;
}

final class NestedBorrowedFields {
  BorrowedFields fields;
  BorrowedFieldsWithBounds bounds;
  BorrowedFieldsWithBounds bounds2;

  NestedBorrowedFields({required this.fields, required this.bounds, required this.bounds2});

  // ignore: unused_element
  // Internal constructor from FFI.
  NestedBorrowedFields._(_NestedBorrowedFieldsFfi underlying, core.List<Object> edge_x, core.List<Object> edge_y, core.List<Object> edge_z) :
    fields = BorrowedFields._(underlying.fields, []),
    bounds = BorrowedFieldsWithBounds._(underlying.bounds, [], [], []),
    bounds2 = BorrowedFieldsWithBounds._(underlying.bounds2, [], [], []);

  // ignore: unused_element
  // If this struct contains any slices, their lifetime-edge-relevant objects (typically _FinalizedArenas) will only
  // be constructed here, and can be appended to any relevant lifetime arrays here. append_array_for_<lifetime> accepts a list
  // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
  // output arrays. Null means that
  _NestedBorrowedFieldsFfi _pointer(ffi.Allocator temp, {core.List<core.List<Object>>? append_array_for_x, core.List<core.List<Object>>? append_array_for_y, core.List<core.List<Object>>? append_array_for_z}) {
    final pointer = temp<_NestedBorrowedFieldsFfi>();
    pointer.ref.fields = fields._pointer(temp);
    pointer.ref.bounds = bounds._pointer(temp);
    pointer.ref.bounds2 = bounds2._pointer(temp);
    return pointer.ref;
  }

  @override
  bool operator ==(Object other) =>
      other is NestedBorrowedFields &&
      other.fields == this.fields &&
      other.bounds == this.bounds &&
      other.bounds2 == this.bounds2;

  @override
  int get hashCode => Object.hashAll([
        this.fields,
        this.bounds,
        this.bounds2,
      ]);

  // ignore: unused element
  // Append all fields corresponding to lifetime `'x`
  // and lifetimes longer than it (Lifetimes: x, y)
  // This is all fields that may be borrowed from if borrowing `'x`
  core.List<Object> _fields_for_lifetime_x() {
    return [fields, bounds];
  }

  // ignore: unused element
  // Append all fields corresponding to lifetime `'y`
  core.List<Object> _fields_for_lifetime_y() {
    return [bounds];
  }

  // ignore: unused element
  // Append all fields corresponding to lifetime `'z`
  core.List<Object> _fields_for_lifetime_z() {
    return [bounds2];
  }
}