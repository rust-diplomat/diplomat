#include <iostream>
#include "../include/CallbackWrapper.hpp"
#include "assert.hpp"

int main(int argc, char *argv[])
{
    CallbackWrapper o;
    int32_t tmp = 0;
    {
        auto out = o.test_multi_arg_callback([&tmp](int32_t a)
                                             { tmp = a; return a+5; }, 5);
        simple_assert_eq("multi_arg_callback arg ", tmp, 15);
        simple_assert_eq("multi_arg_callback output", out, 20);
    }
    {
        tmp = 1;
        auto out = o.test_no_args([&tmp]()
                                  { tmp++; });
        simple_assert_eq("test_no_args arg ", tmp, 2);
        simple_assert_eq("test_no_args output", out, -5);
    }
    {
        tmp = 0;
        auto out = o.test_cb_with_struct([&tmp](CallbackTestingStruct a)
                                         { tmp = a.y; return a.x+a.y; });
        simple_assert_eq("test_cb_with_struct arg ", tmp, 5);
        simple_assert_eq("test_cb_with_struct output", out, 6);
    }
    {
        tmp = 0;
        int32_t tmp2 = 0;
        auto out = o.test_multiple_cb_args([&tmp]()
                                           { tmp = 4; return 10; }, [&tmp2](int32_t a)
                                           {tmp2 = a; return a+1; });
        simple_assert_eq("test_multiple_cb_args arg ", tmp, 4);
        simple_assert_eq("test_multiple_cb_args arg2 ", tmp2, 5);
        simple_assert_eq("test_multiple_cb_args output", out, 16);
    }
    {
        auto out = o.test_str_cb_arg([](std::string_view a)
                                     { return a.length(); });
        simple_assert_eq("test_str_cb_arg output", out, 7);
    }
}
