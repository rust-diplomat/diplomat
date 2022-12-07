#ifndef RefList_H
#define RefList_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


class RefListParameter;


class RefList;



std::unique_ptr<RefList> RefList_node(const RefListParameter& data);
void RefList_destroy(RefList* self);


#endif // RefList_HPP
