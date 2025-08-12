#ifndef test_directory_RenamedDifferentDirectory_D_HPP
#define test_directory_RenamedDifferentDirectory_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    struct RenamedDifferentDirectory;
} // namespace capi
} // namespace

namespace ns {
class RenamedDifferentDirectory {
public:

  inline const ns::capi::RenamedDifferentDirectory* AsFFI() const;
  inline ns::capi::RenamedDifferentDirectory* AsFFI();
  inline static const ns::RenamedDifferentDirectory* FromFFI(const ns::capi::RenamedDifferentDirectory* ptr);
  inline static ns::RenamedDifferentDirectory* FromFFI(ns::capi::RenamedDifferentDirectory* ptr);
  inline static void operator delete(void* ptr);
private:
  RenamedDifferentDirectory() = delete;
  RenamedDifferentDirectory(const ns::RenamedDifferentDirectory&) = delete;
  RenamedDifferentDirectory(ns::RenamedDifferentDirectory&&) noexcept = delete;
  RenamedDifferentDirectory operator=(const ns::RenamedDifferentDirectory&) = delete;
  RenamedDifferentDirectory operator=(ns::RenamedDifferentDirectory&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // test_directory_RenamedDifferentDirectory_D_HPP
