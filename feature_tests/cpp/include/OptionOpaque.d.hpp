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
struct OptionInputStruct;
struct OptionStruct;
class OptionEnum;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OptionOpaque;
} // namespace capi
} // namespace

namespace somelib {
class OptionOpaque {
public:

  inline static std::unique_ptr<somelib::OptionOpaque> new_(int32_t i);

  inline static std::unique_ptr<somelib::OptionOpaque> new_none();

  inline static std::optional<somelib::OptionStruct> returns();

  inline std::optional<intptr_t> option_isize() const;

  inline std::optional<size_t> option_usize() const;

  inline std::optional<int32_t> option_i32() const;

  inline std::optional<uint32_t> option_u32() const;

  inline static somelib::OptionStruct new_struct();

  inline static somelib::OptionStruct new_struct_nones();

  inline const somelib::OptionOpaque* returns_none_self() const;

  inline const somelib::OptionOpaque* returns_some_self() const;

  inline void assert_integer(int32_t i) const;

  inline static bool option_opaque_argument(const somelib::OptionOpaque* arg);

  inline static std::optional<uint8_t> accepts_option_u8(std::optional<uint8_t> arg, uint8_t sentinel);

  inline static std::optional<somelib::OptionEnum> accepts_option_enum(std::optional<somelib::OptionEnum> arg, uint8_t sentinel);

  inline static std::optional<somelib::OptionEnum> accepts_multiple_option_enum(uint8_t sentinel1, std::optional<somelib::OptionEnum> arg1, std::optional<somelib::OptionEnum> arg2, std::optional<somelib::OptionEnum> arg3, uint8_t sentinel2);

  inline static std::optional<somelib::OptionInputStruct> accepts_option_input_struct(std::optional<somelib::OptionInputStruct> arg, uint8_t sentinel);

  inline static somelib::OptionInputStruct returns_option_input_struct();

  inline static size_t accepts_option_str(std::optional<std::string_view> arg, uint8_t sentinel);

  inline static bool accepts_option_str_slice(std::optional<somelib::diplomat::span<const diplomat::string_view_for_slice>> arg, uint8_t sentinel);

  inline static int64_t accepts_option_primitive(std::optional<somelib::diplomat::span<const uint32_t>> arg, uint8_t sentinel);

    inline const somelib::capi::OptionOpaque* AsFFI() const;
    inline somelib::capi::OptionOpaque* AsFFI();
    inline static const somelib::OptionOpaque* FromFFI(const somelib::capi::OptionOpaque* ptr);
    inline static somelib::OptionOpaque* FromFFI(somelib::capi::OptionOpaque* ptr);
    inline static void operator delete(void* ptr);
private:
    OptionOpaque() = delete;
    OptionOpaque(const somelib::OptionOpaque&) = delete;
    OptionOpaque(somelib::OptionOpaque&&) noexcept = delete;
    OptionOpaque operator=(const somelib::OptionOpaque&) = delete;
    OptionOpaque operator=(somelib::OptionOpaque&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_OptionOpaque_D_HPP
