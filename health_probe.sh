while [ true ]; do
  grpc-health-probe -addr=[::1]:8000 -service=hello.Hello
  sleep 1
done
