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

If you are using the same commands as I used in generate RSA keys chapter, then you don't need to mess with key mountings and use commands bellow to link them.

```sh
# navigate to folder containing your keys
ln -s ./grants_private_key.pem ./auth-api/grants_encoding_key
ln -s ./grants_public_key.pem ./auth-api/grants_decoding_key
ln -s ./public_token_private_key.pem ./auth-api/token_encoding_key
ln -s ./public_token_public_key.pem ./auth-api/token_decoding_key
```

Then just edit the `docker-compose.yaml` according to comments.

### Finally start it

```sh
docker compose up -d
```

## How to use

First you need to register or login to get your public token.\
You then put this token into your `Authorization` header (in bearer format) and use this public token to make all your requests.

For user that has all grants you can you this login info:
```json
{
    "email": "root@root.root",
    "password": "toor"
}
```

If you enabled `DEBUG`, then you will get debug responses from all routes.\
It will also enable `/docs` endpoint so don't forget to check it out!
