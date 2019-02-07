#!/bin/bash
now=`date +%s`

while true
do
   data=`date -d @"$now" +%Y-%m-%d`
  echo $data
  rustup update nightly
  if [ "$?" -eq "0" ]; then echo "Bingo"; break; fi
  now=$(($now-86400))
done