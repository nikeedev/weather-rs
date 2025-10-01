# weather-rs: Your terminal weather app 

![video.gif][video.gif]

This program is using [METNO](https://en.wikipedia.org/wiki/Norwegian_Meteorological_Institute)'s weather API, that the famous Norwegian weather app uses: YR. 
This means the program will work optimally in Norway and some parts of Scandinavia (coordinates within forementioned areas). Outside forementioned areas, the weather predictions could be incorrect or unstable. Data that couldn't be retrieved by METNO will show as `null` next to the unit of measurement.

This program is made purely for fun, so enjoy!

## Example commands:

Setting custom latitude and longitude:
```sh
weather-rs --lat 59.000000 --long 5.000000
```
---
Setting custom name:
```sh
weather-rs --lat 59.000000 --long 5.000000
```
---
Print out short version of the weather (ideal for shell startup scripts)
```sh
weather-rs --short
```
---
Use predefined place using the name
```sh
weather-rs --place "Galdhøpiggen"
```

## Using the `places.json` file:
When you want to use predefined places, without needing to remember the coordinates of the place, and call them using `--place` use `places.json` file.

The `places.json` file will be searched in the location where the executable resides, so have the `places.json` file in same folder as the `weather-rs` file

Each place needs a name, latitude and longitude.

Example `places.json` file:
```jsonc
{
    "gald": { // the callname (--place "gald")
        "name" : "Galdhøpiggen", // name that will be used when previewing the weather
        "lat": 61.636389, // latitude
        "long": 8.3125 // longitude
    }
}
```



