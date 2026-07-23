#ifndef SOMELIB_MyString_D_HPP
#define SOMELIB_MyString_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Float64Vec.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct Float64Vec; }
class Float64Vec;
namespace capi { struct MyString; }
class MyString;
} // namespace somelib



namespace somelib {
namespace capi {
    struct MyString;

    typedef struct DiplomatMyStringView {
      const MyString** data;
      size_t len;
    } DiplomatMyStringView;
    extern "C" {
    void MyString_destroy(MyString* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class MyString;
using MyStringRef = somelib::diplomat::Ref<MyString, const somelib::capi::MyString>;
using MyStringRefMut = somelib::diplomat::Ref<MyString, somelib::capi::MyString>;

class MyString : public somelib::diplomat::OpaquePointer<MyString, somelib::capi::MyString, somelib::capi::MyString_destroy> {
public:

  inline static somelib::MyString new_(std::string_view v = { "T", 1 });

  inline static somelib::diplomat::result<somelib::MyString, somelib::diplomat::Utf8Error> new_unsafe(std::string_view v);

  inline static somelib::MyString new_from_first(somelib::diplomat::span<const diplomat::string_view_for_slice> v);

  inline static somelib::MyString new_from_utf16(somelib::diplomat::span<const diplomat::u16string_view_for_slice> v);

  inline void set_str(std::string_view new_str);

  inline std::string get_str() const;
  template<typename W>
  inline void get_str_write(W& writeable_output) const;

  inline static std::string_view get_static_str();

  inline static somelib::diplomat::result<std::string, somelib::diplomat::Utf8Error> string_transform(std::string_view foo);
  template<typename W>
  inline static somelib::diplomat::result<std::monostate, somelib::diplomat::Utf8Error> string_transform_write(std::string_view foo, W& writeable_output);

  inline std::string_view borrow() const DIPLOMAT_LIFETIME_BOUND;

  inline static std::string slice_of_opaques(somelib::diplomat::span<somelib::MyString> sl);
  template<typename W>
  inline static void slice_of_opaques_write(somelib::diplomat::span<somelib::MyString> sl, W& writeable_output);

  inline static std::string optional_slice_of_opaques(somelib::diplomat::span<somelib::diplomat::Optional<somelib::MyStringRef>> sl);
  template<typename W>
  inline static void optional_slice_of_opaques_write(somelib::diplomat::span<somelib::diplomat::Optional<somelib::MyStringRef>> sl, W& writeable_output);

  inline static std::string other_opaque_type(somelib::diplomat::span<somelib::Float64Vec> other);
  template<typename W>
  inline static void other_opaque_type_write(somelib::diplomat::span<somelib::Float64Vec> other, W& writeable_output);

};

} // namespace
#endif // SOMELIB_MyString_D_HPP
