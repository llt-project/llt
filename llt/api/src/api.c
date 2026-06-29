#include "api.h"
#include "arena.h"

Context* context_init() 
{
    Context *ctx = arena_alloc(&g_arena, sizeof(Context));
    if (!ctx) return NULL;
    return ctx;
}

Project* project_init(Context *ctx, Plugins *plgs) 
{
    Project *p = arena_alloc(&g_arena, sizeof(Project));
    if (!p) return NULL;

    p->plugins = plgs;
    p->context = ctx;
    return p;
}