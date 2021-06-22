#!/usr/bin/sh

if [[ $EUID -ne 0 ]]; then
	echo "This script must be run as root" 
	exit 1
fi

cp ./dwm-status /usr/local/bin/

