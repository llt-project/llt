#include "context.h"
#include "api.fn.h"


static Arena arena_api;

void llt_init_memory(size_t size)
{
    arena_init(&arena_api, size);
}


Project* InitProject(Context *ctx, Module *modules, int32_t count)
{
    (void)ctx;

    Project *p = (Project*)malloc(sizeof(Project));
    if (!p) return NULL;

    p->module_count = count;
    p->modules = modules;

    return p;
}