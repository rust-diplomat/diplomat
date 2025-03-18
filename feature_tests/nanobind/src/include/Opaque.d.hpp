#ifndef Opaque_D_HPP
#define Opaque_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"

struct ImportedStruct;
struct MyStruct;


namespace diplomat {
namespace capi {
    struct Opaque;
} // namespace capi
} // namespace

class Opaque {
public:

  inline static std::unique_ptr<Opaque> new_();

  inline static std::unique_ptr<Opaque> try_from_utf8(std::string_view input);

  inline static diplomat::result<std::unique_ptr<Opaque>, diplomat::Utf8Error> from_str(std::string_view input);

  inline std::string get_debug_str() const;

  inline void assert_struct(MyStruct s) const;

  inline static size_t returns_usize();

  inline static ImportedStruct returns_imported();

  inline static int8_t cmp();

  inline const diplomat::capi::Opaque* AsFFI() const;
  inline diplomat::capi::Opaque* AsFFI();
  inline static const Opaque* FromFFI(const diplomat::capi::Opaque* ptr);
  inline static Opaque* FromFFI(diplomat::capi::Opaque* ptr);
  inline static void operator delete(void* ptr);
private:
  Opaque() = delete;
  Opaque(const Opaque&) = delete;
  Opaque(Opaque&&) noexcept = delete;
  Opaque operator=(const Opaque&) = delete;
  Opaque operator=(Opaque&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Opaque_D_HPP
