#ifndef LLT_H
#define LLT_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Project Project;

int32_t add(int32_t a, int32_t b);

struct Project *project_new(void);

void project_free(struct Project *pp);

struct Project *project_(struct Project *pp);

#endif  /* LLT_H */
