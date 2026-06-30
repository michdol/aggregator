# Make services available to localhost
kubectl port-forward service/rabbitmq-service 15672:15672
kubectl port-forward service/rabbitmq-service 5672:5672

