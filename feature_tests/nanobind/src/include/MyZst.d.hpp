#ifndef MyZst_D_HPP
#define MyZst_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    // TODO: Need to add Mut types.
    typedef struct DiplomatMyZstView {
      const MyZst* data;
      size_t len;
    } DiplomatMyZstView;
} // namespace capi
} // namespace


struct MyZst {

};


#endif // MyZst_D_HPP
