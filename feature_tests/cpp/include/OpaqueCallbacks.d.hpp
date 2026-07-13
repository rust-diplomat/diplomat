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
#include "MyString.d.hpp"
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
    extern "C" {
    void OpaqueCallbacks_destroy(OpaqueCallbacks* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class OpaqueCallbacks;
using OpaqueCallbacksRef = somelib::diplomat::Ref<OpaqueCallbacks, const somelib::capi::OpaqueCallbacks>;
using OpaqueCallbacksRefMut = somelib::diplomat::Ref<OpaqueCallbacks, somelib::capi::OpaqueCallbacks>;

class OpaqueCallbacks : public somelib::diplomat::OpaquePointer<OpaqueCallbacks, somelib::capi::OpaqueCallbacks, somelib::capi::OpaqueCallbacks_destroy> {
public:

  inline static somelib::MyStringRef ret_op(std::function<somelib::MyStringRef(somelib::MyStringRef)> f, const somelib::MyString& st);

  inline static somelib::OpaqueCallbacks ctor(std::function<somelib::MyStringRef(somelib::MyStringRef)> f, const somelib::MyString& st);

  inline somelib::MyStringRef opaque_cb_self(std::function<somelib::MyStringRef(somelib::MyStringRef)> cb, const somelib::MyString& st) const;

  inline somelib::MyStringRef opaque_cb_mut_self(std::function<somelib::MyStringRef(somelib::MyStringRef)> cb, const somelib::MyString& st);

};

} // namespace
#endif // SOMELIB_OpaqueCallbacks_D_HPP
