#include <iostream>
#include "../include/ICU4XFixedDecimalFormatter.hpp"
#include "../include/ICU4XLocale.hpp"
#include "assert.hpp"

int main(int argc, char *argv[]) {
    std::unique_ptr<ICU4XFixedDecimal> fd = ICU4XFixedDecimal::new_(123);

    simple_assert("constructing FixedDecimal", !fd->to_string().is_err());

    std::string fd_out = fd->to_string().ok().value();

    simple_assert_eq("Stringifying FixedDecimal", fd_out, "123");

    fd->multiply_pow10(-1);

    fd_out = fd->to_string().ok().value();

    simple_assert_eq("Multiplying FixedDecimal", fd_out, "12.3");

    // std::string out;

    // fd->to_string_to_writeable(out);

    // simple_assert_eq("Formatting FixedDecimal to Writeable", fd_out, "12.3");

    std::unique_ptr<ICU4XLocale> locale = ICU4XLocale::new_("bn");

    std::unique_ptr<ICU4XDataProvider> data_provider = ICU4XDataProvider::new_static();

    auto fdf = ICU4XFixedDecimalFormatter::try_new(*locale, *data_provider, ICU4XFixedDecimalFormatterOptions::default_());

    simple_assert("Formatting FixedDecimal", fdf.is_ok());

    std::string fdf_out = std::move(fdf).ok().value()->format_write(*fd);

    simple_assert_eq("Formatting FixedDecimal", fdf_out, "১২.৩");

    std::cout << "Formatted value is " << fdf_out << std::endl;
}
