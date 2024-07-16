#include <iostream>
#include "../include/FixedDecimalFormatter.hpp"
#include "assert.hpp"

int main(int argc, char *argv[]) {
    FixedDecimal fd = FixedDecimal::new_(123);

    simple_assert("constructing FixedDecimal", !fd.to_string().is_err());

    std::string fd_out = fd.to_string().ok().value();

    simple_assert_eq("Stringifying FixedDecimal", fd_out, "123");

    fd.multiply_pow10(-1);

    fd_out = fd.to_string().ok().value();

    simple_assert_eq("Multiplying FixedDecimal", fd_out, "12.3");

    std::string out;

    fd.to_string_to_write(out);

    simple_assert_eq("Formatting FixedDecimal to Write", fd_out, "12.3");

    Locale locale = Locale::new_("bn");

    DataProvider data_provider = DataProvider::new_static();

    auto fdf = FixedDecimalFormatter::try_new(locale, data_provider, FixedDecimalFormatterOptions::default_());

    simple_assert("Formatting FixedDecimal", fdf.is_ok());

    out = std::move(fdf).ok().value().format_write(fd);

    simple_assert_eq("Formatting FixedDecimal", out, "১২.৩");

    std::cout << "Formatted value is " << out << std::endl;
}
