package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"sync"
	"syscall"
	"time"

	"github.com/twmb/franz-go/pkg/kgo"
	"google.golang.org/protobuf/proto"
)

func main() {
	opts := []kgo.Opt{
		kgo.ConsumerGroup(time.Now().String()), //send all msgs each time
		kgo.SeedBrokers("localhost:9092"),
		kgo.ConsumeTopics("topicA"),
		// kgo.DisableAutoCommit(),
		kgo.RequireStableFetchOffsets(),
		// kgo.FetchMinBytes(1e6),
		kgo.AllowAutoTopicCreation(),
		// kgo.FetchMaxBytes(1000 * 100 * 1024), // Buffer 1000 records, 100KB each
		kgo.OnPartitionsAssigned(func(ctx context.Context, c *kgo.Client, m map[string][]int32) { fmt.Println(fmt.Sprint(m)) }),
		kgo.OnPartitionsRevoked(func(ctx context.Context, c *kgo.Client, m map[string][]int32) {
			fmt.Println("revoked: " + fmt.Sprint(m))
		}),
		kgo.OnPartitionsLost(func(ctx context.Context, c *kgo.Client, m map[string][]int32) {
			fmt.Println("lost: " + fmt.Sprint(m))
		}),
		// kgo.WithLogger(kgo.BasicLogger(os.Stdout, kgo.LogLevelDebug, nil)),
	}
	client, err := kgo.NewClient(opts...)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error creating client: %v\n", err)
		os.Exit(1)
	}
	defer client.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Handle SIGINT and SIGTERM.
	go func() {
		sigCh := make(chan os.Signal, 1)
		signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)
		<-sigCh
		cancel()
	}()

	recordChan := make(chan *kgo.Record, 10000)
	wg := sync.WaitGroup{}
	wg.Add(1)
	go func() {
		produceToTopicB(ctx, client, recordChan)
		wg.Done()
	}()

	startTime := time.Now()
	count := 0

	s := time.Now()
	for {
		fetches := client.PollFetches(ctx)
		if err := fetches.Err(); err != nil {
			fmt.Fprintf(os.Stderr, "error in fetch: %v\n", err)
			panic("error in fetch")
		}

		index := 0
		for _, fetch := range fetches {
			for _, topic := range fetch.Topics {
				for _, partition := range topic.Partitions {
					for _, record := range partition.Records {
						recordChan <- record
						index++
					}
				}
			}
		}

		count += index
		if count/100000 > (count-index)/100000 {
			fmt.Printf("Processed %d, Elapsed %v \n", count, time.Since(s))
			s = time.Now()
		}

		if count == 1_000_000 {
			close(recordChan)
			wg.Wait()
			break
		}
	}

	elapsedTime := time.Since(startTime)
	fmt.Printf("Fetch and produce operation took %v\n", elapsedTime)
	cancel() // Ensure context is cancelled to stop any ongoing operations
}

func produceToTopicB(ctx context.Context, client *kgo.Client, recordChan <-chan *kgo.Record) {
	for record := range recordChan {
		m := createMessageFromKafkaRecord(record.Value, record.Headers)
		payload, err := proto.Marshal(m)
		if err != nil {
			panic(err)
		}
		client.Produce(ctx, &kgo.Record{Topic: "topicB", Key: record.Key, Value: payload}, func(r *kgo.Record, err error) {
			if err != nil {
				fmt.Fprintf(os.Stderr, "error in produce: %v\n", err)
				panic("error in produce")
			}
		})
	}
	client.Flush(ctx)

}

func createMessageFromKafkaRecord(payload []byte, headers []kgo.RecordHeader) *ProtoMessage {
	msg := &ProtoMessage{
		Payload: payload,
		Headers: make(map[string]string),
	}

	for _, header := range headers {
		key := string(header.Key)
		value := string(header.Value)
		switch key {
		case "message_type":
			msg.MessageType = value
		case "some_id":
			msg.SomeId = value
		default:
			msg.Headers[key] = value
		}
	}

	if msg.SomeId == "" || msg.MessageType == "" {
		panic("problem with incoming data")
	}

	return msg
}
