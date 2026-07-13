#ifndef SOMELIB_OptionString_HPP
#define SOMELIB_OptionString_HPP

#include "OptionString.d.hpp"

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
namespace capi {
    extern "C" {

    somelib::capi::OptionString* OptionString_new(somelib::diplomat::capi::DiplomatStringView diplomat_str);

    typedef struct OptionString_write_result { bool is_ok;} OptionString_write_result;
    OptionString_write_result OptionString_write(const somelib::capi::OptionString* self, somelib::diplomat::capi::DiplomatWrite* write);

    typedef struct OptionString_borrow_result {union {somelib::diplomat::capi::DiplomatStringView ok; }; bool is_ok;} OptionString_borrow_result;
    OptionString_borrow_result OptionString_borrow(const somelib::capi::OptionString* self);

    void OptionString_destroy(OptionString* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::Optional<somelib::OptionString> somelib::OptionString::new_(std::string_view diplomat_str) {
    auto result = somelib::capi::OptionString_new({diplomat_str.data(), diplomat_str.size()});
    return somelib::diplomat::Optional<somelib::OptionString>::FromFFI(result);
}

inline somelib::diplomat::result<std::string, std::monostate> somelib::OptionString::write() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    auto result = somelib::capi::OptionString_write(this->AsFFI(),
        &write);
    return result.is_ok ? somelib::diplomat::result<std::string, std::monostate>(somelib::diplomat::Ok<std::string>(std::move(output))) : somelib::diplomat::result<std::string, std::monostate>(somelib::diplomat::Err<std::monostate>());
}
template<typename W>
inline somelib::diplomat::result<std::monostate, std::monostate> somelib::OptionString::write_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = somelib::capi::OptionString_write(this->AsFFI(),
        &write);
    return result.is_ok ? somelib::diplomat::result<std::monostate, std::monostate>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, std::monostate>(somelib::diplomat::Err<std::monostate>());
}

inline somelib::diplomat::Optional<std::string_view> somelib::OptionString::borrow() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::OptionString_borrow(this->AsFFI());
    return result.is_ok ? somelib::diplomat::Optional<std::string_view>(std::string_view(result.ok.data, result.ok.len)) : somelib::diplomat::Optional<std::string_view>(std::nullopt);
}


#endif // SOMELIB_OptionString_HPP
