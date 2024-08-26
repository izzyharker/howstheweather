# Weatherbot with Serenity + Shuttle

Discord bot created in Rust to report the current weather in a location.

_usage_
```
/weather [city]
```
Checks database of cities and returns a weather update on match. Database for cities can be found at `https://github.com/plotly/datasets/blob/master/us-cities-top-1k.csv` and should be added to the repo under the file path `src/data/cities.csv`.

## Future goals
- Expand city selection from limited database to an API call
- Update output formatting to add forecase and graphics styling
