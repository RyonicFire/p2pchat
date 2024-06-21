# RustP2PChat

A simple command based peer to peer chat written in rust

# Commands
| Command   | Description                                                                   |
|-----------| ------------------------------------------------------------------------------|
| /help                                     | Print help message                            |
| /connect <IP:PORT> <CHAT_NAME>            | Create new chat                               |
| /msg <CHAT_NAME> <MSG>                    | Send message to chat                          |
| /create-group <GROUP_NAME> <CHAT_NAME...> | Create a group with the specified chats       |
| /msg-group <GROUP_NAME> <MSG>             | Send message to group                         |
| /broadcast <MSG>                          | Send message to all chats                     |
