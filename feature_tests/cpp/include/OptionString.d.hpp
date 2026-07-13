#ifndef SOMELIB_OptionString_D_HPP
#define SOMELIB_OptionString_D_HPP

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
namespace capi { struct OptionString; }
class OptionString;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OptionString;
    extern "C" {
    void OptionString_destroy(OptionString* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class OptionString;
using OptionStringRef = somelib::diplomat::Ref<OptionString, const somelib::capi::OptionString>;
using OptionStringRefMut = somelib::diplomat::Ref<OptionString, somelib::capi::OptionString>;

class OptionString : public somelib::diplomat::OpaquePointer<OptionString, somelib::capi::OptionString, somelib::capi::OptionString_destroy> {
public:

  inline static somelib::diplomat::Optional<somelib::OptionString> new_(std::string_view diplomat_str);

  inline somelib::diplomat::result<std::string, std::monostate> write() const;
  template<typename W>
  inline somelib::diplomat::result<std::monostate, std::monostate> write_write(W& writeable_output) const;

  inline somelib::diplomat::Optional<std::string_view> borrow() const DIPLOMAT_LIFETIME_BOUND;

};

} // namespace
#endif // SOMELIB_OptionString_D_HPP
