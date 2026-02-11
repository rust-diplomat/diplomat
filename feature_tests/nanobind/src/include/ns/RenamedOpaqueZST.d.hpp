#ifndef SOMELIB_ns_RenamedOpaqueZST_D_HPP
#define SOMELIB_ns_RenamedOpaqueZST_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
namespace somelib {
namespace ns {
namespace capi { struct RenamedOpaqueZST; }
class RenamedOpaqueZST;
namespace capi { struct RenamedOpaqueZSTIterator; }
class RenamedOpaqueZSTIterator;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueZST;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueZST {
public:

  inline static std::unique_ptr<somelib::ns::RenamedOpaqueZST> ctor();

  inline static std::unique_ptr<somelib::ns::RenamedOpaqueZST> make();

  inline static std::string out_string();
  template<typename W>
  inline static void out_string_write(W& writeable_output);

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> member() const;

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> mut_member();

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> operator+(const somelib::ns::RenamedOpaqueZST& _o) const;

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> operator-(const somelib::ns::RenamedOpaqueZST& _o) const;

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> operator*(const somelib::ns::RenamedOpaqueZST& _o) const;

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> operator/(const somelib::ns::RenamedOpaqueZST& _o) const;

  inline static somelib::diplomat::result<std::unique_ptr<somelib::ns::RenamedOpaqueZST>, std::monostate> success_zst(bool return_success);

  inline static somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ns::RenamedOpaqueZST>> fail_zst(bool return_success);

  inline static somelib::diplomat::result<std::unique_ptr<somelib::ns::RenamedOpaqueZST>, std::unique_ptr<somelib::ns::RenamedOpaqueZST>> success_fail_zst(bool return_success);

  inline static std::unique_ptr<somelib::ns::RenamedOpaqueZST> optional_zst(bool is_some);

  inline static std::unique_ptr<somelib::ns::RenamedOpaqueZST> static_getter();

  inline static void static_setter(const somelib::ns::RenamedOpaqueZST& _a);

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> getter() const;

  inline void setter(const somelib::ns::RenamedOpaqueZST& _a) const;

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator> iter() const;
  inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedOpaqueZSTIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZST> operator[](size_t _idx) const;

    inline const somelib::ns::capi::RenamedOpaqueZST* AsFFI() const;
    inline somelib::ns::capi::RenamedOpaqueZST* AsFFI();
    inline static const somelib::ns::RenamedOpaqueZST* FromFFI(const somelib::ns::capi::RenamedOpaqueZST* ptr);
    inline static somelib::ns::RenamedOpaqueZST* FromFFI(somelib::ns::capi::RenamedOpaqueZST* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueZST() = delete;
    RenamedOpaqueZST(const somelib::ns::RenamedOpaqueZST&) = delete;
    RenamedOpaqueZST(somelib::ns::RenamedOpaqueZST&&) noexcept = delete;
    RenamedOpaqueZST operator=(const somelib::ns::RenamedOpaqueZST&) = delete;
    RenamedOpaqueZST operator=(somelib::ns::RenamedOpaqueZST&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueZST_D_HPP
