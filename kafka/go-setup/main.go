package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/twmb/franz-go/pkg/kgo"
)

func main() {
	opts := []kgo.Opt{
		kgo.SeedBrokers("localhost:9092"), // Kafka broker addresses
		kgo.AllowAutoTopicCreation(),
	}

	client, err := kgo.NewClient(opts...)
	if err != nil {
		panic(fmt.Sprintf("error creating client: %v", err))
	}

	defer client.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	err = client.Ping(ctx)
	if err != nil {
		panic(fmt.Sprintf("error connecting: %v", err))
	}

	// Handle SIGINT and SIGTERM.
	go func() {
		sigCh := make(chan os.Signal, 1)
		signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)
		<-sigCh
		cancel()
	}()

	// Produce 100000 random messages to topic B
	const messageCount = 1000000
	const messageSize = 100 * 1024 // 100KB

	message := make([]byte, messageSize)
	for i := 0; i < messageSize; i++ {
		message[i] = 1
	}
	for i := 0; i < messageCount; i++ {
		record := &kgo.Record{
			Topic: "topicA",
			Key:   message[:30],
			Value: append(message, []byte("message "+fmt.Sprint(i))...),
			Headers: []kgo.RecordHeader{
				{Key: "some_id", Value: []byte(fmt.Sprintf("id-%d", i))},
				{Key: "message_type", Value: []byte(fmt.Sprintf("type-%d", i))},
			},
		}
		client.Produce(ctx, record, func(r *kgo.Record, err error) {
			if err != nil {
				panic(fmt.Sprintf("error producing message: %v", err))
			}
		})

		fmt.Printf("Produced %d messages so far\n", i)

	}

	client.Flush(ctx)

	fmt.Printf("Finished producing %d messages\n", messageCount)
}
