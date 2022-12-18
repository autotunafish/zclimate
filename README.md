# zclimate

Typically every call to zcash-cli requires typing in a lengthy string and is cumbersome and error-pone enough that very few power user choose to utilize it at all. "zclimate" is an attempt at making that process less tricky by automating calls to zcash-cli and returning the output in a more easily read format. 

This program is EXTREMELY BETA, USE AT YOUR OWN RISK!! 

This video covers the setup and basic usage (and some troubleshooting lol!) https://youtu.be/48V5eYNzE34

The program was designed and tested on Ubuntu 20 with the standard installation procedures given for zcashd. The zclimate folder should live in the '/home/"You"' directory (where your 'Desktop', 'Documents', 'Videos" etc. all live, the --example directory given in the program would be /home/ferris). 

The downloaded zclimate contain files that require 
A. setting increased permission (anyone-read/write for all plus executable for some) and then 
B. copy and pasting inside the zcash/src folder. 
The 3 bash scripts run inotifywait recursively and watch 3 other files for a close event (Rust writing a zcash-cli command to file and closing), copy the file contents to another executable file (this solved an intermittent bug) and run the command where the output is written to another file (12 + another 2 optional bash scripts; one that executes the other 3 climatenotify scripts simultaneously and a seperate one that monitors the debug.log). 
This is a Beta design choice that leverages the sibling file permissions, eliminates having to build the script prior (!?) but also makes it possible to call commands native to any zcashd node regardless of a RPC user or password via the zcash/src folder. It does however already get at least those 2 fields from the zcash.conf if they exist so that the script could run remotely theoretically.

 

Dependencies: inotify-tools, Rust, zcashd full node 
	run the following if required:
	
		sudo apt install inotifytools
	
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	
Refer to the zcash.readthedocs for zcash installation information.
	
	https://zcash.readthedocs.io/en/latest/rtd_pages/Debian-Ubuntu-build.html

More info:

	https://man7.org/linux/man-pages/man7/inotify.7.html
	https://rustup.rs/
	
Download the repo into the dir of your choice, all of the files will be moved into other folders.

	git clone https://github.com/autotunafish/zclimate.git
	
Create a new cargo binary in the '/home/you' directory, probably named zclimate but whatever you'd like. 
Remember the downloaded repo is named zclimate by default so...

	cargo new climate --bin

Then A: 

	replace the main.rs file inside the newly created climate/src folder with the downloaded main.rs
	
and B: 

	replace the Cargo.toml file as well.

Add read\write and executable permissions for half the remaining 'climate' files. The easiest way is to just add all permissions to all files but specifically we need 'anyone-read\write' for all files and 'executable' for climatenotify ( , 1, 2, NOX), for climateuseNOX (1, 3, 5) and debugscript. 

	Use the file explorer and 
	highlight all the files except 'readme' and 'git'. 
	Right-click and select propreties,
	set read\write for all users.
	
Then:
	Highlight
	
	'climatenotify', 
	'climatenotify1', 
	'climatenotify2',
	'climatenotifyNOX', 
	'climateusenox1.txt', 
	'climateusenox3.txt',
	'climateusenox5.txt',
	'debugscript'
	
	Right-click and select properties.
	Select the allow running as executable checkbox.
	
	(Todo: add the chmod 600(?) method)

Then:

	Copy 'debugscript' (1) file into the .zcash folder and
	Copy all the rest (13) of the files into the zcashd/src folder (except the 'git' and 'readme').
	
		'climatenotify', 
		'climatenotify1', 
		'climatenotify2',
		'climatenotifyNOX', 
		'climateuse0.txt', 
		'climateuse1.txt', 
		'climateuse2.txt',
		'climateusenox.txt', 
		'climateusenox1.txt', 
		'climateusenox2.txt',
		'climateusenox3.txt', 
		'climateusenox4.txt',
		'climateusenox5.txt',

Run the climatenotify bash scripts. This can be done by either opening 3 seperate terminals (or terminal tabs) and running the scripts individually or by opening one seperate terminal and running climatenotifyNOX. The program was tested mainly with the former and is recommended use but the latter seemed to work as well, the difference in performance is unknown(!!).

	Open 3 seperate terminals and in each
	navigate to the zcash/src folder and
	run the following:
	
		./climatenotify
		./climatenotify1
		./climatenotify2
		 
	These scripts run seperately from the zclimate binary and will need terminating after you are finished using the program. 
	To terminate, select the terminal and push 'ctrl C'
		
Run zcashd. The Node will need to be fully sync'd in order to call any zcash-cli command besides 'getinfo' (yes) and 'Get All TXs' (maybe?! best just to wait until it's tipped).

	Navigate to the zcash/src folder and
	run your usual ./zcashd launch command.
	Wait for the node to fully catch up.
		
