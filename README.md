# fsd28-profiler
A custom generator for FSD28 profiles.

FSD28 is an experimental port of [Full Spectrum Dominance](https://fsd-wargame.com/) into a 28mm skirmish game. Compared to the original game the profiles building becomes critical, and a software to do so is really helpful.

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/thelazyone/fsd28-profiler)

## Current State 
[Check out the LIVE DEMO (still WIP)!](https://test.thelazyforger.com/fsd28/)

Currently there is a (possibly broken) implementation is via CLI and a functioning web application deployable with trunk.

<img width="652" alt="image" src="https://github.com/thelazyone/fsd28-profiler/assets/10134358/7c9ed395-9548-4dca-96b5-3ccbdc6511b8">

Once completed it will be a useful tool to test FSD28. The CLI needs a bit more love to become functioning again, but currently the data structure is changing to quickly to maintain two different applications.

## Architecture
The project is divided between a proper library and a CLI software which uses the library and provides all the fancy ASCII art. 
Another module (yet to be implemented) should create fancy-looking cards for printing.
For the future of course a web-based interface is preferable, something in line with the [FSD Builder](https://github.com/thelazyone/fsd_builder) which is written with [Yew](https://github.com/yewstack/yew)

## Deploy
To deploy the application run `trunk serve --release --public-url "./"` and then copy on your server the content of the /dist folder. The last part of the command is necessary to set the relative paths in case you want to serve the application from somewhere that is not the root folder of your website.

## TODOs for a MVP
To get to a minimum viable product the following are still needed:

LIB:
- [x] implementing the default actions for units
- [x] reading and handling of weapon profiles
- [x] weapon-related actions
- [x] options for classes (it's a tricky one!)
- [x] points calculation (propagating it throughout all the profiles)
- [ ] (!) Hero System (will require an extra Modal window)
- [x] saving and loading profiles
- [x] generating a printable (even ASCII) card for each profile.
- [ ] adding a timestamp id to the profiles.
- [ ] adding a unique id to the weapons, for better backward compability.
- [ ] version for save files

WEB
- [x] Basic framework in Yew
- [x] Save-Load with dialog
- [x] profile Visualization
- [x] Add-Remove Profiles
- [x] Edit Profiles (with forms)
- [x] Show Mandatory actions for weapons
- [x] Grey down actions that cannot be taken (because you didn't take a mandatory one or because you are not high tier)
- [ ] Better handle selection of profiles (still buggy) when saving or opening saved ones
- [x] Implement the "Delete Selected" button
- [ ] Anything Roster-Related
