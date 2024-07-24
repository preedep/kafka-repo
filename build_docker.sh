docker build --cpuset-cpus="0,1" --platform linux/amd64 -t kafka-repo-service:latest .
docker tag kafka-repo-service:latest nickmsft/kafka-repo-service:latest
docker push nickmsft/kafka-repo-service:latest

