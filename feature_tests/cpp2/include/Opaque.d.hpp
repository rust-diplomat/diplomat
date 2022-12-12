#ifndef Opaque_D_HPP
#define Opaque_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ImportedStruct.d.hpp"
#include "MyStruct.d.hpp"
#include "Opaque.d.h"


class Opaque {
public:

  inline static std::unique_ptr<Opaque> new_();

  inline void assert_struct(MyStruct s) const;

  inline static size_t returns_usize();

  inline static ImportedStruct returns_imported();

  inline const capi::Opaque* AsFFI() const;
  inline capi::Opaque* AsFFI();
  inline static const Opaque* FromFFI(const capi::Opaque* ptr);
  inline static Opaque* FromFFI(capi::Opaque* ptr);
  inline ~Opaque();
private:
  Opaque() = delete;
};


#endif // Opaque_D_HPP
