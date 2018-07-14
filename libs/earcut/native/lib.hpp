//
// Created by semtexzv on 7/2/18.
//

#ifndef RMAPS_LIB_H
#define RMAPS_LIB_H

#include <stddef.h>
#include <stdint.h>

typedef int32_t COORD_TYPE;
typedef uint32_t INDEX_TYPE;
struct Earcut;

#ifdef __cplusplus
extern "C" {
#endif

Earcut *earcut_new();

void earcut_delete(Earcut *);

void earcut_ring(Earcut *, const COORD_TYPE *data, size_t count);

bool earcut_tesselate(Earcut *);

INDEX_TYPE *earcut_data(Earcut *);

size_t earcut_size(Earcut *);

#ifdef __cplusplus
}
#endif

#endif //RMAPS_LIB_H
