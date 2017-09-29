#How to deploy to Google Cloud:
Build docker machine, if not done already  
`$ docker-machine create --driver virtualbox default`

Set up environment variables  
`$ eval $(docker-machine env default)`

Get IP of docker-machine  
`$ docker-machine ip`

Build image (from directory with Dockerfile)  
`$ docker build -t space_text_img .`

Run image locally  
`$ docker run -tP space_text_img`

List docker processes  
`$ docker ps`

Test that your container is running  
`$ curl {IP from docker-machine}:{PORT from ps}`
``$ curl `docker-machine ip`:{PORT from ps}``

Kill docker container, if can't kill due to flags  
`$ docker stop {NAME from ps}`

Point gcloud CLI to desired project  
`$ gcloud config set project {PROJECT NAME}`

Deploy app to gcloud (from directory with Dockerfile)  
`$ gcloud app deploy`
