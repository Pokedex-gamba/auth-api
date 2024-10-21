# Auth API

## How to run

### Generate RSA keys

You must generate RSA keys for grants and public token.

```sh
# grants keys
openssl genrsa -out grants_private_key.pem 2048
openssl rsa -pubout -in grants_private_key.pem -out grants_public_key.pem
# public token keys
openssl genrsa -out public_token_private_key.pem 2048
openssl rsa -pubout -in public_token_private_key.pem -out public_token_public_key.pem
```

### Download required files

You must download the `docker-compose.yaml` file and `database` folder.

Or clone the repo:

```sh
git clone https://github.com/Pokedex-gamba/auth-api.git
```

### Edit docker-compose.yaml

Make sure decoding keys inside the container are mounted to public keys on your machine
and encoding keys are mounted to private keys on your machine or set them in environment variables.

If you followed chapter `Generate RSA keys`, then your commands will look similar to the commands bellow.\
Make sure the names of files to be linked and the link names are the same for keys to be linked correctly.

```sh
# navigate from parent folder containing keys to folder containing this repo
cd auth-api
# correctly link keys
ln -s ../grants_private_key.pem ./grants_encoding_key
ln -s ../grants_public_key.pem ./grants_decoding_key
ln -s ../public_token_private_key.pem ./token_encoding_key
ln -s ../public_token_public_key.pem ./token_decoding_key
```

Then just edit the `docker-compose.yaml` according to comments.

### Finally start it

```sh
docker compose up -d
```

## How to use

First you need to register or login to get your public token.\
You then put this token into your `Authorization` header (in bearer format) and use this public token to make all your requests.

For user that has all grants you can use this login info:
```json
{
    "email": "root@root.root",
    "password": "toor"
}
```

If you enabled `DEBUG`, then you will get debug responses from all routes.\
It will also enable `/docs` endpoint so don't forget to check it out!
