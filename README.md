Server for Raspberry pi zero w for water system for plant.
Android client for this server you can find [there](https://github.com/Nirklav/PlantsCare_Client).

# Requirements:
* Raspberry Pi Zero W.
* Camera for Raspberry Pi Zero.
* Power supply 12A 1-2A.
* Diaphragm pump 12V ~1A.
* Power converter 12v to 5v 1-2A.
* Funduino 40x16mm water sensor or something like that.
* Npn transistor.
* MOSFET.
* Resistors.

# Scheme:
![alt tag](https://raw.github.com/Nirklav/PlantsCare_Server/master/scheme.png)

# Setup:
I recommend use lite version of raspberry pi os.

1. Install git.
2. Install rust.
3. Clone this repo.
4. Build rust project in release mode.
5. Modify config.json for your environment.
6. Create daemon via systemd.
