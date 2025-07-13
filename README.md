## What?
a packet sniffer and a visualizer using frontend from [Crepe's old sniffer](https://github.com/Crepe-Inc/Iridium/tree/main/frontend) with minimal changes

## Features
- [x] Packet visualizing
- [x] correct sequence

## What doesn't work
- loading the editor to see the packet more clearer does not work due to a promise error ( i've never used svelte or js so yea )
- trying to export selected packets ( as clicking invokes the editor so it becomes unresposive )
- all the buttons doesn't work ( most are useless anyways )
- reading server to client messages ( as i'm hooking a specific handler inside and don't know really how to go about it )

## Usage
- you should have node installed as it is required for the frontend then inisde the directory run ``npm install``,``npm run build`` then ``npm run start`` use the address it showed you and you should be there ~( for some reason it isn't working with browsers other than firefox so yea that too )~
- compile the dll wait 5 secs for the game to start then start ( note: injecting directly doesn't work )
- the dll interfaces with the frontend but you still have console output you can either ignore it or look at it instead of the visualizer ( depends on person tbh )

## Special thanks
- [Yon](https://github.com/yoncodes)
- [Ken](https://github.com/hcazten)

## issues 
- open in issues :)

## Contributions
- You're welcome to submit through a pull request

## License
This project is licensed under the [MIT](https://github.com/Yoshk4e/sonetto-sniffery/blob/master/LICENSE) licesnse
