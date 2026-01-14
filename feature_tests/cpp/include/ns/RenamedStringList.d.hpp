#ifndef SOMELIB_STRING_LIST_D_HPP
#define SOMELIB_STRING_LIST_D_HPP

#include "../diplomat_runtime.hpp"

namespace somelib::ns {
    namespace capi {
        struct RenamedStringList;
    }
    class RenamedStringList {
        public:
            std::vector<std::string> return_new();
    }
}

#endif