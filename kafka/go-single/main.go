package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/twmb/franz-go/pkg/kgo"
)

func main() {
	opts := []kgo.Opt{
		kgo.ConsumerGroup(time.Now().String()), //send all msgs each time
		kgo.SeedBrokers("localhost:9092"),
		kgo.ConsumeTopics("topicA"),
		kgo.DisableAutoCommit(),
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

	// Produce to topic B
	produceToTopicB := func(ctx context.Context, record *kgo.Record) {
		record.Topic = "topicB"
		client.Produce(ctx, record, func(r *kgo.Record, err error) {
			if err != nil {
				fmt.Fprintf(os.Stderr, "error in produce: %v\n", err)
				panic("error in produce")
			}
		})
	}

	startTime := time.Now()
	recordsToCommit := make([]*kgo.Record, 1000)
	count := 0

	s := time.Now()
	for {
		fetches := client.PollRecords(ctx, 1000)
		if err := fetches.Err(); err != nil {
			fmt.Fprintf(os.Stderr, "error in fetch: %v\n", err)
			panic("error in fetch")
		}

		index := 0
		for _, fetch := range fetches {
			for _, topic := range fetch.Topics {
				for _, partition := range topic.Partitions {
					for _, record := range partition.Records {
						produceToTopicB(ctx, record)
						recordsToCommit[index] = record
						index++
					}
				}
			}
		}

		count += index
		if count/10000 > (count-index)/10000 {
			client.Flush(ctx)
			fmt.Printf("Processed %d, Elapsed %v \n", count, time.Since(s))
			s = time.Now()
		}
		recordsToCommit = recordsToCommit[:index]

		// Commit all records after producing
		if len(recordsToCommit) > 0 {
			client.MarkCommitRecords(recordsToCommit...)
			if err := client.CommitMarkedOffsets(ctx); err != nil {
				panic(err)
			}
			recordsToCommit = make([]*kgo.Record, 1000)
		}

		if count == 100000 {
			client.Flush(ctx)
			break
		}

	}

	elapsedTime := time.Since(startTime)
	fmt.Printf("Fetch and produce operation took %v\n", elapsedTime)
	cancel() // Ensure context is cancelled to stop any ongoing operations
}
