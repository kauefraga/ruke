#! /bin/bash

binary='ruke-x86_64-unknown-linux-gnu.tar.gz'

if [[ "$OSTYPE" == "darwin"* ]]; then
    binary='ruke-x86_64-apple-darwin.tar.gz'
fi

curl -OLs https://github.com/kauefraga/ruke/releases/latest/download/$binary

tar -zxvf $binary

rm $binary

echo 'Reminder: add ruke in your PATH or move it to a local where PATH is already set'
echo '$ mv ruke /usr/bin/'
