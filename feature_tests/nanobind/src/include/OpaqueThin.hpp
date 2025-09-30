#ifndef SOMELIB_OpaqueThin_HPP
#define SOMELIB_OpaqueThin_HPP

#include "OpaqueThin.d.hpp"

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

    int32_t OpaqueThin_a(const somelib::capi::OpaqueThin* self);

    float OpaqueThin_b(const somelib::capi::OpaqueThin* self);

    void OpaqueThin_c(const somelib::capi::OpaqueThin* self, somelib::diplomat::capi::DiplomatWrite* write);

    void OpaqueThin_destroy(OpaqueThin* self);

    } // extern "C"
} // namespace capi
} // namespace

inline int32_t somelib::OpaqueThin::a() const {
    auto result = somelib::capi::OpaqueThin_a(this->AsFFI());
    return result;
}

inline float somelib::OpaqueThin::b() const {
    auto result = somelib::capi::OpaqueThin_b(this->AsFFI());
    return result;
}

inline std::string somelib::OpaqueThin::c() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::OpaqueThin_c(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::OpaqueThin::c_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::OpaqueThin_c(this->AsFFI(),
        &write);
}

inline const somelib::capi::OpaqueThin* somelib::OpaqueThin::AsFFI() const {
    return reinterpret_cast<const somelib::capi::OpaqueThin*>(this);
}

inline somelib::capi::OpaqueThin* somelib::OpaqueThin::AsFFI() {
    return reinterpret_cast<somelib::capi::OpaqueThin*>(this);
}

inline const somelib::OpaqueThin* somelib::OpaqueThin::FromFFI(const somelib::capi::OpaqueThin* ptr) {
    return reinterpret_cast<const somelib::OpaqueThin*>(ptr);
}

inline somelib::OpaqueThin* somelib::OpaqueThin::FromFFI(somelib::capi::OpaqueThin* ptr) {
    return reinterpret_cast<somelib::OpaqueThin*>(ptr);
}

inline void somelib::OpaqueThin::operator delete(void* ptr) {
    somelib::capi::OpaqueThin_destroy(reinterpret_cast<somelib::capi::OpaqueThin*>(ptr));
}


#endif // SOMELIB_OpaqueThin_HPP
