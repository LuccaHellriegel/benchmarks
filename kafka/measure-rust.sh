(cd ./go-setup && go run .) &
cd ./rust-rdkafka-single && ./measure.sh
cd ..
wait
