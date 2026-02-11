#include "../../raylib/src/raylib.h"
#include "../../raylib/src/rcamera.h"
#include "../../raylib/src/rlgl.h"

#define RAYMATH_IMPLEMENTATION
#include "../../raylib/src/raymath.h"

Matrix MatrixLookAt(Vector3 eye, Vector3 target, Vector3 up);
float16 MatrixToFloatV(Matrix mat);
