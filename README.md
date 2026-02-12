# Xbox controller workaround for Intiface Central on Linux

## Dependecies
* Rust
* Socat
* Intiface Central
* Root
* Xbox Controller (or any Force Feedback controllers supported by evdev)

## Configurations
A fifo (first in first out) file is created at a path specified in main.rs and tty.sh.\
By default this exists somewhere in your home directory.\
You can change this to wherever you want just make sure they point to the same location. \
The main.rs should crash if you screwed up.

The tty.sh will overwrite one of your ttyS ports # See Caveats
By default this is set to ttyS31\
This is because overwriting the file will turn it into a link that is destroyed after use\
I would not recommend using ttyS0 as it would be the most likely port to be used\
In contrast ttyS31 would likely only be in use if all other ports were in use or an application set it as there default \# I would have set this to the pts that socat makes but Intiface doesn't detect that, nor any devices >31


## Setup
\# Requires root to alter ttyS devices due to being a hacky workaround. Read #Caveats
* Install any dependecies
* Download the code and extract it to a folder
* Navigate to the folder in the terminal and run tty.sh 
* Run main.rs
* Open intiface central
* Scroll down to Show Advanced/Experimental Settings and enable
* Scroll down to Serial Port and enable
* Go to Devices, scroll down to Serial Devices, expand. You should see Add Serial Devices
* Select the following
    * Protocol Type : activejoy
    * Port Name : /dev/ttyS31 #This is configurable
    * Baud : 9600
    * Data Bits : 8
    * Parity : N
    * StopBits : 1
* Click add Serial Device
* Start the intiface server
* Scan for devices

You can verify if this worked by using the vibrate bar under your device\
If you stop the tty.sh while the intiface server is running it will crash. Stop intiface first.

## Caveats
This is ultimately not a proper solution. I do not understand how to get websockets working, and that would be the proper way to do something like this. That would remove the need for root, the ttyS, and make the only thing you need being rust. If you care to you can fork this and replace the fifo code with websocket. From what I found on the Inficace docs the code snippet for websockets is underdeveloped connection with code. In its current form its most suitable for controlling devices. This may just me being dumb.


## Support
This product will go unsupported. It only exists because there are no existing repos for this. 
