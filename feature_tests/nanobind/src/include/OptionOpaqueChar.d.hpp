#ifndef SOMELIB_OptionOpaqueChar_D_HPP
#define SOMELIB_OptionOpaqueChar_D_HPP

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
    struct OptionOpaqueChar;
    extern "C" {
    void OptionOpaqueChar_destroy(OptionOpaqueChar* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class OptionOpaqueChar;
using OptionOpaqueCharRef = somelib::diplomat::Ref<OptionOpaqueChar, const somelib::capi::OptionOpaqueChar>;
using OptionOpaqueCharRefMut = somelib::diplomat::Ref<OptionOpaqueChar, somelib::capi::OptionOpaqueChar>;

class OptionOpaqueChar : public somelib::diplomat::OpaquePointer<OptionOpaqueChar, somelib::capi::OptionOpaqueChar, somelib::capi::OptionOpaqueChar_destroy> {
public:

  inline void assert_char(char32_t ch) const;

};

} // namespace
#endif // SOMELIB_OptionOpaqueChar_D_HPP
