#ifndef SOMELIB_MutableCallbackHolder_D_HPP
#define SOMELIB_MutableCallbackHolder_D_HPP

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
namespace capi { struct MutableCallbackHolder; }
class MutableCallbackHolder;
class FFIError;
} // namespace somelib



namespace somelib {
namespace capi {
    struct MutableCallbackHolder;
} // namespace capi
} // namespace

namespace somelib {
class MutableCallbackHolder {
public:

  inline static std::unique_ptr<somelib::MutableCallbackHolder> new_fallible(std::function<somelib::diplomat::result<int32_t, somelib::FFIError>(int32_t)> func);

  inline int32_t call(int32_t a);

    inline const somelib::capi::MutableCallbackHolder* AsFFI() const;
    inline somelib::capi::MutableCallbackHolder* AsFFI();
    inline static const somelib::MutableCallbackHolder* FromFFI(const somelib::capi::MutableCallbackHolder* ptr);
    inline static somelib::MutableCallbackHolder* FromFFI(somelib::capi::MutableCallbackHolder* ptr);
    inline static void operator delete(void* ptr);
private:
    MutableCallbackHolder() = delete;
    MutableCallbackHolder(const somelib::MutableCallbackHolder&) = delete;
    MutableCallbackHolder(somelib::MutableCallbackHolder&&) noexcept = delete;
    MutableCallbackHolder operator=(const somelib::MutableCallbackHolder&) = delete;
    MutableCallbackHolder operator=(somelib::MutableCallbackHolder&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_MutableCallbackHolder_D_HPP
