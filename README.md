# Highscore-API

## What is Highscore API?

The Highscore API provides essential functions for our organization, including creating a game developer, creating a game for a specific developer, and managing high scores for games by a specific developer.

## Project Structure

### Scripts

**create-gamedev**

The script `create-gamedev` is responsible for creating a game developer that does not exist yet. If the developer already exists, the script will raise a `DuplicateEntryError`. See the examples below:

Remember to change <gamedev_name> to the GameDev's real name.
`$ cargo run -- create-gamedev <gamedev_name>`

If the developer already exists:

`Error to create a new GameDev: DuplicateEntryError`

If the developer does not exist, the script returns a `GAMEDEV_ID` and an `API_KEY`. Example response:

`GAMEDEV_ID: <gamedev_id>, API_KEY: <api_key>`

**refresh_api_key**

If, for any reason, you need a new `API_KEY` for an already registered developer, you can use this script to generate a new key. Examples:

Remember to change <gamedev_id> to the GameDev's real id.
`$ cargo run -- refresh-api-key <gamedev_id>`

If there is no developer with the specified ID:

`Error to refresh gamedev api_key: NotFound`

If the developer with the specified ID exists, only the `API_KEY` will be changed:

`Successfully refreshed the gamedev api_key: <new_api_key>`

### Handlers

The Highscores API includes useful handlers for daily needs. Below, we list and provide examples for each of them.

#### Utils

**health_handler**

This handler is used to check the health of the API. Make a GET request to the URL/health. If the API is functioning correctly, a status 200 and a JSON will be returned, as shown below:

```json
{
  "status": "pass"
}
```

**full_health_handler**

This handler is used to check the health of both the API and the database. Make a GET request to URL/health/full. Depending on the situation, it will return a status 200 or an error, accompanied by a JSON. Examples:

If everything is OK:

```json
{
  "status": "pass",
  "uptime": 60,
  "db": true
}
```

In the case of a database error:

```json
{
  "status": "fail",
  "uptime": 60,
  "db": false
}
```

#### Game

**create_game_handler**

The `create_game_handler` is used to create a new game for a specific developer, provided there is no other game with the same name. To do this, send a POST request to the URL/games, where the header must contain a Bearer Token with a valid `API_KEY`, and the body must be a JSON, as shown below:

Request body:

Remember to change <game_name> to the Game real name.

```json
{
  "name": "<game_name>"
}
```

Successful creation response:

```json
{
  "id": "<game_id>",
  "name": "<game_name>"
}
```

**get_games_handler**

This handler is responsible for returning all games created for a developer. Send a GET request to URL/games, where the header must contain a Bearer Token with a valid `API_KEY`. In case of success, the handler will return a JSON with an array of games, as shown below:

Success response:

```json
{
  "games": [
    {
      "id": "<game_id>",
      "name": "<game_name>"
    },
    {
      "id": "<game_id>",
      "name": "<game_name>"
    },
    {
      "id": "<game_id>",
      "name": "<game_name>"
    },
    {...}
  ]
}
```

**get_game_handler**

Finally, the `get_game_handler` is responsible for returning a specific game for a developer. Make a GET request to URL/game/{game_id}, and the handler will respond with a status 200 and a JSON containing the found game, or a 404 if no games are created with the specified ID. See the success example below:

```json
{
  "id": "<game_id>",
  "name": "<game_name>"
}
```

### Storage

#### GameDev

**create_gamedev**

The `create_gamedev` function is responsible for creating the game developer. It receives a single parameter named `NAME` of type String. It uses the `get_gamedev_by_name` function to search the database for a game developer with the same name passed in the `Name` parameter. If it finds a developer, it returns a `AppError::DuplicateEntryError`. Otherwise, it creates the developer with the fields `ID: Uuid`, `NAME: String`, `API_KEY: Uuid`, and calls the `get_gamedev_by_id` function to return a `GameDev`.

**get_gamedev_by_id**

The `get_gamedev_by_id` function uses a filter by `ID` of type `Uuid` to search the database for a `GameDev`. If found, the function returns the result. If no `GameDev` with the same `ID` is found, it returns an `AppError::NotFound`.

**get_gamedev_by_name**

The `get_gamedev_by_name` function uses a filter by `NAME` of type `String` to search the database for a `GameDev`. If found, the function returns the result. If no `GameDev` with the same `NAME` is found, it returns an `AppError::NotFound`.

**get_gamedev_by_api_key**

The `get_gamedev_by_api_key` function uses a filter by `API_KEY` of type `Uuid` to search the database for a `GameDev`. If found, the function returns the result. If no `GameDev` with the same `API_KEY` is found, it returns an `AppError::NotFound`.

#### Games

**create_game**

