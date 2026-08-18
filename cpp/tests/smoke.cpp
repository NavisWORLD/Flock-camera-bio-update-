#include <flock_signal/flock_signal.hpp>

#include <array>
#include <iostream>

int main() {
    std::array<float, 4> samples{1.0F, -1.0F, 1.0F, -1.0F};
    fs_channel_frame frame{
        samples.data(),
        samples.size(),
        4.0,
        42,
    };

    flock_signal::Engine engine;
    auto result = engine.extract(std::span<const fs_channel_frame>(&frame, 1));
    if (!(result.quality() > 0.0F && result.quality() <= 1.0F)) {
        std::cerr << "unexpected template quality\n";
        return 1;
    }

    const auto features = result.features();
    if (features.size() != 3 || !(features[0] > 0.0F)) {
        std::cerr << "unexpected feature vector\n";
        return 2;
    }

    const auto digest = result.source_digest();
    bool any_digest_byte = false;
    for (const auto byte : digest) {
        any_digest_byte = any_digest_byte || byte != 0;
    }
    if (!any_digest_byte) {
        std::cerr << "source digest was unexpectedly all zero\n";
        return 3;
    }

    std::cout << "C++/Rust FFI smoke OK quality=" << result.quality()
              << " features=" << features.size() << '\n';
    return 0;
}
