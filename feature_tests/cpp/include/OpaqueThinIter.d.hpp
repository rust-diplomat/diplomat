#ifndef SOMELIB_OpaqueThinIter_D_HPP
#define SOMELIB_OpaqueThinIter_D_HPP

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
namespace capi { struct OpaqueThin; }
class OpaqueThin;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OpaqueThinIter;
} // namespace capi
} // namespace

namespace somelib {
class OpaqueThinIter {
public:

  inline const somelib::OpaqueThin* next();

    inline const somelib::capi::OpaqueThinIter* AsFFI() const;
    inline somelib::capi::OpaqueThinIter* AsFFI();
    inline static const somelib::OpaqueThinIter* FromFFI(const somelib::capi::OpaqueThinIter* ptr);
    inline static somelib::OpaqueThinIter* FromFFI(somelib::capi::OpaqueThinIter* ptr);
    inline static void operator delete(void* ptr);
private:
    OpaqueThinIter() = delete;
    OpaqueThinIter(const somelib::OpaqueThinIter&) = delete;
    OpaqueThinIter(somelib::OpaqueThinIter&&) noexcept = delete;
    OpaqueThinIter operator=(const somelib::OpaqueThinIter&) = delete;
    OpaqueThinIter operator=(somelib::OpaqueThinIter&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_OpaqueThinIter_D_HPP
