###############################################################################
# NAME:		    Makefile
#
# AUTHOR:	    Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:	    Makefile for the project.
#
# CREATED:	    07/27/2021
#
# LAST EDITED:	    07/27/2021
###

all: build.lock

djangoApp = compilations
staticDir = $(djangoApp)/static/$(djangoApp)
jsIndex = $(staticDir)/js/index.js

pythonDeps = \
	$(shell find $(djangoApp)) \
	MANIFEST.in \
	setup.py

build.lock: $(jsIndex) $(pythonDeps)
	python3 setup.py sdist
	python3 setup.py bdist_wheel
	touch $@

jsDeps = \
	$(shell find js) \
	webpack.config.js \
	package.json \
	package-lock.json

$(jsIndex): $(jsDeps)
	npm run build

###############################################################################
