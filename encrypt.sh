#!/bin/bash
###############################################################################
# NAME:		    encrypt.sh
#
# AUTHOR:	    Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:	    Facilitates file encryption
#
# CREATED:	    07/27/2021
#
# LAST EDITED:	    06/17/2022
###

if [[ -z "$1" || -z "$2" ]]; then
    >&2 printf 'Usage: %s <encrypt|decrypt> <mod>' "$0"
    exit 1
fi

ENCRYPTED="$2.rs.age"
DECRYPTED="src/$2.rs"

case "$1" in
    decrypt)
        age -d -i ~/.ssh/id_rsa $ENCRYPTED > $DECRYPTED
        ;;
    encrypt)
        age -R ~/.ssh/id_rsa.pub $DECRYPTED > $ENCRYPTED
        ;;
    *)
        >&2 printf 'Usage: %s <encrypt|decrypt> <mod>' "$0"
        RC=1
        ;;
esac

###############################################################################
