#!/bin/bash

if [ -e database.sqlite3 ]
then
	rm database.sqlite3
fi

cat create_db.sql | sqlite3 database.sqlite3
