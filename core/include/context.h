// Author: Y.P.
#ifndef CONTEXT
#define CONTEXT

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#include "ir/type.h"
#include "llt/arena.h"


typedef struct context Context;
typedef struct module Module;

typedef struct project Project;

struct context 
{

    Symbol *symbols;
    TypeKind *types;

    size_t symbol_count;
    size_t type_count;
};

struct project
{
    int module_count;
    Module *modules;
};

#endif