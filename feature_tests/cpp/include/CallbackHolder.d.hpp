#ifndef SOMELIB_CallbackHolder_D_HPP
#define SOMELIB_CallbackHolder_D_HPP

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
namespace capi { struct CallbackHolder; }
class CallbackHolder;
} // namespace somelib



namespace somelib {
namespace capi {
    struct CallbackHolder;
} // namespace capi
} // namespace

namespace somelib {
class CallbackHolder {
public:

  inline static std::unique_ptr<somelib::CallbackHolder> new_(std::function<int32_t(int32_t)> func);

  inline int32_t call(int32_t a) const;

    inline const somelib::capi::CallbackHolder* AsFFI() const;
    inline somelib::capi::CallbackHolder* AsFFI();
    inline static const somelib::CallbackHolder* FromFFI(const somelib::capi::CallbackHolder* ptr);
    inline static somelib::CallbackHolder* FromFFI(somelib::capi::CallbackHolder* ptr);
    inline static void operator delete(void* ptr);
private:
    CallbackHolder() = delete;
    CallbackHolder(const somelib::CallbackHolder&) = delete;
    CallbackHolder(somelib::CallbackHolder&&) noexcept = delete;
    CallbackHolder operator=(const somelib::CallbackHolder&) = delete;
    CallbackHolder operator=(somelib::CallbackHolder&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_CallbackHolder_D_HPP
