#ifndef FLOCK_SIGNAL_H
#define FLOCK_SIGNAL_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct fs_engine fs_engine;
typedef struct fs_template fs_template;

typedef struct fs_channel_frame {
    const float* samples;
    size_t sample_count;
    double sample_rate_hz;
    int64_t timestamp_ns;
} fs_channel_frame;

enum {
    FS_OK = 0,
    FS_ERR_NULL = -1,
    FS_ERR_INVALID = -2,
    FS_ERR_INTERNAL = -3
};

int32_t fs_engine_create(fs_engine** out_engine);
void fs_engine_destroy(fs_engine* engine);
int32_t fs_engine_extract_template(
    fs_engine* engine,
    const fs_channel_frame* channels,
    size_t channel_count,
    fs_template** out_template
);
float fs_template_quality(const fs_template* value);
size_t fs_template_feature_count(const fs_template* value);
int32_t fs_template_copy_features(
    const fs_template* value,
    float* out_features,
    size_t capacity
);
int32_t fs_template_copy_source_digest(
    const fs_template* value,
    uint8_t* out_digest,
    size_t capacity
);
void fs_template_destroy(fs_template* value);

#ifdef __cplusplus
}
#endif

#endif
