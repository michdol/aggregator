# From root dir
build-producer:
	docker build -t iot-producer:local -f apps/iot_producer/Dockerfile .

build-aggregator:
	docker build -t aggregator:latest -f apps/aggregator/Dockerfile .

kill-a:
	kubectl delete deploy aggregator

delete-img:
	minikube image rm iot-producer:local

delete-img-a:
	minikube image rm aggregator:local
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

rebuild-p: build-producer delete-img load restart

rebuild-a: build-aggregator clear-img-a load-a restart-a

deploy-agg:
	kubectl apply -f ./infra/aggregator/deployment.yaml

deploy-a: build-aggregator delete-img-a load-a deploy-agg

redeploy-a: build-aggregator load-a restart-a

wait-for-termination:
	kubectl wait --for=delete deployment/aggregator --timeout=60s
