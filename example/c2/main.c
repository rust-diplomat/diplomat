#include <stdio.h>
#include <assert.h>

#include "include/ICU4XLocale.h"
#include "include/ICU4XDataProvider.h"
#include "include/ICU4XFixedDecimal.h"
#include "include/ICU4XFixedDecimalFormatter.h"
#include "include/ICU4XFixedDecimalFormatterOptions.h"

void print_decimal(ICU4XFixedDecimal* fd) {
    char output[40];
    DiplomatWrite out = diplomat_simple_write(output, 40);
    assert(icu4x_ICU4XFixedDecimal_to_string_mv1(fd, &out).is_ok == true);
    output[out.len] = '\0';
    printf("%s\n", output);
}

void format_decimal(ICU4XFixedDecimalFormatter* fdf, ICU4XFixedDecimal* fd) {
    char output[40];
    DiplomatWrite out = diplomat_simple_write(output, 40);
    icu4x_ICU4XFixedDecimalFormatter_format_write_mv1(fdf, fd, &out);
    output[out.len] = '\0';
    printf("%s\n", output);
}

int main(int argc, char *argv[]) {
    ICU4XFixedDecimal* fd = icu4x_ICU4XFixedDecimal_new_mv1(123);

    print_decimal(fd);

    icu4x_ICU4XFixedDecimal_multiply_pow10_mv1(fd, -1);
    printf("multiplied by 0.1\n");

    print_decimal(fd);

    ICU4XLocale* locale = icu4x_ICU4XLocale_new_mv1("bn", 2);

    ICU4XDataProvider* data_provider = icu4x_ICU4XDataProvider_new_static_mv1();

    struct icu4x_ICU4XFixedDecimalFormatter_try_new_mv1_result fdf = icu4x_ICU4XFixedDecimalFormatter_try_new_mv1(locale, data_provider, icu4x_ICU4XFixedDecimalFormatterOptions_default_mv1());
    printf("%d\n", fdf.is_ok);
    format_decimal(fdf.ok, fd);
}
