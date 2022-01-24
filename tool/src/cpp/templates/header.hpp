#ifndef {{ typ_name }}_HPP
#define {{ typ_name }}_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
{%- for header in headers %}
{{ header }}
{%- endfor %}
#include "diplomat_runtime.hpp"

namespace capi {
#include "{{ typ_name }}.h"
}
