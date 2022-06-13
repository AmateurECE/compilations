#!/bin/sh
###############################################################################
# NAME:		    encrypt.sh
#
# AUTHOR:	    Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:	    Facilitates file encryption
#
# CREATED:	    07/27/2021
#
# LAST EDITED:	    06/13/2022
###

ENCRYPTED=filter.rs.age
DECRYPTED=src/filter.rs

case "$1" in
    decrypt)
        age -d -i ~/.ssh/id_rsa $ENCRYPTED > $DECRYPTED
        ;;
    *)
        age -R ~/.ssh/id_rsa.pub $DECRYPTED > $ENCRYPTED
        ;;
esac

###############################################################################
