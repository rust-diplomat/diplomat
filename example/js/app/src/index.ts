import { FixedDecimal, Locale, DataProvider, FixedDecimalFormatterOptions, FixedDecimalFormatter } from "../node_modules/demo/api/index.js";

const locale = Locale.new("bn");

const data_provider = DataProvider.new_static();

const fdf = FixedDecimalFormatter.try_new(locale, data_provider, FixedDecimalFormatterOptions.default());

export function format(n: number): string {
    if (n > 2147483647 || n < -2147483648 || n % 1 !== 0) {
        throw Error(`Not an i32: ${n}`);
    }
    return fdf.format_write(FixedDecimal.new(n));
}
