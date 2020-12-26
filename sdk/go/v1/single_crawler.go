package v1

import (
	"context"

	"github.com/dollarkillerx/rust_single_crawler/sdk/go/v1/proto"
	"google.golang.org/grpc"
)

type SingleCrawler struct {
	client proto.SingleCrawlerClient
}

func New(addr string) (*SingleCrawler, error) {
	conn, err := grpc.Dial(addr, grpc.WithInsecure())
	if err != nil {
		return nil,err
	}
	client := proto.NewSingleCrawlerClient(conn)

	return &SingleCrawler{
		client: client,
	}, nil
}

// timeout: mill
func (c *SingleCrawler) SingleCrawler(url string, timeout uint64) (*proto.CrawlerResp, error) {
	return c.client.SingleCrawler(context.TODO(),&proto.CrawlerReq{
		Url: url,
		Timeout: timeout,
	})
}