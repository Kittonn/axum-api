IF NOT EXISTS (
    SELECT * FROM sys.tables WHERE name = 'users'
)
BEGIN
    CREATE TABLE users (
        id UNIQUEIDENTIFIER NOT NULL
            CONSTRAINT pk_users PRIMARY KEY
            DEFAULT NEWID(),

        email NVARCHAR(255) NOT NULL,
        password NVARCHAR(255) NOT NULL,
        name NVARCHAR(255) NOT NULL,

        created_at DATETIME2 NOT NULL DEFAULT GETDATE(),
        updated_at DATETIME2 NOT NULL DEFAULT GETDATE()
    );

    CREATE UNIQUE INDEX idx_users_email_unique
        ON users (email);
END
GO
