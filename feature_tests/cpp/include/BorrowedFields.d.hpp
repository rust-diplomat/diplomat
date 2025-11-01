#ifndef SOMELIB_BorrowedFields_D_HPP
#define SOMELIB_BorrowedFields_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct Bar; }
class Bar;
struct BorrowedFields;
} // namespace somelib



namespace somelib {
namespace capi {
    struct BorrowedFields {
      somelib::diplomat::capi::DiplomatString16View a;
      somelib::diplomat::capi::DiplomatStringView b;
      somelib::diplomat::capi::DiplomatStringView c;
    };

    typedef struct BorrowedFields_option {union { BorrowedFields ok; }; bool is_ok; } BorrowedFields_option;
} // namespace capi
} // namespace


namespace somelib {
struct BorrowedFields {
    std::u16string_view a;
    std::string_view b;
    std::string_view c;

  inline static somelib::diplomat::result<somelib::BorrowedFields, somelib::diplomat::Utf8Error> from_bar_and_strings(const somelib::Bar& bar, std::u16string_view dstr16, std::string_view utf8_str);

    inline somelib::capi::BorrowedFields AsFFI() const;
    inline static somelib::BorrowedFields FromFFI(somelib::capi::BorrowedFields c_struct);
};

} // namespace
#endif // SOMELIB_BorrowedFields_D_HPP
