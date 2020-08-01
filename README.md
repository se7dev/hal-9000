![hal-9000](https://live.staticflickr.com/1974/43234531050_c7a7f3ff1f.jpg)

["HAL 9000"](https://www.flickr.com/photos/tomronworldwide/43234531050) by [Ron Frazier](https://www.flickr.com/photos/tomronworldwide/) is licensed under [CC BY 2.0](https://creativecommons.org/licenses/by/2.0/#)
# Description: 
This is a chatbot for the streaming platfrom Twitch.tv written in Rust and realized through Docker.
It offers several features for your chat, including:
- Voting for multiple options directly in the chat
- Organizing giveaways
- Logging user messages sent in the chat
- Filtering swear words in mutliple Languages (Currently German and English)

# Installation: 
- Create a new account on Twitch.tv for your chatbot
- Generate a oauth token for the chatbot account under [this link.](https://twitchapps.com/tmi/)
- Simply make a local copy of this repository by ``` git clone ``` either via http oder ssh
- Rename **.env-sampe** to **.env** and replace <> it with your information:
```
MONGO_INITDB_ROOT_USERNAME=<database_root_username>
MONGO_INITDB_ROOT_PASSWORD=<database_root_password>
MONGO_INITDB_ROOT_DATABASE=<database_root_database_name>
RUST_BACKTRACE=1
TWITCHPW=<oauth_token_of_twitch_bot_account>
NICKNAME=<name_of_twitchbot_account>
SERVERNAME=irc.chat.twitch.tv
SERVERPORT=6667
CHANNELS=<channels_chatbot_should_join>
LANG=<language_for_swear_word_filter>
```
A few notes on some of the variables:

- **CHANNELS:** Should be in the form of #username
- **LANG:** Can be Deutsch, German, English or Englisch. If anything else is put in here, it defaults to English.

# Usage: 
- ```cd``` into the directory of the checked out repo
- use ```cargo run``` to start the chatbot
- In your chat, you can now send commands to your chatbot

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

See [GPT3-License](https://choosealicense.com/licenses/gpl-3.0/) for details.

# Self criticism
- Sadly not all our set goals were reachable (Point system)
- This was due to not being able to extract users from sent messages
- With the other set goals we are quite happy, although some of the functions could still use refactoring
- Getting used to the async concept (for accessing our MongoDB) in Rust was a bit tricky at the beginning, 
but it was good to dive into it nonetheless
