#ifndef FLOCK_SIGNAL_HPP
#define FLOCK_SIGNAL_HPP

#include "flock_signal.h"

#include <cstdint>
#include <vector>

namespace flock_signal {

struct Channel {
    std::vector<float> samples;
    double sample_rate_hz{0.0};
    std::int64_t timestamp_ns{0};
};

struct SignalTemplate {
    float quality{0.0F};
    float uncertainty{1.0F};
};

class Engine {
public:
    Engine();
    ~Engine();

    Engine(const Engine&) = delete;
    Engine& operator=(const Engine&) = delete;
    Engine(Engine&& other) noexcept;
    Engine& operator=(Engine&& other) noexcept;

    [[nodiscard]] SignalTemplate extract(const std::vector<Channel>& channels) const;

private:
    fs_engine* engine_{nullptr};
};

}  // namespace flock_signal

#endif
