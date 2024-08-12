export DOCKER_BUILDKIT=1
docker build --platform linux/amd64 -t kafka-repo-service:latest .
# Check if the command succeeded
if [ $? -ne 0 ]; then
    echo "Error: some_command failed"
    exit 1
fi
docker tag kafka-repo-service:latest nickmsft/kafka-repo-service:latest
docker push nickmsft/kafka-repo-service:latest
