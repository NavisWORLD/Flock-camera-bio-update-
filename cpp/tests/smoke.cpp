#include "flock_signal/flock_signal.hpp"

#include <cmath>
#include <iostream>
#include <vector>

int main() {
    try {
        flock_signal::Engine engine;
        flock_signal::Channel channel;
        channel.sample_rate_hz = 8.0;
        channel.timestamp_ns = 0;
        channel.samples = {0.0F, 1.0F, 0.0F, -1.0F, 0.0F, 1.0F, 0.0F, -1.0F};
        const auto result = engine.extract(std::vector<flock_signal::Channel>{channel});
        if (!std::isfinite(result.quality) || result.quality < 0.0F || result.quality > 1.0F) {
            std::cerr << "quality out of range\n";
            return 2;
        }
        if (!std::isfinite(result.uncertainty) || result.uncertainty < 0.0F || result.uncertainty > 1.0F) {
            std::cerr << "uncertainty out of range\n";
            return 3;
        }
        return 0;
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
