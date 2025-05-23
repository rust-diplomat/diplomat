// generated by diplomat-tool
// dart format off

part of 'lib.g.dart';

final class _FixedDecimalFormatterOptionsFfi extends ffi.Struct {
  @ffi.Int32()
  external int groupingStrategy;
  @ffi.Bool()
  external bool someOtherConfig;
}

final class FixedDecimalFormatterOptions {
  FixedDecimalGroupingStrategy groupingStrategy;
  bool someOtherConfig;

  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  // ignore: unused_element
  FixedDecimalFormatterOptions._fromFfi(_FixedDecimalFormatterOptionsFfi ffi) :
    groupingStrategy = FixedDecimalGroupingStrategy.values[ffi.groupingStrategy],
    someOtherConfig = ffi.someOtherConfig;

  // ignore: unused_element
  _FixedDecimalFormatterOptionsFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_FixedDecimalFormatterOptionsFfi>();
    struct.groupingStrategy = groupingStrategy.index;
    struct.someOtherConfig = someOtherConfig;
    return struct;
  }

  factory FixedDecimalFormatterOptions({FixedDecimalGroupingStrategy? groupingStrategy, bool? someOtherConfig}) {
    final result = _icu4x_FixedDecimalFormatterOptions_default_mv1();
    final dart = FixedDecimalFormatterOptions._fromFfi(result);
    if (groupingStrategy != null) {
      dart.groupingStrategy = groupingStrategy;
    }
    if (someOtherConfig != null) {
      dart.someOtherConfig = someOtherConfig;
    }
    return dart;
  }


  @override
  bool operator ==(Object other) =>
      other is FixedDecimalFormatterOptions &&
      other.groupingStrategy == groupingStrategy &&
      other.someOtherConfig == someOtherConfig;

  @override
  int get hashCode => Object.hashAll([
        groupingStrategy,
        someOtherConfig,
      ]);
}

@_DiplomatFfiUse('icu4x_FixedDecimalFormatterOptions_default_mv1')
@ffi.Native<_FixedDecimalFormatterOptionsFfi Function()>(isLeaf: true, symbol: 'icu4x_FixedDecimalFormatterOptions_default_mv1')
// ignore: non_constant_identifier_names
external _FixedDecimalFormatterOptionsFfi _icu4x_FixedDecimalFormatterOptions_default_mv1();

// dart format on
