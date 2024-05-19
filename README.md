# rusty-poker

## API v1
All endpoints should be prepended with `v1/`, i.e. `v1/create_game`

## Endpoints
- POST /create_game  - creates new game
    Request body parameters:
    ```js
    {
        "seats_count": int,
        "small_blind": int,
        "big_blind": int,
        "initial_balance": int,
        "bet_time": int
    }
    ``` 
    Responses:
    - (201) `{"message": "success", "game_id": int}`
    - (400) `{"error": string}`
    - (500) `{"error": string}`
<br>

- GET /games - gets all games <br>
    Responses:
    - (200) `{"message": "success", "games": Game[]}`
    - (500) `{"error": string}`
<br>

- POST /join_game - creates new game
    Request body parameters:
    ```js
    {
        "game_id": uuid,
        "player_name": string,
        "chosen_seat": int
    }
    ``` 
    Responses:
    - (200) `{"message": "success"}`
    - (400) `{"error": string}`
    - (409) `{"error": string}`
    - (500) `{"error": string}`
<br>

- POST /set_ready - set ready state
  Request body parameters:
    ```js
    {
        "game_id": uuid,
        "new_ready_state": boolean
    }
    ``` 
    Responses:
    - (200) `{"message": "success"}`
    - (400) `{"error": string}`
    - (500) `{"error": string}`
<br>

- GET /game_state - get the state of a given game, response differs if sender cookie authentisizes one of the players
  Query parameters:
  ```
  ?game_id=uuid
  ```
    Responses:
    - (200) `{"message": "success", game_state: GameState}`
    - (400) `{"error": string}`
    - (500) `{"error": string}`
<br>

- GET /listen_changes - long polling for other players actions, response differs if sender cookie authentisizes one of the players
  Query parameters:
  ```
  ?game_id=uuid
  ```
    Responses:
    - (200) `{"message": "updated", game_state: GameState}`
    - (200) `{"message": "nothing changed"}`
    - (400) `{"error": string}`
    - (500) `{"error": string}`
<br>

- POST /perform_action - used for placing bets, checking, calling etc.
  Request body parameters:
    ```js
    {
        "game_id": uuid,
        // is required for actions "raise" or "call"
        "bet"?: int,
        "action": "raise" | "call" | "check" | "fold"
    }
    ```
    Responses:
    - (200) `{"message": "success"}`
    - (400) `{"error": string}`
    - (402) `{"error": "bet is too low"}`
    - (500) `{"error": string}`
<br>

- POST /quit_game - used when you don't want to play anymore
  Request body parameters:
    ```js
    {
        "game_id": uuid
    }
    ``` 
    Responses:
    - (200) `{"message": "success"}`
    - (400) `{"error": string}`
    - (500) `{"error": string}`

## Types
Game:
```js
{
    "seats_count": int,
    "seats_occupied": int,
    "small_blind": int,
    "big_blind": int,
    "initial_balance": int,
    "bet_time": int
}
```

GameState:
```js
{
    "community_cards": Card[],
    "personal_cards": Card[],
    "bets_placed": Bet[],
    "pot": int,
    "players": Player[],
    "small_blind": int,
    "big_blind": int,
    "dealer": int
}
```

Card:
```js
{
    "suit": Enum::Color,
    "rank": Enum::Rank
}
```

Player:
```js
{
    "seat_index": int,
    "balance": int,
    "state": Enum::PlayerState,
    "bet_amount": int,
    "nickname": string
}
```

## Enums
```js
PlayerState[
    "ready",
    "not_ready",
    "active",
    "all-ined",
    "folded",
    "left"
]
```

```js
Color[
    "heart",
    "diamond",
    "spade",
    "club"
]
```

```js
Rank[
    "2",
    "3",
    "4",
    ...
    "Jack",
    "Queen",
    "King",
    "Ace"
]
```
