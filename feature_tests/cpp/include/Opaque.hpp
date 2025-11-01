#ifndef SOMELIB_Opaque_HPP
#define SOMELIB_Opaque_HPP

#include "Opaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "ImportedStruct.hpp"
#include "MyStruct.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::Opaque* Opaque_new(void);

    somelib::capi::Opaque* Opaque_try_from_utf8(somelib::diplomat::capi::DiplomatStringView input);

    somelib::capi::Opaque* Opaque_from_str(somelib::diplomat::capi::DiplomatStringView input);

    void Opaque_get_debug_str(const somelib::capi::Opaque* self, somelib::diplomat::capi::DiplomatWrite* write);

    void Opaque_assert_struct(const somelib::capi::Opaque* self, somelib::capi::MyStruct s);

    size_t Opaque_returns_usize(void);

    somelib::capi::ImportedStruct Opaque_returns_imported(void);

    int8_t Opaque_cmp(void);

    void Opaque_destroy(Opaque* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::Opaque> somelib::Opaque::new_() {
    auto result = somelib::capi::Opaque_new();
    return std::unique_ptr<somelib::Opaque>(somelib::Opaque::FromFFI(result));
}

inline std::unique_ptr<somelib::Opaque> somelib::Opaque::try_from_utf8(std::string_view input) {
    auto result = somelib::capi::Opaque_try_from_utf8({input.data(), input.size()});
    return std::unique_ptr<somelib::Opaque>(somelib::Opaque::FromFFI(result));
}

inline somelib::diplomat::result<std::unique_ptr<somelib::Opaque>, somelib::diplomat::Utf8Error> somelib::Opaque::from_str(std::string_view input) {
    if (!somelib::diplomat::capi::diplomat_is_str(input.data(), input.size())) {
    return somelib::diplomat::Err<somelib::diplomat::Utf8Error>();
  }
    auto result = somelib::capi::Opaque_from_str({input.data(), input.size()});
    return somelib::diplomat::Ok<std::unique_ptr<somelib::Opaque>>(std::unique_ptr<somelib::Opaque>(somelib::Opaque::FromFFI(result)));
}

inline std::string somelib::Opaque::get_debug_str() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::Opaque_get_debug_str(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::Opaque::get_debug_str_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::Opaque_get_debug_str(this->AsFFI(),
        &write);
}

inline void somelib::Opaque::assert_struct(somelib::MyStruct s) const {
    somelib::capi::Opaque_assert_struct(this->AsFFI(),
        s.AsFFI());
}

inline size_t somelib::Opaque::returns_usize() {
    auto result = somelib::capi::Opaque_returns_usize();
    return result;
}

inline somelib::ImportedStruct somelib::Opaque::returns_imported() {
    auto result = somelib::capi::Opaque_returns_imported();
    return somelib::ImportedStruct::FromFFI(result);
}

inline int8_t somelib::Opaque::cmp() {
    auto result = somelib::capi::Opaque_cmp();
    return result;
}

inline const somelib::capi::Opaque* somelib::Opaque::AsFFI() const {
    return reinterpret_cast<const somelib::capi::Opaque*>(this);
}

inline somelib::capi::Opaque* somelib::Opaque::AsFFI() {
    return reinterpret_cast<somelib::capi::Opaque*>(this);
}

inline const somelib::Opaque* somelib::Opaque::FromFFI(const somelib::capi::Opaque* ptr) {
    return reinterpret_cast<const somelib::Opaque*>(ptr);
}

inline somelib::Opaque* somelib::Opaque::FromFFI(somelib::capi::Opaque* ptr) {
    return reinterpret_cast<somelib::Opaque*>(ptr);
}

inline void somelib::Opaque::operator delete(void* ptr) {
    somelib::capi::Opaque_destroy(reinterpret_cast<somelib::capi::Opaque*>(ptr));
}


#endif // SOMELIB_Opaque_HPP
