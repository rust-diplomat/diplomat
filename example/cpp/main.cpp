#include <iostream>
#include "api.hpp"

int main(int argc, char *argv[]) {
    ICU4XFixedDecimal fd = ICU4XFixedDecimal::new_(123);

    std::cout << fd.to_string() << std::endl;

    fd.multiply_pow10(-1);
    std::cout << "multiplied by 0.1" << std::endl;

    std::cout << fd.to_string() << std::endl;

    ICU4XLocale locale = ICU4XLocale::new_("bn", 2);

    ICU4XDataProvider data_provider = ICU4XDataProvider::new_static();

    auto fdf = ICU4XFixedDecimalFormat::try_new(locale, data_provider, ICU4XFixedDecimalFormatOptions::default_());
    std::cout << fdf.success << std::endl;
    std::cout << fdf.fdf.value().format_write(fd) << std::endl;
}
