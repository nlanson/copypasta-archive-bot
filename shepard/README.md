# Bot Module
This module is in charge of running the reddit bot.

### Usage requirements
To use this module, a `.env` file must be created with environment variables.
| Variable | Meaning |
|--|--|
| bot_username | The user name of the bot account to be controlled by the module |
| bot_password | The password of the bot account |
| bot_userAgent | The user agent of the bot account. This can be anything. |
| bot_clientId | The client ID of the bot account. This can be created by registering a reddit bot under the account |
| bot_clientSecret | The client secret of the bot account. Created by registering a reddit bot. |
| auth_key | The authentication key to access the backend database. This should be the same as the auth_key variable in the backend `.env` file |
| db_host | (Optional) The host location of the backend database. Will default to localhost if not set. |