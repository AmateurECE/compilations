###############################################################################
# NAME:		    Makefile
#
# AUTHOR:	    Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:	    Makefile for the project.
#
# CREATED:	    07/27/2021
#
# LAST EDITED:	    05/31/2022
###

all: build.lock

decrypt: filter.enc
	openssl aes-256-cbc -d -a -pbkdf2 -in $< -out service/src/filter.rs
	touch $<

# build.lock: $(jsIndex) filter.enc
build.lock: filter.enc

# $(jsIndex): $(jsDeps)
# 	npm run build
# 	touch $@

filter.enc: service/src/filter.rs
	openssl aes-256-cbc -a -salt -pbkdf2 -in $< -out $@

###############################################################################
