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

inline std::unique_ptr<somelib::OptionString> somelib::OptionString::new_(std::string_view diplomat_str) {
    auto result = somelib::capi::OptionString_new({diplomat_str.data(), diplomat_str.size()});
    return std::unique_ptr<somelib::OptionString>(somelib::OptionString::FromFFI(result));
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

inline std::optional<std::string_view> somelib::OptionString::borrow() const {
    auto result = somelib::capi::OptionString_borrow(this->AsFFI());
    return result.is_ok ? std::optional<std::string_view>(std::string_view(result.ok.data, result.ok.len)) : std::nullopt;
}

inline const somelib::capi::OptionString* somelib::OptionString::AsFFI() const {
    return reinterpret_cast<const somelib::capi::OptionString*>(this);
}

inline somelib::capi::OptionString* somelib::OptionString::AsFFI() {
    return reinterpret_cast<somelib::capi::OptionString*>(this);
}

inline const somelib::OptionString* somelib::OptionString::FromFFI(const somelib::capi::OptionString* ptr) {
    return reinterpret_cast<const somelib::OptionString*>(ptr);
}

inline somelib::OptionString* somelib::OptionString::FromFFI(somelib::capi::OptionString* ptr) {
    return reinterpret_cast<somelib::OptionString*>(ptr);
}

inline void somelib::OptionString::operator delete(void* ptr) {
    somelib::capi::OptionString_destroy(reinterpret_cast<somelib::capi::OptionString*>(ptr));
}


#endif // SOMELIB_OptionString_HPP
