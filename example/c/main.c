#include <stdio.h>
#include <assert.h>

#include "include/ICU4XFixedDecimalFormatter.h"

void print_decimal(ICU4XFixedDecimal* fd) {
    char output[40];
    DiplomatWriteable out = diplomat_simple_writeable(output, 40);
    assert(ICU4XFixedDecimal_to_string(fd, &out).is_ok == true);
    output[out.len] = '\0';
    printf("%s\n", output);
}

void format_decimal(ICU4XFixedDecimalFormatter* fdf, ICU4XFixedDecimal* fd) {
    char output[40];
    DiplomatWriteable out = diplomat_simple_writeable(output, 40);
    ICU4XFixedDecimalFormatter_format_write(fdf, fd, &out);
    output[out.len] = '\0';
    printf("%s\n", output);
}

int main(int argc, char *argv[]) {
    ICU4XFixedDecimal* fd = ICU4XFixedDecimal_new(123);

    print_decimal(fd);

    ICU4XFixedDecimal_multiply_pow10(fd, -1);
    printf("multiplied by 0.1\n");

    print_decimal(fd);

    ICU4XLocale* locale = ICU4XLocale_new("bn", 2);

    ICU4XDataProvider* data_provider = ICU4XDataProvider_new_static();

    diplomat_result_box_ICU4XFixedDecimalFormatter_void fdf = ICU4XFixedDecimalFormatter_try_new(locale, data_provider, ICU4XFixedDecimalFormatterOptions_default());
    printf("success: %d\n", fdf.is_ok);
    format_decimal(fdf.ok, fd);
}
