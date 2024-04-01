(cd ./go-setup && go run .) &
cd ./go-single && ./measure.sh
cd ..
wait
