#include <metal_stdlib>

using namespace metal;

kernel void base_field_batch_inverse(
    const device uint* column [[buffer(0)]],
    device uint* result [[buffer(1)]],
    device uint* size_pointer [[buffer(2)]],
    device uint* log_size_pointer [[buffer(3)]],
    uint tid [[thread_position_in_threadgroup]],
    uint id [[thread_position_in_grid]]
) {
    threadgroup uint shared_example[256];
    shared_example[tid] = *log_size_pointer;
//    threadgroup_barrier(mem_flags::mem_threadgroup);
    if (id < *size_pointer) {
        result[id] = shared_example[tid];
    }
}