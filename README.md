![hal-9000](https://live.staticflickr.com/1974/43234531050_c7a7f3ff1f.jpg)

["HAL 9000"](https://www.flickr.com/photos/tomronworldwide/43234531050) by [Ron Frazier](https://www.flickr.com/photos/tomronworldwide/) is licensed under [CC BY 2.0](https://creativecommons.org/licenses/by/2.0/#)
# Description: 
This is a chatbot for the streaming platfrom Twitch.tv written in Rust and realized through Docker.
It offers several features for your chat, including:
- Voting for multiple options directly in the chat
- Organizing giveaways
- Logging user messages sent in the chat
- Filtering swear words in mutliple languages (currently German and English)

# Installation: 
- Create a new account on Twitch.tv for your chatbot
- Generate an oauth token for the chatbot account under [this link.](https://twitchapps.com/tmi/)
- Simply make a local copy of this repository by ``` git clone ```
- Rename **.env-sample** to **.env** and replace ```<_>``` with your information:
```
MONGO_INITDB_ROOT_USERNAME=<database_root_username>
MONGO_INITDB_ROOT_PASSWORD=<database_root_password>
MONGO_INITDB_DATABASE=<database_root_database_name>
MONGO_INITDB_HOSTNAME=<Hostname-_of_mongoDb_host_as_seen_by_app>
TWITCHPW=<oauth_token_of_twitch_bot_account>
NICKNAME=<name_of_twitchbot_account>
SERVERNAME=irc.chat.twitch.tv
CHANNELS=<channels_chatbot_should_join>
LANG=<language_for_swear_word_filter>
```
A few notes on some of the variables:

- **CHANNELS:** Should be in the form of #username
- **LANG:** Can be Deutsch, German, English or Englisch. If anything else is put in here, filter will default to English.

# Usage: 
- ```cd``` into the directory of the checked out repo
- Use ```cargo run``` to start the chatbot
- In your chat, you can now see the chatbot as a new user
- You can now use the chatbot commands below from your chat

## List of chatbot commands:
- **!ping**: Chatbot should respond with "!pong"
- **!vote <option\>**: Users can vote for an option by sending this command, where <option\> should be one of the offered options by the chatbot
- **!votestart <option_n\>**: Users / Mods can initiate a vote on multiple options, each separated by a whitespace
- **!voteend**: Users / Mods can end a vote and the chatbot sends the result of the vote
- **!giveawaystart**: Users / Mods can start a giveaway
- **!giveawayend**: Users / Mods can end a giveaway and a winner is chosen
- **!giveawayenter <username\>**: Users can enter a giveaway by putting in their username


# Credits: 

[se7dev](https://github.com/se7dev)

[RoWe90](https://github.com/RoWe90)

# License: 
hal9000 is distributed under the terms of the GPT2 License.

See [GPT2-License](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html) for details.

# Self criticism
- Sadly not all our set goals were reachable (Point system)
- This was due to not being able to extract users from sent messages
- With the other set goals we are quite happy, although some of the functions could still use refactoring
- Test coverage could be better for some functions
- Testing the database and Twitch IRC with Travis was tricky, so we resorted to testing it only locally (See PR #34)