Run zclimate. 

	Navigate to your climate binary folder and 
	run the following:
  
		cargo run
			
The program will prompt you for the paths to
	the zcash.conf file,
	the zcash/src folder and
	the .zcash/debug log.
The paths must be the absolute path and correct (Inputting an invalid path is not recoverable!). If not the program will not function and will likely immediately panic when it tries to open a file not there. The program has no persistant memory and this must be entered every time but only once at the beginning. 
The --example paths are default for the normal zcashd installations such as where zcash.conf lives but can vary.

	Enter path to zcash.conf
	--example /home/ferris/.zcash/zcash.conf

	Enter path to zcash/src
	--example   /home/ferris/zcash/src

	Enter path to .zcash/debug.log 
	--example   /home/ferris/.zcash/debug.log
	
You'll be presented with a list of options.

	Type [] to select:
	G - getinfo 
	| L - Show RX History 
		Shows addresses and allows z_listreceivedbyaddress
		
	| B - Wallet 
		Shows address/account details, balances and allows z_sendmany
		
	| A - All Addresses 
		List all addressess and receivers
		
	| N -New Account 
		Allows z_getnewaccount
		
	| F - New Unified Address
		Allows z_getaddressforaccount
		 
	| H - Get All TXs 
		Iterates through debug.log for all add event txids
		
	| C - z_getbalance 		
		Allows z_getbalance
		
	| O - Operations 
		Shows all opids in memory with z_getoperationstatus
		
	| S - Sign Message 
		Allows signmessage
		
	| V - Verify Message 
		Allows verifymessage
		
	| R - List Unified Receivers |
		Allows z_listunifiedreceivers
		
	| X - Exit
		Close the program


Notes: The program ran fine all through testing until nearly the end of writing the code when (this is the only thing I can figure) an OS update sitting in memory slowed the outside inotifywait scripts to the point where the program ran extremely slow and showed incorrect balance values. The system allowed the 'CLOSE' events to persist so long that the zclimate program was immediately triggered and read in the same data as the call prior to it, before the system had set the new file instance wih the correct output in memory. This need for buffering is echoed in the cp event in the climatenotify scripts and the various thread::sleep events in the program. Some may not be be necessary but any noticible, persistant lag is probably associated with either the system or with zcashd in which case just ensure to apply any pending updates and reboot before tweaking the delay values.

During the 'Send' process, it is possible to enter an invalid 'amount' and the process continue as it does not parse or check the input inside a loop which is required or else it will break the main loop and Exit completely. There was a check against the total balance - 0.00001 for sufficient funds before the option to 'Send' but not a second time. The tx will, however, abort by default anyways with any unknown input (must enter capital 'Y' to send) and even then the z_sendmany call will fail without problem. 

Extra: The 'debugscript' is a bash script that outputs new lines added to the debug.log file in the terminal. It is an extra program unrelated to the first and is for easily viewing the current debug.log information without opening the file which is well over 1 GB and takes a very large amount of resources to even execute. To run it, navigate to your .zcash folder and run the following command (Terminate with Ctrl C)
  
  	./debugscript
