#ifndef SOMELIB_Utf16Wrap_D_HPP
#define SOMELIB_Utf16Wrap_D_HPP

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
namespace capi { struct Utf16Wrap; }
class Utf16Wrap;
} // namespace somelib



namespace somelib {
namespace capi {
    struct Utf16Wrap;
    extern "C" {
    void Utf16Wrap_destroy(Utf16Wrap* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class Utf16Wrap;
using Utf16WrapRef = somelib::diplomat::Ref<Utf16Wrap, const somelib::capi::Utf16Wrap>;
using Utf16WrapRefMut = somelib::diplomat::Ref<Utf16Wrap, somelib::capi::Utf16Wrap>;

class Utf16Wrap : public somelib::diplomat::OpaquePointer<Utf16Wrap, somelib::capi::Utf16Wrap, somelib::capi::Utf16Wrap_destroy> {
public:

  inline static somelib::Utf16Wrap from_utf16(std::u16string_view input);

  inline std::string get_debug_str() const;
  template<typename W>
  inline void get_debug_str_write(W& writeable_output) const;

  inline std::u16string_view borrow_cont() const DIPLOMAT_LIFETIME_BOUND;

};

} // namespace
#endif // SOMELIB_Utf16Wrap_D_HPP
