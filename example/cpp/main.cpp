#include <iostream>
#include "ICU4XFixedDecimalFormat.hpp"

int main(int argc, char *argv[]) {
    ICU4XFixedDecimal fd = ICU4XFixedDecimal::new_(123);

    if (fd.to_string().is_err()) {
        std::cout<<"Failed to construct FixedDecimal"<<std::endl;
        return 1;
    }

    std::string fd_out = fd.to_string().ok().value();

    if (fd_out != "123") {
        std::cout << "Expected 123, found " << fd_out << std::endl;
        return 1;
    }
    
    fd.multiply_pow10(-1);

    fd_out = fd.to_string().ok().value();
    if (fd_out != "12.3") {
        std::cout << "Expected 12.3, found " << fd_out << std::endl;
        return 1;
    }
    std::string out;

    fd.to_string_to_writeable(out);

    if (out != "12.3") {
        std::cout << "Writeable: expected 12.3, found " << out << std::endl;
        return 1;
    }

    std::array<uint8_t, 2> bytes = {'e', 'n'};
    ICU4XLocale locale = ICU4XLocale::new_from_bytes(bytes);

    locale = ICU4XLocale::new_("bn");

    ICU4XDataProvider data_provider = ICU4XDataProvider::new_static();

    auto fdf = ICU4XFixedDecimalFormat::try_new(locale, data_provider, ICU4XFixedDecimalFormatOptions::default_());
    if (!fdf.success) {
        std::cout << "Failed to format fixed decimal" << std::endl;
        return 1;
    }

    out = fdf.fdf.value().format_write(fd);

    if (out != "১২.৩") {
        std::cout << "Expected ১২.৩, found " << out << std::endl;
        return 1;
    }
    std::cout << "Formatted value is " << out << std::endl;
}
