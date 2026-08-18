#include "flock_signal/flock_signal.hpp"

#include <stdexcept>
#include <utility>

namespace flock_signal {
namespace {

void require_ok(int code, const char* operation) {
    if (code != FS_OK) {
        throw std::runtime_error(std::string(operation) + " failed with status " + std::to_string(code));
    }
}

}  // namespace

Engine::Engine() {
    require_ok(fs_engine_create(&engine_), "fs_engine_create");
}

Engine::~Engine() {
    fs_engine_destroy(engine_);
    engine_ = nullptr;
}

Engine::Engine(Engine&& other) noexcept : engine_(std::exchange(other.engine_, nullptr)) {}

Engine& Engine::operator=(Engine&& other) noexcept {
    if (this != &other) {
        fs_engine_destroy(engine_);
        engine_ = std::exchange(other.engine_, nullptr);
    }
    return *this;
}

SignalTemplate Engine::extract(const std::vector<Channel>& channels) const {
    if (engine_ == nullptr) {
        throw std::runtime_error("Engine has no native handle");
    }
    if (channels.empty()) {
        throw std::invalid_argument("at least one channel is required");
    }

    std::vector<fs_channel_frame> native;
    native.reserve(channels.size());
    for (const auto& channel : channels) {
        native.push_back(fs_channel_frame{
            channel.samples.data(),
            channel.samples.size(),
            channel.sample_rate_hz,
            channel.timestamp_ns,
        });
    }

    fs_template* result = nullptr;
    require_ok(
        fs_engine_extract_template(engine_, native.data(), native.size(), &result),
        "fs_engine_extract_template"
    );

    float quality = 0.0F;
    float uncertainty = 1.0F;
    const int quality_status = fs_template_quality(result, &quality);
    const int uncertainty_status = fs_template_uncertainty(result, &uncertainty);
    fs_template_destroy(result);
    require_ok(quality_status, "fs_template_quality");
    require_ok(uncertainty_status, "fs_template_uncertainty");

    return SignalTemplate{quality, uncertainty};
}

}  // namespace flock_signal
