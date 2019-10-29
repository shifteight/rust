#!/bin/sh
for file in `ls`; do
if [ -x "$file" ]; then
   if [ "$file" != "rmX.sh" ]; then
      rm -rf $file
   fi
fi
done