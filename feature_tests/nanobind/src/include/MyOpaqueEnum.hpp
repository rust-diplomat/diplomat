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

inline somelib::MyOpaqueEnum somelib::MyOpaqueEnum::new_() {
    auto result = somelib::capi::MyOpaqueEnum_new();
    return somelib::MyOpaqueEnum::FromFFI(result);
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


#endif // SOMELIB_MyOpaqueEnum_HPP
