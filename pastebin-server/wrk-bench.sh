URL=`python3 test.py`
wrk -t16 -c128 -d10s --latency ${URL}
