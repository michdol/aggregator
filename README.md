# Make services available to localhost
kubectl port-forward service/rabbitmq-service 15672:15672
kubectl port-forward service/rabbitmq-service 5672:5672
kubectl port-forward service/redis-commander-service 8081:8081
kubectl port-forward service/redis-service 6379:6379
kubectl port-forward service/postgres-service 5432:5432

