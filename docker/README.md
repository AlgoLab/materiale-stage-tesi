# Docker

Come creare un'immagine Docker da un dockerfile e caricarla su docker.io (bisogna avere un account su docker.io e far parte dell'organizzazione algolab).
```
# create a Dockerfile
# make changes to Dockerfile
cd /dir/Dockerfile/
docker build -t algolab/{image-name}:{image-tag} . # eg. asgal:v1.1.6

# if you have test data, test the newly created local image
docker run algolab/{image-name}:{image-tag}

# tag the image (maybe unnecessary):
docker images # to get the {image-id} (IMAGE ID column)
docker tag {image-id} algolab/{image-name}:{image-tag}

# login to docker.io
docker login -u {username} --password-stdin docker.io
# or docker login docker.io

# push the local image
docker push {image-name}:{image-tag}
```
