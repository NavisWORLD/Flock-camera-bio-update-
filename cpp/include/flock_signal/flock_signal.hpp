#ifndef FLOCK_SIGNAL_HPP
#define FLOCK_SIGNAL_HPP

#include "flock_signal.h"

#include <span>
#include <stdexcept>
#include <string>
#include <utility>

namespace flock_signal {

class SignalTemplate {
public:
    explicit SignalTemplate(fs_template* handle) : handle_(handle) {
        if (!handle_) {
            throw std::invalid_argument("SignalTemplate requires a non-null handle");
        }
    }

    ~SignalTemplate() { fs_template_destroy(handle_); }

    SignalTemplate(const SignalTemplate&) = delete;
    SignalTemplate& operator=(const SignalTemplate&) = delete;

    SignalTemplate(SignalTemplate&& other) noexcept : handle_(std::exchange(other.handle_, nullptr)) {}

    SignalTemplate& operator=(SignalTemplate&& other) noexcept {
        if (this != &other) {
            fs_template_destroy(handle_);
            handle_ = std::exchange(other.handle_, nullptr);
        }
        return *this;
    }

    [[nodiscard]] float quality() const noexcept {
        return fs_template_quality(handle_);
    }

private:
    fs_template* handle_{};
};

class Engine {
public:
    Engine() {
        const auto status = fs_engine_create(&handle_);
        if (status != FS_OK || !handle_) {
            throw std::runtime_error("failed to create flock signal engine");
        }
    }

    ~Engine() { fs_engine_destroy(handle_); }

    Engine(const Engine&) = delete;
    Engine& operator=(const Engine&) = delete;
    Engine(Engine&&) = delete;
    Engine& operator=(Engine&&) = delete;

    [[nodiscard]] SignalTemplate extract(std::span<const fs_channel_frame> channels) const {
        fs_template* output = nullptr;
        const auto status = fs_engine_extract_template(
            handle_, channels.data(), channels.size(), &output
        );
        if (status != FS_OK || !output) {
            throw std::runtime_error("signal template extraction failed with status " + std::to_string(status));
        }
        return SignalTemplate(output);
    }

private:
    fs_engine* handle_{};
};

}  // namespace flock_signal

#endif
