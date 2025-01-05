#include <metal_stdlib>

using namespace metal;

kernel void base_field_batch_inverse(
    const device uint* column [[buffer(0)]],
    device uint* result [[buffer(1)]],
    device uint* size_pointer [[buffer(2)]],
    device uint* log_size_pointer [[buffer(3)]],
    device uint* shared_memory_size_pointer [[buffer(4)]],
    device uint* log_shared_memory_size_pointer [[buffer(5)]],
    threadgroup uint* shared_element_tree [[threadgroup(0)]],
    uint tid [[thread_position_in_threadgroup]],
    uint id [[thread_position_in_grid]]
) {
    uint size = *size_pointer;
    uint log_size = *log_size_pointer;
    if(size > *shared_memory_size_pointer) {
        size = *shared_memory_size_pointer;
        log_size = *log_shared_memory_size_pointer;
    }

    // Example
    shared_element_tree[tid] = size;
    // threadgroup_barrier(mem_flags::mem_threadgroup);
    if (id < size) {
        result[id] = shared_element_tree[tid];
    }
}