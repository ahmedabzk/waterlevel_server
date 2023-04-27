-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        first_name VARCHAR(255) NOT NULL,
        last_name VARCHAR(255) NOT NULL,
        email VARCHAR(100) NOT NULL UNIQUE,
        password VARCHAR(100) NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );

CREATE TABLE
    IF NOT EXISTS stats (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        user_id UUID REFERENCES users(id) ON DELETE CASCADE,
        chlorine_level FLOAT NOT NULL,
        ph FLOAT NOT NULL UNIQUE,
        turbidity FLOAT NOT NULL,
        water_level FLOAT NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );

    CREATE TABLE
    IF NOT EXISTS pumps (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        user_id UUID REFERENCES users(id) ON DELETE CASCADE,
        water_status BOOLEAN NOT NULL,
        treatment_status BOOLEAN NOT NULL UNIQUE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );