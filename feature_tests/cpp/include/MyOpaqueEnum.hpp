#ifndef SOMELIB_MyOpaqueEnum_HPP
#define SOMELIB_MyOpaqueEnum_HPP

#include "MyOpaqueEnum.d.hpp"

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

    somelib::capi::MyOpaqueEnum* MyOpaqueEnum_new(void);

    void MyOpaqueEnum_to_string(const somelib::capi::MyOpaqueEnum* self, somelib::diplomat::capi::DiplomatWrite* write);

    void MyOpaqueEnum_destroy(MyOpaqueEnum* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::MyOpaqueEnum> somelib::MyOpaqueEnum::new_() {
    auto result = somelib::capi::MyOpaqueEnum_new();
    return std::unique_ptr<somelib::MyOpaqueEnum>(somelib::MyOpaqueEnum::FromFFI(result));
}

inline std::string somelib::MyOpaqueEnum::to_string() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::MyOpaqueEnum_to_string(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::MyOpaqueEnum::to_string_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::MyOpaqueEnum_to_string(this->AsFFI(),
        &write);
}

inline const somelib::capi::MyOpaqueEnum* somelib::MyOpaqueEnum::AsFFI() const {
    return reinterpret_cast<const somelib::capi::MyOpaqueEnum*>(this);
}

inline somelib::capi::MyOpaqueEnum* somelib::MyOpaqueEnum::AsFFI() {
    return reinterpret_cast<somelib::capi::MyOpaqueEnum*>(this);
}

inline const somelib::MyOpaqueEnum* somelib::MyOpaqueEnum::FromFFI(const somelib::capi::MyOpaqueEnum* ptr) {
    return reinterpret_cast<const somelib::MyOpaqueEnum*>(ptr);
}

inline somelib::MyOpaqueEnum* somelib::MyOpaqueEnum::FromFFI(somelib::capi::MyOpaqueEnum* ptr) {
    return reinterpret_cast<somelib::MyOpaqueEnum*>(ptr);
}

inline void somelib::MyOpaqueEnum::operator delete(void* ptr) {
    somelib::capi::MyOpaqueEnum_destroy(reinterpret_cast<somelib::capi::MyOpaqueEnum*>(ptr));
}


#endif // SOMELIB_MyOpaqueEnum_HPP
