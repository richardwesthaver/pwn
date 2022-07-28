#!/usr/bin/sh

# update pacman key db and pkg list
pacman-key –init && pacman-key –populate
