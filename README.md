# Highscore-API

### What is Highscore API?

The Highscore API is an API that provides essential methods for our game developers, such as creating a new game and recording highscores.

## Project structure

### Handlers

#### Utils

**health_handler**

This handler is used to test the API health. To try this, simply send a GET request to URL/health, and the health handler will respond with a status 200 and a JSON, like this:

```json
{
  "status": "pass"
}
```

**full_health_handler**

This handler is used to test the API and Database health. To try this, simply send a GET request to URL/health/full, and the health handler will respond with a status 200 and a JSON, like this:

```json
{
  "status": "pass",
  "db": true
}
```

---

#### Games

**create_game_handler**

With this handler, we can easily create a new game for a specific game developer. To do this, it is necessary to send a POST request with the Bearer Token header, along with a JSON-type body to URL/games. The Bearer Token must contain an API_KEY, and the body should include a 'name' field, for example:

```json
{
  "name": "<your_game_name>"
}
```

The handler will responde with a status 200 and JSON, for example:

```json
{
  "id": "<your_game_id>",
  "name": "<your_game_name>"
}
```

**game_game**

This handler is responsible for returning a game to a specific game developer. To achieve this, we need to make a GET request to the URL/games/{game_id} route, and the header must include a Bearer Token with a valid API_KEY. See the response example below:

```json
{
  "id": "<your_game_id>",
  "name": "<your_game_name>"
}
```

**get_games**

The get_games handler is responsible for returning all games created by a game developer. We can make a GET request to the URL/games, passing a Bearer Token with a valid API_KEY in the header. See the response example below:

```json
{
	"games": [
		{
			"id": "<your_game_id>",
			"name": "<your_game_name>"
		},
		{
			"id": "<your_game_id>",
			"name": "<your_game_name>"
		},
		{
			"id": "<your_game_id>",
			"name": "<your_game_name>"
		},
		{...}
	]
}
```

---

#### Highscore

**update_highscore**

Update_highscore is responsible for updating players' scores. If there is no recorded score for the player, this handler will create a score. If the existing score is lower, it will update the value with the highest score and respond with a status 200. To achieve this, we need to make a POST request to URL/games/{game_id}/highscores, passing a Bearer Token with a valid API_KEY in the header, and the body must contain a JSON. See the example below:

```json
{
  "score": 90,
  "username": "<username>"
}
```

**get_highscore**

```json
{
  "highscores": [
    {
      "score": 95,
      "username": "<username>"
    },
    {
      "score": 90,
      "username": "<username>"
    }
  ]
}
```

### Storage

[Description on the storage directory]

### Scripts

[Description on the scripts directory]

## How to run this project

[Description on how to run this project locally]

## Env Vars

[Description on each of the ENV VARS needed to run the project]
