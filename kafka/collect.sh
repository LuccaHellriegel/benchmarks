sudo -v
for i in {1..20}; do
    ./fresh.sh &
    FRESH_PID=$!
    sleep 20
    ./measure-rust.sh
    kill $FRESH_PID
    ./fresh.sh &
    FRESH_PID=$!
    sleep 20
    ./measure-go.sh
    kill $FRESH_PID
done
