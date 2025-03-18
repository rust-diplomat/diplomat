#ifndef OptionOpaque_D_HPP
#define OptionOpaque_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"

struct OptionInputStruct;
struct OptionStruct;
class OptionEnum;


namespace diplomat {
namespace capi {
    struct OptionOpaque;
} // namespace capi
} // namespace

class OptionOpaque {
public:

  inline static std::unique_ptr<OptionOpaque> new_(int32_t i);

  inline static std::unique_ptr<OptionOpaque> new_none();

  inline static std::optional<OptionStruct> returns();

  inline std::optional<intptr_t> option_isize() const;

  inline std::optional<size_t> option_usize() const;

  inline std::optional<int32_t> option_i32() const;

  inline std::optional<uint32_t> option_u32() const;

  inline static OptionStruct new_struct();

  inline static OptionStruct new_struct_nones();

  inline void assert_integer(int32_t i) const;

  inline static bool option_opaque_argument(const OptionOpaque* arg);

  inline static std::optional<uint8_t> accepts_option_u8(std::optional<uint8_t> arg);

  inline static std::optional<OptionEnum> accepts_option_enum(std::optional<OptionEnum> arg);

  inline static std::optional<OptionInputStruct> accepts_option_input_struct(std::optional<OptionInputStruct> arg);

  inline static OptionInputStruct returns_option_input_struct();

  inline const diplomat::capi::OptionOpaque* AsFFI() const;
  inline diplomat::capi::OptionOpaque* AsFFI();
  inline static const OptionOpaque* FromFFI(const diplomat::capi::OptionOpaque* ptr);
  inline static OptionOpaque* FromFFI(diplomat::capi::OptionOpaque* ptr);
  inline static void operator delete(void* ptr);
private:
  OptionOpaque() = delete;
  OptionOpaque(const OptionOpaque&) = delete;
  OptionOpaque(OptionOpaque&&) noexcept = delete;
  OptionOpaque operator=(const OptionOpaque&) = delete;
  OptionOpaque operator=(OptionOpaque&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // OptionOpaque_D_HPP
