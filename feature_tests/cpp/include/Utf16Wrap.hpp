#ifndef SOMELIB_Utf16Wrap_HPP
#define SOMELIB_Utf16Wrap_HPP

#include "Utf16Wrap.d.hpp"

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

    somelib::capi::Utf16Wrap* Utf16Wrap_from_utf16(somelib::diplomat::capi::DiplomatString16View input);

    void Utf16Wrap_get_debug_str(const somelib::capi::Utf16Wrap* self, somelib::diplomat::capi::DiplomatWrite* write);

    somelib::diplomat::capi::DiplomatString16View Utf16Wrap_borrow_cont(const somelib::capi::Utf16Wrap* self);

    void Utf16Wrap_destroy(Utf16Wrap* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::Utf16Wrap> somelib::Utf16Wrap::from_utf16(std::u16string_view input) {
    auto result = somelib::capi::Utf16Wrap_from_utf16({input.data(), input.size()});
    return std::unique_ptr<somelib::Utf16Wrap>(somelib::Utf16Wrap::FromFFI(result));
}

inline std::string somelib::Utf16Wrap::get_debug_str() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::Utf16Wrap_get_debug_str(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::Utf16Wrap::get_debug_str_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::Utf16Wrap_get_debug_str(this->AsFFI(),
        &write);
}

inline std::u16string_view somelib::Utf16Wrap::borrow_cont() const {
    auto result = somelib::capi::Utf16Wrap_borrow_cont(this->AsFFI());
    return std::u16string_view(result.data, result.len);
}

inline const somelib::capi::Utf16Wrap* somelib::Utf16Wrap::AsFFI() const {
    return reinterpret_cast<const somelib::capi::Utf16Wrap*>(this);
}

inline somelib::capi::Utf16Wrap* somelib::Utf16Wrap::AsFFI() {
    return reinterpret_cast<somelib::capi::Utf16Wrap*>(this);
}

inline const somelib::Utf16Wrap* somelib::Utf16Wrap::FromFFI(const somelib::capi::Utf16Wrap* ptr) {
    return reinterpret_cast<const somelib::Utf16Wrap*>(ptr);
}

inline somelib::Utf16Wrap* somelib::Utf16Wrap::FromFFI(somelib::capi::Utf16Wrap* ptr) {
    return reinterpret_cast<somelib::Utf16Wrap*>(ptr);
}

inline void somelib::Utf16Wrap::operator delete(void* ptr) {
    somelib::capi::Utf16Wrap_destroy(reinterpret_cast<somelib::capi::Utf16Wrap*>(ptr));
}


#endif // SOMELIB_Utf16Wrap_HPP
