package v1

import (
	"log"
	"testing"
)

func TestSingleCrawler(t *testing.T) {
	crawler, err := New("127.0.0.1:8281")
	if err != nil {
		log.Fatalln(err)
	}

	var a = []string{
		"https://blog.csdn.net/LL845876425/article/details/89743593",
		"http://www.baidu.com",
		"https://github.com/dollarkillerx/rust_single_crawler",
		"https://www.cnblogs.com/liluxiang/p/9474907.html",
		"https://github.com/dollarkillerx/XSQ",
		"https://kafka.apache.org/documentation/streams/",
		"https://www.bing.com/?mkt=zh-CN",
	}

	for _, v := range a {
		singleCrawler, err := crawler.SingleCrawler(v, 6000)
		if err != nil {
			log.Println(err, v)
			continue
		}
		if len(singleCrawler.Body) == 0 {
			log.Println("hat: ", v)
			continue
		}
		log.Println(singleCrawler.HttpCode)
	}
}
