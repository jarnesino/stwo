#include <metal_stdlib>

using namespace metal;


// ************** Fields **************

constant uint P = 2147483647;

uint mul(uint a, uint b) {
    uint64_t v = ((uint64_t) a * (uint64_t) b);
    uint64_t w = v + (v >> 31);
    uint64_t u = v + (w >> 31);
    return u & P;
}

uint64_t pow_to_power_of_two(int n, uint t) {
    int i = 0;
    while (i < n) {
        t = mul(t, t);
        i++;
    }
    return t;
}

uint inv(uint t) {
    uint64_t t0 = mul(pow_to_power_of_two(2, t), t);
    uint64_t t1 = mul(pow_to_power_of_two(1, t0), t0);
    uint64_t t2 = mul(pow_to_power_of_two(3, t1), t0);
    uint64_t t3 = mul(pow_to_power_of_two(1, t2), t0);
    uint64_t t4 = mul(pow_to_power_of_two(8, t3), t3);
    uint64_t t5 = mul(pow_to_power_of_two(8, t4), t3);
    return mul(pow_to_power_of_two(7, t5), t2);
}

// ************************************



template<typename T>
void new_forward_parent(threadgroup T *from, threadgroup T *dst, int index) {
    // Computes the value of the parent from the multiplication of two children.
    // dst  : Pointer to the beginning of the parent's level.
    // from : Pointer to the beginning of the children level.
    // index: Index of the computed parent.
    dst[index] = mul(from[index << 1], from[(index << 1) + 1]);
}

template<typename T>
void new_backward_children(threadgroup T *from, threadgroup T *dst, int index) {
    // Computes the inverse of the two children from the inverse of the parent.
    // dst  : Pointer to the beginning of the children's level.
    // from : Pointer to the beginning of the parent's level.
    // index: Index of the computed children.
    T temp = dst[index << 1];
    dst[index << 1] = mul(from[index], dst[(index << 1) + 1]);
    dst[(index << 1) + 1] = mul(from[index], temp);
}

template<typename T>
void batch_inverse(const device T *from, device T *dst, int size, int log_size, threadgroup T *s_from, threadgroup T *s_inner_tree, uint thread_position_in_threadgroup, uint threads_per_threadgroup, uint threadgroup_position_in_grid) {
    // Input:
    // - from      : array of T representing field elements.
    // - inner_tree: array of T used as an auxiliary variable.
    // - size      : size of "from" and "inner_tree".
    // - log_size  : log(size).
    // Output:
    // - dst       : array of T with the inverses of "from".
    //
    // Variation of Montgomery's trick to leverage GPU parallelization.
    // Construct a binary tree:
    //    - from      : stores the leaves
    //    - inner_tree: stores the inner nodes and the root.
    //
    // The algorithm has three parts:
    //    - Cumulative product: each parent is the product of its children.
    //    - Compute inverse of root node
    //    - Backward pass: compute inverses of children using the fact that
    //          inv_left_child  = inv_parent * right_child
    //          inv_right_child = inv_parent * left_child
    int index = thread_position_in_threadgroup;
    int block_index = threadgroup_position_in_grid;
    int block_size = threads_per_threadgroup;

    s_from[index] = from[2 * block_index * block_size + index];
    s_from[index + block_size] = from[2 * block_index * block_size + index + block_size];
    threadgroup_barrier(mem_flags::mem_threadgroup);

    dst = &dst[2 * block_index * block_size];

    // Size tracks the number of threads working.

    size = size >> 1;

    // The first level is a special case because inner_tree and leaves
    // are stored in separate variables.
    if(index < size) {
        new_forward_parent(s_from, s_inner_tree, index);
        // from      : | a_0       | a_1       | ... | a_(n/2 - 1)       |      ...    | a_(n-1)
        // inner_tree: | a_0 * a_1 | a_2 * a_3 | ... | a_(n-2) * a_(n-1) | empty | ... | empty
    }

    int from_offset = 0;   // Offset at inner_tree to get the children.
    int dst_offset = size; // Offset at inner_tree to store the parents.
    size >>= 1;            // Next level is half the size.

    // Each step will compute one level of the inner_tree.
    // If size = 4 inner tree stores:
    // |       Level 1         |        Root           |
    // | a_0 * a_1 | a_2 * a_3 | a_0 * a_1 * a_2 * a_3 |
    // Construct tree up to the level with 32 leaves to leverage
    // SIMD synchronization within a warp
    int step = 1;
    while(step + 5 < log_size) {
        threadgroup_barrier(mem_flags::mem_threadgroup);

        if(index < size) {
            // Each thread computes one parent as the product of left and right children
            new_forward_parent(&s_inner_tree[from_offset], &s_inner_tree[dst_offset], index);
        }

        from_offset = dst_offset;       // Output of this level is input of next one.
        dst_offset = dst_offset + size; // Skip the number of nodes computed.

        size >>= 1; // Next level is half the size.
        step++;
    }

    // Compute inverse of the root.
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if(index < (size << 1)){
        s_inner_tree[from_offset + index] = inv(s_inner_tree[from_offset + index]);
    }

    // Backward Pass: compute the inverses of the children using the parents.
    step = 5;
    size = 32;
    //from_offset = dst_offset - 1;
    dst_offset = from_offset - (size << 1);
    while(step < log_size - 1) {
        threadgroup_barrier(mem_flags::mem_threadgroup);
        if(index < size) {
            // Compute children inverses from parent inverses.
            new_backward_children(&s_inner_tree[from_offset], &s_inner_tree[dst_offset], index);
        }

        size <<= 1; // Each level doubles up its size.

        from_offset = dst_offset;               // Output of this level is input of next one.
        dst_offset = from_offset - (size << 1); // Size threads work but 2*size children are computed.

        step++;
    }

    threadgroup_barrier(mem_flags::mem_threadgroup);
    // The inner_tree has all its inverses computed, now
    // we have to compute the inverses of the leaves:

    if(index < size) {
        dst[index << 1] = mul(s_inner_tree[index], s_from[(index << 1) + 1]);
        dst[(index << 1) + 1] = mul(s_inner_tree[index], s_from[index << 1]);
    }
}


kernel void base_field_batch_inverse(
    const device uint* column [[buffer(0)]],
    device uint* result [[buffer(1)]],
    device uint* size_pointer [[buffer(2)]],
    device uint* log_size_pointer [[buffer(3)]],
    device uint* shared_memory_size_pointer [[buffer(4)]],
    device uint* log_shared_memory_size_pointer [[buffer(5)]],
    threadgroup uint* shared_element_tree [[threadgroup(0)]],
    uint thread_position_in_threadgroup [[thread_position_in_threadgroup]],
    uint threads_per_threadgroup [[threads_per_threadgroup]],
    uint threadgroup_position_in_grid [[threadgroup_position_in_grid]]
) {
    uint size = *size_pointer;
    uint log_size = *log_size_pointer;
    if(size > *shared_memory_size_pointer) {
        size = *shared_memory_size_pointer;
        log_size = *log_shared_memory_size_pointer;
    }
    threadgroup uint* shared_inner_tree = &shared_element_tree[size];

    batch_inverse(column, result, size, log_size, shared_element_tree, shared_inner_tree, thread_position_in_threadgroup, threads_per_threadgroup, threadgroup_position_in_grid);
}