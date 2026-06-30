# From root dir
build-producer:
	docker build -t iot-producer:local -f apps/iot_producer/Dockerfile .

build-aggregator:
	docker build -t aggregator:local -f apps/aggregator/Dockerfile .

kill-a:
	kubectl delete deploy aggregator
# maybe need to remove old image or somehow override old one
load:
	minikube image load iot-producer:local --overwrite

load-a:
	minikube image load aggregator:local --overwrite

clear-img-a:
	minikube image rm aggregator:local

restart:
	kubectl rollout restart deployment/iot-producer
restart-a:
	kubectl rollout restart deployment/aggregator

rebuild-p: build-producer load restart

rebuild-a: build-aggregator kill-a clear-img-a load-a restart-a
deploy-agg:
	kubectl apply -f ./infra/aggregator/deployment.yaml

deploy-a: build-aggregator load-a deploy-agg

redeploy-a: build-aggregator load-a restart-a
