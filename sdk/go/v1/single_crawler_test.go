package v1

import (
	"log"
	"testing"
)

func TestSingleCrawler(t *testing.T) {
	crawler, err := New("127.0.0.1:8081")
	if err != nil {
		log.Fatalln(err)
	}

	singleCrawler, err := crawler.SingleCrawler("http://www.baidu.com", 600)
	if err != nil {
		log.Fatalln(err)
	}
	log.Println(singleCrawler)
}
