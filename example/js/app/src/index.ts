import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormatterOptions, ICU4XFixedDecimalFormatter } from "../node_modules/demo/api/index.js";

const locale = ICU4XLocale.new("bn");

const data_provider = ICU4XDataProvider.new_static();

const fdf = ICU4XFixedDecimalFormatter.try_new(locale, data_provider, ICU4XFixedDecimalFormatterOptions.default());

export function format(n: number): string {
    if (n > 2147483647 || n < -2147483648 || n % 1 !== 0) {
        throw Error(`Not an i32: ${n}`);
    }
    return fdf.format_write(ICU4XFixedDecimal.new(n));
}
