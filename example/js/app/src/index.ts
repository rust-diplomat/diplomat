import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormatOptions, ICU4XFixedDecimalFormat } from "../node_modules/demo/api/index.js";

const locale = ICU4XLocale.new("bn");

const data_provider = ICU4XDataProvider.new_static();

const fdf = ICU4XFixedDecimalFormat.try_new(locale, data_provider, ICU4XFixedDecimalFormatOptions.default());
if (!fdf.success) {
    throw Error("Failed to create fixed decimal formatter");
}

export function format(n: number): string {
    if (n > 2147483647 || n < -2147483648 || n % 1 !== 0) {
        throw Error(`Not an i32: ${n}`);
    }
    return fdf.fdf.format_write(ICU4XFixedDecimal.new(n));
}
