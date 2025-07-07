# Ip analysis

## Purpose
This is to aid analysis of trackers. Its purpose is to resolve domains and format them. 

## Instructions 
This works from a .txt export from wireshark when opening a .pcap. file.
Go Wireshark > statistics > IPv$ Statistics > All Addresses > Save as > output.txt.
You can also use other methods to convert into a .txt file,  tcpdump -A -r dump.pcap > output.txt also appears to work.
If you do not have a .pcap file, any list of ips (one per line) will work. 
Then run grep -E -o "([0-9]{1,3}[\.]){3}[0-9]{1,3}" output.txt >> filtered.txt | sort -u to massage the data into the correct format. 
Then replace add that to this repo at the root level. 
Then run the program and pipe (>>) it to a file.
