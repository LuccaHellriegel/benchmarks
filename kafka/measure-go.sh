(cd ./go-setup && go run .) &
cd ./go-single && ./measure.sh >>../log.txt
cd ..
wait
