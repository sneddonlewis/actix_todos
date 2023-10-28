#!/bin/bash

cd ..
sudo docker-compose up -d

until sudo pg_isready -h localhost -p 5432 -U root
do
	echo "Waiting for postgres"
	sleep 2;
done
echo "postgres container is now running"
sudo docker-compose down
