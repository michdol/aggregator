# From root dir
build:
	docker build -t iot-producer:local -f apps/iot_producer/Dockerfile .

# maybe need to remove old image or somehow override old one
load:
	minikube image load iot-producer:local

restart:
	kubectl rollout restart deployment/iot-producer

rebuild: build load restart
