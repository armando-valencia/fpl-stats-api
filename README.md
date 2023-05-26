# fpl-stats-api
An API to retrieve various player stats for the online fantasy soccer (football) game, [Fantasy Premier League](https://fantasy.premierleague.com/). Created with [Rust](https://www.rust-lang.org/) and using [Rocket](https://rocket.rs/) as the web framework. 

## Testing
To test out the API, you can use a tool like [ngrok](https://ngrok.com/download).

- Run `cargo run` to start the service
- In your terminal, run `ngrok http 8000`
  -  If this doesn't work, you might be using a different port. Check the "Rocket has launched from ..." message in your terminal after running `cargo run` to find the correct port number.
- A new terminal tab should open up with all the information about your new ngrok connection.
- Look for the `Forwarding` row
- The first link provided (it will look something like `https://....ngrok.io`) is the address that you can use to test the API
- To actually test it out, you can use the temporary connection address and add one of the endpoints to pull data from
  -  i.e. This will look something like `https://....ngrok.io/transfers/in`
