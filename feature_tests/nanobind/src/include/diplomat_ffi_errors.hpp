
#include "FFIError.d.hpp"


// Helpers for callbacks and traits, to grab the representation of a struct when a callback or trait has failed.

namespace somelib {

template<typename T>
struct get_ffi_error;



template<>
struct get_ffi_error<FFIError> {
    static constexpr FFIError get() {
        return FFIError::Value::FFI;
    }
};



}
