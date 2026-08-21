-- schema de la base de datos para meowbot / valenwoof (supabase / postgres)

-- tabla de comandos custom
CREATE TABLE IF NOT EXISTS custom_commands (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    response TEXT NOT NULL,
    cooldown_seconds INT NOT NULL DEFAULT 5,
    enabled BOOLEAN NOT NULL DEFAULT true,
    permission_level VARCHAR(20) NOT NULL DEFAULT 'everyone', -- 'everyone', 'mod', 'broadcaster'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- indice para busqueda rapida por nombre de comando
CREATE INDEX IF NOT EXISTS idx_custom_commands_name ON custom_commands(name);
