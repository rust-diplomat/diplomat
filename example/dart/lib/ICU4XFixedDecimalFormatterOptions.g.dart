// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _ICU4XFixedDecimalFormatterOptionsFfi extends ffi.Struct {
  @ffi.Int32()
  external int groupingStrategy;
  @ffi.Bool()
  external bool someOtherConfig;
}

final class ICU4XFixedDecimalFormatterOptions {
  final _ICU4XFixedDecimalFormatterOptionsFfi _underlying;

  // ignore: unused_element
  ICU4XFixedDecimalFormatterOptions._(this._underlying);

  factory ICU4XFixedDecimalFormatterOptions() {
    final pointer = ffi2.calloc<_ICU4XFixedDecimalFormatterOptionsFfi>();
    final result = ICU4XFixedDecimalFormatterOptions._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  ICU4XFixedDecimalGroupingStrategy get groupingStrategy =>
      ICU4XFixedDecimalGroupingStrategy.values[_underlying.groupingStrategy];
  set groupingStrategy(ICU4XFixedDecimalGroupingStrategy groupingStrategy) {
    _underlying.groupingStrategy = groupingStrategy.index;
  }

  bool get someOtherConfig => _underlying.someOtherConfig;
  set someOtherConfig(bool someOtherConfig) {
    _underlying.someOtherConfig = someOtherConfig;
  }

  factory ICU4XFixedDecimalFormatterOptions() {
    final result = _ICU4XFixedDecimalFormatterOptions_default();
    return ICU4XFixedDecimalFormatterOptions._(result);
  }
  // ignore: non_constant_identifier_names
  static final _ICU4XFixedDecimalFormatterOptions_default = _capi<
          ffi.NativeFunction<
              _ICU4XFixedDecimalFormatterOptionsFfi
                  Function()>>('ICU4XFixedDecimalFormatterOptions_default')
      .asFunction<_ICU4XFixedDecimalFormatterOptionsFfi Function()>(
          isLeaf: true);

  @override
  bool operator ==(Object other) =>
      other is ICU4XFixedDecimalFormatterOptions &&
      other._underlying.groupingStrategy == _underlying.groupingStrategy &&
      other._underlying.someOtherConfig == _underlying.someOtherConfig;

  @override
  int get hashCode => Object.hashAll([
        _underlying.groupingStrategy,
        _underlying.someOtherConfig,
      ]);
}
