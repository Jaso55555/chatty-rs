# Chatty-rs




## Running

`cargo run --bin <client/server>`

or if `just` is installed (`cargo install just`)

`just <client/server>`

## Docker

### Build
`docker build -t chatty-rs .`

### Run
`docker run -d -p 5678:5678 --name chatty-rs -v /path/to/config:/config -v /path/for/logs:/logs chatty-rs `
