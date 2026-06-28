// Author: Y.P.
#ifndef CONTEXT
#define CONTEXT

#include "type.h"

typedef struct context Context;
typedef struct symbol  Symbol;
typedef struct module Module;

typedef struct project Project;

struct context 
{
    Arena *arena;

    Symbol *symbols;
    TypeKind *types;

    size_t symbol_count;
    size_t type_count;
};

struct symbol 
{
    const char *name;
    void *definition;
};

struct project
{
    int module_count;
    Module *modules;
};

#endif