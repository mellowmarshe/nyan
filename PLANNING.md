## Movement System/House Travelling

```
+ A B C D
0 ⚔️
1
2
3 🛏️    🍪
```

Above is an example of what a house may be like but in text. We will have each of these coordinates mapped to a specific waypoint like:

```
A0 = Battles
A3 = Sleep
D3 = Eat/Drink
```

this will be mapped in an enum with each waypoint.

TLDR;

Each "map" or "house" will have a few points mapped to a location on an enum, we will load this with the config and load each location accordingly. The cats table will have another row called "location" which is an enum matching each of these locations. We then use that enum to figure out what they can and cant do.
