#ifndef ns_RenamedMyIterator_D_HPP
#define ns_RenamedMyIterator_D_HPP

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
    struct RenamedMyIterator;
} // namespace capi
} // namespace

namespace ns {
class RenamedMyIterator {
public:

  inline std::optional<uint8_t> next();

    inline const ns::capi::RenamedMyIterator* AsFFI() const;
    inline ns::capi::RenamedMyIterator* AsFFI();
    inline static const ns::RenamedMyIterator* FromFFI(const ns::capi::RenamedMyIterator* ptr);
    inline static ns::RenamedMyIterator* FromFFI(ns::capi::RenamedMyIterator* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedMyIterator() = delete;
    RenamedMyIterator(const ns::RenamedMyIterator&) = delete;
    RenamedMyIterator(ns::RenamedMyIterator&&) noexcept = delete;
    RenamedMyIterator operator=(const ns::RenamedMyIterator&) = delete;
    RenamedMyIterator operator=(ns::RenamedMyIterator&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedMyIterator_D_HPP
