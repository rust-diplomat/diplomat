#include <stdio.h>
#include <assert.h>

#include "include/icu4x_Locale.h"
#include "include/icu4x_DataProvider.h"
#include "include/icu4x_FixedDecimal.h"
#include "include/icu4x_FixedDecimalFormatter.h"
#include "include/icu4x_FixedDecimalFormatterOptions.h"

void print_decimal(icu4x_FixedDecimal* fd) {
    char output[40];
    DiplomatWrite out = diplomat_simple_write(output, 40);
    assert(icu4x_FixedDecimal_to_string(fd, &out).is_ok == true);
    output[out.len] = '\0';
    printf("%s\n", output);
}

void format_decimal(icu4x_FixedDecimalFormatter* fdf, icu4x_FixedDecimal* fd) {
    char output[40];
    DiplomatWrite out = diplomat_simple_write(output, 40);
    icu4x_FixedDecimalFormatter_format_write(fdf, fd, &out);
    output[out.len] = '\0';
    printf("%s\n", output);
}

int main(int argc, char *argv[]) {
    icu4x_FixedDecimal* fd = icu4x_FixedDecimal_new(123);

    print_decimal(fd);

    icu4x_FixedDecimal_multiply_pow10(fd, -1);
    printf("multiplied by 0.1\n");

    print_decimal(fd);

    icu4x_Locale* locale = icu4x_Locale_new("bn", 2);

    icu4x_DataProvider* data_provider = icu4x_DataProvider_new_static();

    struct icu4x_FixedDecimalFormatter_try_new_result fdf = icu4x_FixedDecimalFormatter_try_new(locale, data_provider, icu4x_FixedDecimalFormatterOptions_default());
    printf("%d\n", fdf.is_ok);
    format_decimal(fdf.ok, fd);
}
