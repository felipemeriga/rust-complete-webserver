# rust-complete-webserver

## Introduction

This is a simple example of a Rust webserver, containing some important features that 
may be required on a corporate environment. Therefore, here is what we tried to cover on this 
application:
- Refined error handling for the application and HTTP responses
- Configuration management
- Using traits for database entities
- authorization/authentication
- Unit tests

Which tools we are using:
- Actix
- Tokio
- Serde
- Mongodb
- testcontainers
- twelf
- mockall
- thiserror


##  How to Run

We have a Makefile in this project, so you can execute the following commands:

### Lint project 

```shell
make lint
```

### Run unit tests
```shell
make test
```

### Run application
```shell
make run
```

### Build application
```shell
make build
```
