CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE camera_events (
    event_id UUID PRIMARY KEY,
    camera_id TEXT NOT NULL,
    observed_at_ns NUMERIC(30,0) NOT NULL,
    zone_id TEXT,
    event_kind TEXT NOT NULL,
    attributes JSONB NOT NULL DEFAULT '{}'::jsonb,
    source_uri TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE sensor_observations (
    observation_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    window_id UUID NOT NULL UNIQUE,
    start_ns NUMERIC(30,0) NOT NULL,
    end_ns NUMERIC(30,0) NOT NULL,
    sensor_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    raw_retained BOOLEAN NOT NULL DEFAULT FALSE,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE signal_templates (
    template_id UUID PRIMARY KEY,
    observation_id UUID REFERENCES sensor_observations(observation_id) ON DELETE CASCADE,
    feature_schema TEXT NOT NULL,
    feature_version TEXT NOT NULL,
    features JSONB NOT NULL,
    quality REAL NOT NULL CHECK (quality >= 0 AND quality <= 1),
    uncertainty REAL NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 1),
    source_digest BYTEA NOT NULL,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE safety_events (
    safety_event_id UUID PRIMARY KEY,
    camera_event_id UUID REFERENCES camera_events(event_id) ON DELETE SET NULL,
    template_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    severity SMALLINT NOT NULL CHECK (severity >= 0 AND severity <= 100),
    requires_human_review BOOLEAN NOT NULL DEFAULT TRUE,
    reasons JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE ledger_records (
    record_id UUID PRIMARY KEY,
    observed_at_ns NUMERIC(30,0) NOT NULL,
    event_id UUID NOT NULL,
    payload_digest BYTEA NOT NULL,
    previous_record_digest BYTEA NOT NULL,
    record_digest BYTEA NOT NULL UNIQUE,
    signer_key_id TEXT NOT NULL,
    signature BYTEA NOT NULL,
    policy_context JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE audit_events (
    audit_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    request_id UUID,
    actor_id TEXT NOT NULL,
    action TEXT NOT NULL,
    object_type TEXT NOT NULL,
    object_id TEXT,
    authorization_ref TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE retention_jobs (
    job_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    object_type TEXT NOT NULL,
    object_id TEXT NOT NULL,
    scheduled_for TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    status TEXT NOT NULL DEFAULT 'pending'
);

CREATE TABLE policy_decisions (
    decision_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    request_id UUID,
    action TEXT NOT NULL,
    allowed BOOLEAN NOT NULL,
    elevated BOOLEAN NOT NULL,
    requires_human_review BOOLEAN NOT NULL,
    reasons JSONB NOT NULL,
    authorization_ref TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX camera_events_time_idx ON camera_events (observed_at_ns);
CREATE INDEX camera_events_zone_idx ON camera_events (zone_id);
CREATE INDEX safety_events_created_idx ON safety_events (created_at);
CREATE INDEX audit_events_created_idx ON audit_events (created_at);
CREATE INDEX retention_jobs_pending_idx ON retention_jobs (status, scheduled_for);

COMMENT ON TABLE signal_templates IS 'Anonymous derived signal representations. Production schema intentionally contains no civilian identity mapping.';
