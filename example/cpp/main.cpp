#include <iostream>
#include "ICU4XFixedDecimalFormat.hpp"

int main(int argc, char *argv[]) {
    ICU4XFixedDecimal fd = ICU4XFixedDecimal::new_(123);

    std::cout << "ok" << fd.to_string().is_err() << std::endl;

    std::string fd_out = fd.to_string().ok().value();

    std::cout << fd_out << std::endl;

    fd.multiply_pow10(-1);
    std::cout << "multiplied by 0.1" << std::endl;

    fd_out = fd.to_string().ok().value();
    std::cout << fd_out << std::endl;

    std::string out;

    fd.to_string_to_writeable(out);

    std::cout << "writeable: " << out << std::endl;

    ICU4XLocale locale = ICU4XLocale::new_("bn");

    ICU4XDataProvider data_provider = ICU4XDataProvider::new_static();

    auto fdf = ICU4XFixedDecimalFormat::try_new(locale, data_provider, ICU4XFixedDecimalFormatOptions::default_());
    std::cout << fdf.success << std::endl;
    std::cout << fdf.fdf.value().format_write(fd) << std::endl;
}
