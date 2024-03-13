# EchoTree
Topic based pub-sub distributed database with automated real-time syncing.

## Concept
- Tree based data storage e.g:
  - `/general/teams`
  - `/scoring/scores`
  - `/users`
- Endpoints are bin trees with JSON string structures
  - `/general/teams` -> `HahsMap<String, String>`
  - `/scoring/scores` -> `HahsMap<String, String>`
  - `/users` -> `HahsMap<String, String>`
- Automated accessible topology `/metadata/hierarchy`
  - maps each endpoint using the branch name. e.g:
    - `/general/teams` as the key and the [JSON Schema](https://json-schema.org/) as value
- Automated Pub Sub messaging for db updates e.g:
  - `POST http://example.com/subscribe/general/teams`:
    - Updated when `/`, `/general` and `/general/teams` are changed
  - `POST http://example.com/subscribe`
    - Updated for every change in the database
- Bi-directional websocket communications
  - Constant connection with update messages: 
    - `{"type": "insert", "tree": "/general/teams/41", "json": "{}"}`
- Distributed data sets (replicated data set on client side based on subscription)
  - A copy of the subscribed data is put in storage client side (persistent)
  - Data stored and accessed in memory cache before syncing with local copy.
  - Three step client side storage process. `Memory -> Storage -> Server Pub Sync`
  - Three step server side storage process. `Memory -> Storage -> Client Sub Sync`
  - Automated data syncing from local and server side data
- User/Role based encryption based on tree e.g:
  - admin -> [`/`]
  - judge -> [`/general/teams`, `/scoring/scores`]
  - etc...