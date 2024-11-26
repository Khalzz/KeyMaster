# Arrownier

https://github.com/user-attachments/assets/62281e6b-c52a-4fc5-a6fe-8dc8cde80f46

Arrowner is a “4 key rhythm game” made from scratch using SDL2, the main focus of the game is letting the player create, share and play songs.

The way it works its that each time you open the song list, the game will look inside the folder `songs` and if you want to add songs, you should set here the folders for them.

Each song contains 3 files, if you do your own song, this names should not change:

- **audio.mp3:** The song that will be played on the level.
- **cover.jpg:** The image that will be shown on the level selector.
- **data.json:** The needed data for the game, for things like time for each note, time holding the note, sync values, and when it ends.

The data.json file looks like this:

```rust
{
	"name":"Test",
	"left_keys":[],
	"up_keys":[],
	"bottom_keys":[],
	"right_keys":[],
	"end":133,	  
	"sync": 0,
  "bpm": null
}
```

The important values to take account on is that the keys and end values will be setted from the game itself while the value of bpm can be setted from here, and the existence of it will set “beat lines” in the level itself.

------

## How to play

To move inside the game menu, you use the keys you use to play, by default these being:

- **D:** back.
- **F:** up.
- **J:** down.
- **K:** select.

This values can change depending on how your controls are setted, if for example you go to `settings > controller` you can directly change what buttons to use for playing and the way you move on the menu will also change, so you can play without having to take your hands off the buttons you use to play.

Speaking about settings here you could find that this window activates mouse input, and the main elements you can find here are:

![Opciones](https://i.imgur.com/NzfpxjN.png)

1. **Controller:** You can set the buttons you use to play
2. **Calibration:** You reset the time a note takes to spawn and touch the place where the note selector is in the screen.
3. **Manual calibration:** You can manually set the speed of the notes to adjust desynchronization problems.
4. **Audio:** Change main game volume.
5. **Visualization settings:** You set what you want as a visualizer in game, each one of them just adds to the end audio visualizer, is not that you can select one or other, you can have booth at the same time if you want.

If you instead go to the play button you'll find a list of songs directly from the songs folder, by just selecting one you could directly start playing it.

The game has a general synchronization but each game has a sync value too, so for example if the creator has sync problems you can tune it to be the best version of it by pressing space on the play menu.

------

## Making songs

To make songs you have different ways of approaching this, but the best one is by playing your song on the way you want.

For this you can directly copy a song already existing in the `songs` folder, you add the song you want and change the name of it to the respective `audio.mp3` , the same you can do with the cover that needs to have that name and remember that the format can be either `svg` or `png` .

Then as a personal recommendation go to the `data.json` folder and the value of `end` set it to a high value (for example 100000) and then you can open it in game.

When opening in game you have to ignore the notes that will appear and start playing the game as you like, once you play it on the way you loved just press the `S` key, this will save a `data.json` file in the root folder of the game, replace the song json with this one and the next time you play it, the notes you setted would be there.

------

## Song editor

The editor is a important part of the game, because when the player creates a song, this editor will let them to make changes on small parts of the song itself.

The editor mode will let the player:

1. Check all the notes on the song
2. Add new notes
3. Delete existing notes
4. Transform a single note into a Holding one
5. Change the start of the song
6. input the end value of the song
7. set a name

This mechanic is still in development.