With this function, you can create a new game for a game developer. It receives a `NewGame` object with the fields `NAME: String` and `GAMEDEV_ID: Uuid`, calls the `get_gamedev_by_id` function. If there is no `GameDev` created with the same `ID`, it returns an `AppError::NotFound`. If it finds a `GameDev`, it moves on to the next step, validating the existence of a game with the same `NAME`. If it exists, it returns an `AppError:DuplicateEntryError`. If it does not exist, it instantiates a `Game` object with the fields `ID: Uuid`, `Name: String`, `GAMEDEV_ID: Uuid`, creates a new game in the game collection, and finally uses the `get_game_by_name` function to return the game directly from the database.

**get_game_by_name**

The `get_game_by_name` function uses a filter by `NAME: String` and `GAMEDEV_ID: Uuid` to search the game collection. If no game or game developer is found, it returns an `AppError::NotFound`. If it finds a game, it returns it.

**get_game_by_id**

The `get_game_by_id` function uses a filter by `ID: Uuid` and `GAMEDEV_ID: Uuid` to search the game collection. If no game or game developer is found, it returns an `AppError::NotFound`. If it finds a game, it returns it.

**get_games**

The `get_games` function uses a filter by `GAMEDEV_ID: Uuid` to search the game collection. If no game or game developer is found, it returns an `AppError::NotFound`. If it finds a game developer, it returns an array of games with all the created games.

#### Highscores

**update_highscore**

`Update_highscore` is a function that receives a `NewHighscore` object: `GAME_ID: Uuid`, `GAMEDEV_ID: Uuid`, `SCORE: u32`, and `USERNAME: String`. It begins by checking the existence of a game using the `get_game_by_id` function. If found, it proceeds with the function flow; if not found, it interrupts the flow and reports an `AppError::NotFound`. The next step is creating a `Highscore` object with the fields `GAME_ID: Uuid`, `ID: Uuid`, `SCORE: u32`, `UPDATED_AT: DateTime`, `USERNAME: String`. After creating the object, it checks for the existence of a `current_highscore`. In the absence of a `current_highscore`, it creates a `Highscore` in the database and calls the `get_highscore` function, passing the `new_highscore.game_id` and `new_highscore.username` to fetch the value directly from the database. If there is a `Highscore`, it checks if the value of `new_highscore.score` is greater than the `current_highscore.score` in the database. If the check is negative, the flow ends without changing the value, but it calls the `get_highscore` function, passing the `new_highscore.game_id` and `new_highscore.username` to fetch the value directly from the database. If it is positive, the function uses a filter by `GAME_ID: Uuid` and `USERNAME: String` to update the value in the database with `new_highscore.score`, and finally, it calls the `get_highscore` function, passing the `new_highscore.game_id` and `new_highscore.username` to fetch the value directly from the database.

**get_highscore**

`Get_highscore` is the function used to retrieve a high score in the Highscore collection. It receives two parameters, `GAME_ID: Uuid` and `USERNAME: String`. With these parameters, it creates a filter by `GAME_ID: Uuid` and `USERNAME: String`. If there is a record in the database, it returns it.

**get_highscores**

To return all Highscores for a Game of a specific GameDev, we use this function. It receives two parameters, `GAME_ID: Uuid` and `GAMEDEV_ID: Uuid`. It uses the `get_game_by_id` function; if there is a game, it continues the flow. The next step is defining two filters, the first by `GAME_ID: Uuid` and the second by `SCORE: u32` listing from the highest to the lowest and by the last update. Each value retrieved from the database is saved in an array of the cursor type `Cursor<Highscore,>` which is later transformed into a vector `highscores` of type `Highscore`, and finally, it returns the vector.

## How to Run This Project

1.  To run this application, clone the repository [Highscore-API](https://github.com/andref-dev/highscore-api).
2.  Install Rust by visiting the official [Rust](https://www.rust-lang.org/tools/install) website and following the steps. 2.1. Linux may require the installation of GCC to run Actix correctly. Execute the command `$ sudo apt install build-essential`.
3.  Create a _.env_ file and configure the environment variables. You can use the values from _.env.example_.
4.  Run the command `$ cargo build` to install the project's binaries and libraries.
5.  Now run the command `$ cargo run`, and the application will execute successfully. Enjoy!

## Environment Variables

In this section, we will detail the environment variables and their roles in our API. These are the necessary environment variables to run the application successfully, and you can see example values in the _.env.example_ file. They are as follows:

- `RELEASE_MODE`: This variable is responsible for defining the execution level of the application, either in development (`dev`) or production (`prod`). Choose the option that best suits your scenario. Opting for `dev` sets the lowest log level to `Debug`, while in `prod`, the lowest log level is set to `Info`.
- `API_PORT`: Specify the port on which the API will receive requests. Feel free to choose according to the most convenient scenario. By default, we set the value to 4000.
- `MONGO_URI`: This variable should point to your preferred MongoDB database, whether local or in the cloud. Remember that this field is of type `String`.
- `TIMEOUT`: Finally, the variable responsible for determining the timeout for requests. By default, this value is 75, but it can be changed according to your organization. This field is of type `Number`.
