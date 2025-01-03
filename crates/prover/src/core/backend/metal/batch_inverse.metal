#include <metal_stdlib>

using namespace metal;

kernel void add_arrays(const device float* in1 [[buffer(0)]],
                       const device float* in2 [[buffer(1)]],
                       device float* result [[buffer(2)]],
                       uint id [[thread_position_in_grid]]) {
    result[id] = in1[id] + in2[id];
}