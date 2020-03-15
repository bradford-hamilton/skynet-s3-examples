package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/credentials"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/s3"
	"github.com/aws/aws-sdk-go/service/s3/s3manager"
)

func main() {
	// Build a skynet-s3 client
	creds := credentials.NewStaticCredentials(
		os.Getenv("ACCESS_KEY_ID"),
		os.Getenv("SECRET_KEY"),
		"",
	)
	sess := session.New(&aws.Config{
		Credentials: creds,

		// Here is where we will point to our private cloud s3 server
		Endpoint: aws.String(os.Getenv("SKYNET_S3_SERVER")),

		// Region is required even though we are connecting to our private cloud skynet server
		Region: aws.String("us-east-1"),

		// Currently only have an http server running - TODO: Update when I change to a VM with ssl certs
		DisableSSL: aws.Bool(true),

		// This ensures the URL is constructed like: `http://some.ip.address/skynet-s3` and not `http://skynet-s3.some.ip.adddress` (newer aws s3 approach)
		S3ForcePathStyle: aws.Bool(true),
	})
	s3Client := s3.New(sess)

	// Test bucket and key to use
	bucket := aws.String("go_test_bucket")
	key := aws.String("go_test_file")

	// Create a new bucket
	_, err := s3Client.CreateBucket(&s3.CreateBucketInput{Bucket: bucket})
	if err != nil {
		fmt.Println(err.Error())
		return
	}

	// Upload a new object to our bucket
	_, err = s3Client.PutObject(&s3.PutObjectInput{
		Body:   strings.NewReader("You know what they say - a doctor a day keeps the apples away"),
		Bucket: bucket,
		Key:    key,
	})
	if err != nil {
		fmt.Printf("Failed to upload data to %s/%s, %s\n", *bucket, *key, err.Error())
		return
	}
	fmt.Printf("Successfully created bucket %s and uploaded data with a key of %s\n", *bucket, *key)

	// Create a file for storing the content we will download from our skynet s3 bucket
	file, err := os.Create("object_contents")
	if err != nil {
		fmt.Println("Failed to create file", err)
		return
	}
	defer file.Close()

	// Create downloader and download contents from the bucket/key we just created
	d := s3manager.NewDownloader(sess)
	numBytes, err := d.Download(file, &s3.GetObjectInput{
		Bucket: bucket,
		Key:    key,
	})
	if err != nil {
		fmt.Println("Failed to download file", err)
		return
	}
	fmt.Println("Downloaded file", file.Name(), numBytes, "bytes")
}
