docker stop kafka-repo-service
docker rm kafka-repo-service
docker run -d --name kafka-repo-service -p 8888:8888 kafka-repo-service:latest