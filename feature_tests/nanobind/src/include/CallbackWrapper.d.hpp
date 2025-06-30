#ifndef CallbackWrapper_D_HPP
#define CallbackWrapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct MyString; }
class MyString;
struct CallbackTestingStruct;


namespace diplomat {
namespace capi {
    struct CallbackWrapper {
      bool cant_be_empty;
    };

    typedef struct CallbackWrapper_option {union { CallbackWrapper ok; }; bool is_ok; } CallbackWrapper_option;


    // TODO: Need to add Mut types.
    typedef struct DiplomatCallbackWrapperView {
      const CallbackWrapper* data;
      size_t len;
    } DiplomatCallbackWrapperView;
} // namespace capi
} // namespace


struct CallbackWrapper {
  bool cant_be_empty;

  inline static int32_t test_multi_arg_callback(std::function<int32_t(int32_t)> f, int32_t x);

  inline static int32_t test_no_args(std::function<void()> h);

  inline static int32_t test_cb_with_struct(std::function<int32_t(CallbackTestingStruct)> f);

  inline static int32_t test_multiple_cb_args(std::function<int32_t()> f, std::function<int32_t(int32_t)> g);

  inline static int32_t test_str_cb_arg(std::function<int32_t(std::string_view)> f);

  inline static void test_opaque_cb_arg(std::function<void(MyString&)> cb, MyString& a);

  inline diplomat::capi::CallbackWrapper AsFFI() const;
  inline static CallbackWrapper FromFFI(diplomat::capi::CallbackWrapper c_struct);
};


#endif // CallbackWrapper_D_HPP
