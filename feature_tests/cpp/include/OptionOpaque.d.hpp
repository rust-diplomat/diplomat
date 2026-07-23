#ifndef SOMELIB_OptionOpaque_D_HPP
#define SOMELIB_OptionOpaque_D_HPP

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
namespace capi { struct OptionOpaque; }
class OptionOpaque;
struct BorrowingOptionStruct;
struct OptionInputStruct;
struct OptionStruct;
class OptionEnum;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OptionOpaque;
    extern "C" {
    void OptionOpaque_destroy(OptionOpaque* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class OptionOpaque;
using OptionOpaqueRef = somelib::diplomat::Ref<OptionOpaque, const somelib::capi::OptionOpaque>;
using OptionOpaqueRefMut = somelib::diplomat::Ref<OptionOpaque, somelib::capi::OptionOpaque>;

class OptionOpaque : public somelib::diplomat::OpaquePointer<OptionOpaque, somelib::capi::OptionOpaque, somelib::capi::OptionOpaque_destroy> {
public:

  inline static somelib::diplomat::Optional<somelib::OptionOpaque> new_(int32_t i);

  inline static somelib::diplomat::Optional<somelib::OptionOpaque> new_none();

  inline static somelib::diplomat::Optional<somelib::OptionStruct> returns();

  inline somelib::diplomat::Optional<intptr_t> option_isize() const;

  inline somelib::diplomat::Optional<size_t> option_usize() const;

  inline somelib::diplomat::Optional<int32_t> option_i32() const;

  inline somelib::diplomat::Optional<uint32_t> option_u32() const;

  inline static somelib::OptionStruct new_struct();

  inline static somelib::OptionStruct new_struct_nones();

  inline somelib::diplomat::Optional<somelib::OptionOpaqueRef> returns_none_self() const DIPLOMAT_LIFETIME_BOUND;

  inline somelib::diplomat::Optional<somelib::OptionOpaqueRef> returns_some_self() const DIPLOMAT_LIFETIME_BOUND;

  inline void assert_integer(int32_t i) const;

  inline static bool option_opaque_argument(somelib::diplomat::Optional<somelib::OptionOpaqueRef> arg);

  inline static somelib::diplomat::Optional<uint8_t> accepts_option_u8(somelib::diplomat::Optional<uint8_t> arg, uint8_t sentinel);

  inline static somelib::diplomat::Optional<somelib::OptionEnum> accepts_option_enum(somelib::diplomat::Optional<somelib::OptionEnum> arg, uint8_t sentinel);

  inline static void accepts_borrowing_option_struct(somelib::BorrowingOptionStruct arg);

  inline static somelib::diplomat::Optional<somelib::OptionEnum> accepts_multiple_option_enum(uint8_t sentinel1, somelib::diplomat::Optional<somelib::OptionEnum> arg1, somelib::diplomat::Optional<somelib::OptionEnum> arg2, somelib::diplomat::Optional<somelib::OptionEnum> arg3, uint8_t sentinel2);

  inline static somelib::diplomat::Optional<somelib::OptionInputStruct> accepts_option_input_struct(somelib::diplomat::Optional<somelib::OptionInputStruct> arg, uint8_t sentinel);

  inline static somelib::OptionInputStruct returns_option_input_struct();

  inline static size_t accepts_option_str(somelib::diplomat::Optional<std::string_view> arg, uint8_t sentinel);

  inline static bool accepts_option_str_slice(somelib::diplomat::Optional<somelib::diplomat::span<const diplomat::string_view_for_slice>> arg, uint8_t sentinel);

  inline static int64_t accepts_option_primitive(somelib::diplomat::Optional<somelib::diplomat::span<const uint32_t>> arg, uint8_t sentinel);

};

} // namespace
#endif // SOMELIB_OptionOpaque_D_HPP
