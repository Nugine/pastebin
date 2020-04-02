URL=`python3 test.py`
ab -k -v 1 -n 1000000 -c 128 ${URL}
