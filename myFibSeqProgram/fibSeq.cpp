#include "cxx-fibSeq/myFibSeqProgram/fibSeq.h"

rust::Vec<::std::int32_t> fibonacci(int n) {
    std::vector<int> sequence(n);
    sequence[0] = 0;
    sequence[1] = 1;
    for (int i = 2; i < n; i++) {
        sequence[i] = sequence[i - 1] + sequence[i - 2];
    }

    rust::Vec<::std::int32_t> rust_sequence;
    for (const auto& value : sequence) {
        rust_sequence.push_back(value);
    }

    return rust_sequence;
}