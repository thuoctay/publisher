-- Create the transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    payload JSONB NOT NULL,
    status VARCHAR(20) DEFAULT 'pending'
);

-- Create the processed_jobs table for tracking processed transactions
CREATE TABLE IF NOT EXISTS processed_jobs (
    record_id BIGINT PRIMARY KEY,
    tx_hash TEXT,
    status TEXT CHECK (status IN ('pending', 'sent', 'confirmed', 'failed')),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insert some dummy data with explicit UTC timestamps
INSERT INTO transactions (created_at, payload, status) VALUES
    ((CURRENT_TIMESTAMP AT TIME ZONE 'UTC' - INTERVAL '5 minutes'), '{"amount": "100", "from": "0x123", "to": "0x456"}'::jsonb, 'pending'),
    ((CURRENT_TIMESTAMP AT TIME ZONE 'UTC' - INTERVAL '4 minutes'), '{"amount": "200", "from": "0x789", "to": "0xabc"}'::jsonb, 'pending'),
    ((CURRENT_TIMESTAMP AT TIME ZONE 'UTC' - INTERVAL '3 minutes'), '{"amount": "300", "from": "0xdef", "to": "0xghi"}'::jsonb, 'pending'),
    ((CURRENT_TIMESTAMP AT TIME ZONE 'UTC' - INTERVAL '2 minutes'), '{"amount": "400", "from": "0xjkl", "to": "0xmno"}'::jsonb, 'pending'),
    ((CURRENT_TIMESTAMP AT TIME ZONE 'UTC' - INTERVAL '1 minute'), '{"amount": "500", "from": "0xpqr", "to": "0xstu"}'::jsonb, 'pending'); 