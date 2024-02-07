#include <iostream>
#include "../include/AttrOpaque1.hpp"
#include "assert.hpp"

int main(int argc, char *argv[]) {
    AttrOpaque1 o = AttrOpaque1::new_();
    // the cpp2 renames don't apply. However, these must link correctly!!
    simple_assert_eq("method should call", o.method(), 77);
    simple_assert_eq("method should call", o.abirenamed(), 123);

    // These C names should also resolve
    void* renamed = (void*)capi::renamed_on_abi_only;
    std::cout<<"Renamed function at "<<renamed<<std::endl;
    renamed = (void*)capi::namespace_AttrOpaque1_method;
    std::cout<<"Renamed function at "<<renamed<<std::endl;
}
