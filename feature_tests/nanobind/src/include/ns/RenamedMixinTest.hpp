#ifndef SOMELIB_ns_RenamedMixinTest_HPP
#define SOMELIB_ns_RenamedMixinTest_HPP

#include "RenamedMixinTest.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {
    extern "C" {

    void namespace_MixinTest_hello(somelib::diplomat::capi::DiplomatWrite* write);

    void namespace_MixinTest_destroy(RenamedMixinTest* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string somelib::ns::RenamedMixinTest::hello() {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::ns::capi::namespace_MixinTest_hello(&write);
    return output;
}
template<typename W>
inline void somelib::ns::RenamedMixinTest::hello_write(W& writeable) {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::ns::capi::namespace_MixinTest_hello(&write);
}

inline const somelib::ns::capi::RenamedMixinTest* somelib::ns::RenamedMixinTest::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedMixinTest*>(this);
}

inline somelib::ns::capi::RenamedMixinTest* somelib::ns::RenamedMixinTest::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedMixinTest*>(this);
}

inline const somelib::ns::RenamedMixinTest* somelib::ns::RenamedMixinTest::FromFFI(const somelib::ns::capi::RenamedMixinTest* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedMixinTest*>(ptr);
}

inline somelib::ns::RenamedMixinTest* somelib::ns::RenamedMixinTest::FromFFI(somelib::ns::capi::RenamedMixinTest* ptr) {
    return reinterpret_cast<somelib::ns::RenamedMixinTest*>(ptr);
}

inline void somelib::ns::RenamedMixinTest::operator delete(void* ptr) {
    somelib::ns::capi::namespace_MixinTest_destroy(reinterpret_cast<somelib::ns::capi::RenamedMixinTest*>(ptr));
}


#endif // SOMELIB_ns_RenamedMixinTest_HPP
