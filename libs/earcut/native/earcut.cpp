#include "./lib.hpp"
#include <array>

template<typename T>
struct Ring {
public:
    typedef std::array<T, 2> value_type;

    const value_type *data;
    size_t count;

    size_t size() const {
        return count;
    }

    bool empty() const {
        return count == 0;
    }

    value_type const &operator[](int idx) const {
        return data[idx];
    }
};

#include "./earcut.hpp"
#include <vector>
#include <stdint.h>


struct Earcut {
    std::vector <Ring<COORD_TYPE>> rings;
    std::vector <INDEX_TYPE> indices;
};

extern "C" Earcut *earcut_new() {
    return new Earcut{};
}

extern "C" void earcut_delete(Earcut *e) {
    delete e;
}

extern "C" void earcut_ring(Earcut *e, const COORD_TYPE *data, size_t count) {
    assert(sizeof(Ring<COORD_TYPE>::value_type) == 8);
    assert(count % 2 == 0);
    e->rings.push_back(Ring<COORD_TYPE>{reinterpret_cast<const Ring<COORD_TYPE>::value_type *> (data), count / 2});
}

extern "C" bool earcut_tesselate(Earcut *e) {
    e->indices = mapbox::earcut(e->rings);
    return true;
}

extern "C" INDEX_TYPE *earcut_data(Earcut *e) {
    return e->indices.data();
}

extern "C" size_t earcut_size(Earcut *e) {
    return e->indices.size();
}

