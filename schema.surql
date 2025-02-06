DELETE user;

DEFINE TABLE OVERWRITE user SCHEMAFULL
    PERMISSIONS
        FOR SELECT, UPDATE, DELETE WHERE $auth.id == id,
        FOR CREATE NONE;

DEFINE TABLE OVERWRITE cosmetic SCHEMAFULL
    PERMISSIONS
        FOR SELECT FULL;

-- Define a relation table, and constrain the type of relation which can be stored
DEFINE TABLE OVERWRITE needs TYPE RELATION FROM user TO cosmetic
    PERMISSIONS
        FOR SELECT, UPDATE, DELETE, CREATE WHERE in.id = $auth.id;

DEFINE INDEX OVERWRITE unique_needs
    ON TABLE needs
    COLUMNS in, out UNIQUE;

DEFINE TABLE OVERWRITE cosmeticFound SCHEMAFULL
    PERMISSIONS
        FOR SELECT FULL,
        FOR CREATE, DELETE WHERE $auth.id == foundUser.id;

    DEFINE FIELD OVERWRITE foundUser ON cosmeticFound TYPE record<user>;
    DEFINE FIELD OVERWRITE foundCosmetic ON cosmeticFound TYPE record<cosmetic>;
    DEFINE FIELD OVERWRITE time ON cosmeticFound TYPE datetime DEFAULT time::now()
        PERMISSIONS
            FOR UPDATE NONE;

DEFINE ACCESS OVERWRITE account ON DATABASE TYPE RECORD
SIGNIN ({
    let $response = http::get('https://discord.com/api/users/@me', {
        'Authorization': 'Bearer ' + $discordToken,
        'Content-Type': 'application/json'
    });

    -- Check if the HTTP request was successful
    if !$response.id {
        THROW 'Failed to authenticate with Discord';
    };

    -- If the user does not exist, create a new one
    RETURN UPSERT user SET id = $response.id, name = $response.username;
})
DURATION FOR TOKEN 1h, FOR SESSION 5h;
