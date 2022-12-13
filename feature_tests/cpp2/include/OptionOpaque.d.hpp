#ifndef OptionOpaque_D_HPP
#define OptionOpaque_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "OptionOpaque.d.h"
#include "OptionStruct.d.hpp"

struct OptionStruct;


class OptionOpaque {
public:

  inline static std::unique_ptr<OptionOpaque> new_(int32_t i);

  inline static std::unique_ptr<OptionOpaque> new_none();

  inline static OptionStruct new_struct();

  inline static OptionStruct new_struct_nones();

  inline void assert_integer(int32_t i) const;

  inline static bool option_opaque_argument(std::optional<const std::reference_wrapper<OptionOpaque>> arg);

  inline const capi::OptionOpaque* AsFFI() const;
  inline capi::OptionOpaque* AsFFI();
  inline static const OptionOpaque* FromFFI(const capi::OptionOpaque* ptr);
  inline static OptionOpaque* FromFFI(capi::OptionOpaque* ptr);
  inline ~OptionOpaque();
private:
  OptionOpaque() = delete;
};


#endif // OptionOpaque_D_HPP
