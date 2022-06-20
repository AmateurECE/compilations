#!/bin/bash
###############################################################################
# NAME:		    secure.sh
#
# AUTHOR:	    Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:	    Facilitates file encryption
#
# CREATED:	    07/27/2021
#
# LAST EDITED:	    06/20/2022
###

if [[ -z "$1" ]]; then
    >&2 printf 'Usage: %s <encrypt|decrypt>' "$0"
    exit 1
fi

FILES=(
    "model/src/lib.rs"
    "frontend/src/filter.rs"
    "compilations/src/extractor.rs"
)
DECRYPTED_ARCHIVE=protected.tar.gz
ENCRYPTED_ARCHIVE=${DECRYPTED_ARCHIVE}.age

case "$1" in
    decrypt)
        age -d -i ~/.ssh/id_rsa $ENCRYPTED_ARCHIVE > $DECRYPTED_ARCHIVE
        tar xzvf $DECRYPTED_ARCHIVE
        ;;
    encrypt)
        tar czvf $DECRYPTED_ARCHIVE ${FILES[@]}
        age -R ~/.ssh/id_rsa.pub $DECRYPTED_ARCHIVE > $ENCRYPTED_ARCHIVE
        ;;
    *)
        >&2 printf 'Usage: %s <encrypt|decrypt>' "$0"
        RC=1
        ;;
esac

###############################################################################
