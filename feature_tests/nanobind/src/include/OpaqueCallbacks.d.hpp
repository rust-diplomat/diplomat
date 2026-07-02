#ifndef SOMELIB_OpaqueCallbacks_D_HPP
#define SOMELIB_OpaqueCallbacks_D_HPP

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
namespace capi { struct MyString; }
class MyString;
namespace capi { struct OpaqueCallbacks; }
class OpaqueCallbacks;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OpaqueCallbacks;
} // namespace capi
} // namespace

namespace somelib {
class OpaqueCallbacks {
public:

  inline static const somelib::MyString& ret_op(std::function<const somelib::MyString&(const somelib::MyString&)> f, const somelib::MyString& st);

  inline static std::unique_ptr<somelib::OpaqueCallbacks> ctor(std::function<const somelib::MyString&(const somelib::MyString&)> f, const somelib::MyString& st);

  inline const somelib::MyString& opaque_cb_self(std::function<const somelib::MyString&(const somelib::MyString&)> cb, const somelib::MyString& st) const;

  inline const somelib::MyString& opaque_cb_mut_self(std::function<const somelib::MyString&(const somelib::MyString&)> cb, const somelib::MyString& st);

    inline const somelib::capi::OpaqueCallbacks* AsFFI() const;
    inline somelib::capi::OpaqueCallbacks* AsFFI();
    inline static const somelib::OpaqueCallbacks* FromFFI(const somelib::capi::OpaqueCallbacks* ptr);
    inline static somelib::OpaqueCallbacks* FromFFI(somelib::capi::OpaqueCallbacks* ptr);
    inline static void operator delete(void* ptr);
private:
    OpaqueCallbacks() = delete;
    OpaqueCallbacks(const somelib::OpaqueCallbacks&) = delete;
    OpaqueCallbacks(somelib::OpaqueCallbacks&&) noexcept = delete;
    OpaqueCallbacks operator=(const somelib::OpaqueCallbacks&) = delete;
    OpaqueCallbacks operator=(somelib::OpaqueCallbacks&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_OpaqueCallbacks_D_HPP
