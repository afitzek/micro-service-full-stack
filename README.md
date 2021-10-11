# Micro frontends example

Start both services and check the front end page.

The "weather service" can be thought of a micro service, that provides its
own UI elements:

`my-weather-widget`

This element is used in the frontend page and loaded from the api server on the fly.

## Service Weather

Run the weather service:
```
cd service-weather
# Build Frontend Webcomponent
cd ui/weather-service-ui
npm i
npm run build


# Build Backend server hosting the webcomponent
cargo run
```

## Start FrontPage

```
cd frontend-page
npm i
npm run dev
```
