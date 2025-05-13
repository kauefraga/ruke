#!/bin/bash

sum=0

for i in {1..1000}
do
    output=$(cargo run rm -n "$i" )

    time_and_unit=$(echo $output | grep -o -E 'duration: [0-9]+\.[0-9]+(ms|µs)' | grep -o -E '[0-9]+\.[0-9]+(ms|µs)')

    time=$(echo $time_and_unit | grep -o -E '[0-9]+\.[0-9]+')
    unit=$(echo $time_and_unit | grep -o -E '(ms|µs)')

    if [ "$unit" = "µs" ]; then
        time=$(echo "$time / 1000" | bc -l)
    fi

    sum=$(echo "$sum + $time" | bc -l)
done

average=$(echo "scale=6; $sum / 100" | bc -l)

echo "Average time elapsed  $average ms"

