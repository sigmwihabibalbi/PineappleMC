# Ely.by sessions

Horizon can join Ely.by authenticated online-mode servers with an Ely.by account added in Settings.
Choose **Sign in with Ely.by** to open Ely.by's official browser login and approve Horizon with its
one-time code. Horizon never asks for or receives your Ely.by password. It keeps the access and
refresh tokens in the OS keychain, refreshes the access token when the game starts, and passes it to
the game through a short-lived session file.

For manual session handoff, obtain a current access token and profile UUID from Ely.by and start
Horizon with a fresh session file containing:

```json
{
  "name": "YourElyName",
  "uuid": "undashed-profile-uuid",
  "access_token": "ely.by access token",
  "user_type": "ely",
  "session_join_url": "https://authserver.ely.by/session/join"
}
```

Pass it with `--session-file PATH`. The game consumes and deletes the file at startup; on Unix it
must be readable only by its owner (`chmod 600`). Horizon sends the standard `accessToken`,
`selectedProfile` and `serverId` join request to Ely.by's `/session/join` endpoint. Microsoft
sessions continue to use Mojang's services discovery document.

Ely.by access tokens are credentials. Do not put them in shell history, source control, logs, or
shared files. Ely.by tokens can expire; use its refresh flow and create a new short-lived session
file before starting Horizon again. Ely.by session servers validate online-mode joins, so the
server must also be configured for Ely.by authentication.

References: [Ely.by authentication for Minecraft](https://docs.ely.by/en/minecraft-auth.html),
[Ely.by API](https://docs.ely.by/en/api.html).
