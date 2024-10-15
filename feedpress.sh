#!/bin/bash

## This exists as a stepping off point - and may expand in the future to setup initial crontabs,
## confirm various things, setup default settings, etc.

## For now it just fires off the server.
echo "Starting feedpress with command: feedpress --serve"
/app/feedpress --serve

