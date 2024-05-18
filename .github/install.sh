#! /bin/bash

curl -OLs https://github.com/kauefraga/ruke/releases/download/v0.1.8/ruke-x86_64-unknown-linux-gnu.tar.gz

tar -zxvf ruke-x86_64-unknown-linux-gnu.tar.gz

echo 'Add ruke in your PATH or move it to a local where PATH is already set'
echo '$ echo "mv ruke /usr/bin/"'
