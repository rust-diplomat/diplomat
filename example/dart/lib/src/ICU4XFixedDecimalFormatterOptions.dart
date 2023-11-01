import 'ICU4XFixedDecimalGroupingStrategy.dart';
import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';


class ICU4XFixedDecimalFormatterOptionsFfi extends ffi.Struct {
    @ffi.Int32()
    external ICU4XFixedDecimalGroupingStrategyFfi groupingStrategy;
    @ffi.Bool()
    external bool someOtherConfig;
}

class ICU4XFixedDecimalFormatterOptions {
  final ICU4XFixedDecimalFormatterOptionsFfi _underlying;
  ICU4XFixedDecimalGroupingStrategy get groupingStrategy => ICU4XFixedDecimalGroupingStrategyFromFfi(this._underlying.groupingStrategy);
  void set groupingStrategy(ICU4XFixedDecimalGroupingStrategy groupingStrategy) {this._underlying.groupingStrategy = ICU4XFixedDecimalGroupingStrategyAsFfi(groupingStrategy);
  }

  bool get someOtherConfig => this._underlying.someOtherConfig;
  void set someOtherConfig(bool someOtherConfig) {this._underlying.someOtherConfig = someOtherConfig;
  }

factory ICU4XFixedDecimalFormatterOptions.default() { 
    final result = _defaultFfi();
    return ICU4XFixedDecimalFormatterOptionsFromFfi(result);
  }
  static late final _defaultFfi = capi<ffi.NativeFunction<ICU4XFixedDecimalFormatterOptionsFfi Function()>>('ICU4XFixedDecimalFormatterOptions_default')
  .asFunction<ICU4XFixedDecimalFormatterOptionsFfi Function()>();
  ICU4XFixedDecimalFormatterOptions._(this._underlying);
}

// These are not methods because we want to keep them package-private, and methods are either private or public
ICU4XFixedDecimalFormatterOptions ICU4XFixedDecimalFormatterOptionsFromFfi(ICU4XFixedDecimalFormatterOptionsFfi underlying) => ICU4XFixedDecimalFormatterOptions._(underlying);
ICU4XFixedDecimalFormatterOptionsFfi ICU4XFixedDecimalFormatterOptionsAsFfi(ICU4XFixedDecimalFormatterOptions t) => t._underlying;

