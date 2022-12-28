build:
	echo build

docker-setup:
	cd it && docker-compose up -d

clean:
	./it/scripts/stop-apps.sh; echo "Stopping apps if required ... "
	cd it && docker-compose down; echo "Stopping containers if required ... "

integration-test: clean
	cd it && docker-compose up -d
	./it/scripts/start-apps.sh
	cd it && cargo test
	./it/scripts/stop-apps.sh
	cd it && docker-compose down

start-apps:
	./it/scripts/start-apps.sh

stop-apps:
	./it/scripts/stop-apps.sh