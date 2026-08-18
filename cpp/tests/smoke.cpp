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
    std::cout << "C++/Rust FFI smoke OK quality=" << result.quality() << '\n';
    return 0;
}
