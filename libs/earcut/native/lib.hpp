//
// Created by semtexzv on 7/2/18.
//

#ifndef RMAPS_LIB_H
#define RMAPS_LIB_H


template<typename T>
struct Point {
    T x;
    T y;
};


template<typename T>
void earcut(Point<T>* data, int count);


template<>
void earcut(Point<int>* data, int count);
#endif //RMAPS_LIB_H
