#include <stdio.h>
#include <assert.h>

#include "include/Locale.h"
#include "include/DataProvider.h"
#include "include/FixedDecimal.h"
#include "include/FixedDecimalFormatter.h"
#include "include/FixedDecimalFormatterOptions.h"

void print_decimal(FixedDecimal* fd) {
    char output[40];
    DiplomatWrite out = diplomat_simple_write(output, 40);
    assert(icu4x_FixedDecimal_to_string_mv1(fd, &out).is_ok == true);
    output[out.len] = '\0';
    printf("%s\n", output);
}

void format_decimal(FixedDecimalFormatter* fdf, FixedDecimal* fd) {
    char output[40];
    DiplomatWrite out = diplomat_simple_write(output, 40);
    icu4x_FixedDecimalFormatter_format_write_mv1(fdf, fd, &out);
    output[out.len] = '\0';
    printf("%s\n", output);
}

int main(int argc, char *argv[]) {
    FixedDecimal* fd = icu4x_FixedDecimal_new_mv1(123);

    print_decimal(fd);

    icu4x_FixedDecimal_multiply_pow10_mv1(fd, -1);
    printf("multiplied by 0.1\n");

    print_decimal(fd);

    Locale* locale = icu4x_Locale_new_mv1((DiplomatStringView){.data = "bn", .len = 2});

    DataProvider* data_provider = icu4x_DataProvider_new_static_mv1();

    struct icu4x_FixedDecimalFormatter_try_new_mv1_result fdf = icu4x_FixedDecimalFormatter_try_new_mv1(locale, data_provider, icu4x_FixedDecimalFormatterOptions_default_mv1());
    printf("%d\n", fdf.is_ok);
    format_decimal(fdf.ok, fd);
}
