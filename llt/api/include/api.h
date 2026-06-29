#ifndef API
#define API

#include "arena.h"
#include "plugin.h"

typedef struct context {

} Context;

typedef struct project {
    Plugins *plugins;
    Context *context;

    Arena arena;
} Project;

Context* context_init();
Project* project_init(Context *ctx, Plugins *plgs);

#endif