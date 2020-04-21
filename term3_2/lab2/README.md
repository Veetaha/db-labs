# lab2

## Bootstrap

### Installation

The following script downloads and runs the linux binaries `console_ui` and `worker`
from GitHub releases. Make sure to run it the root of the repo or manually add
`$REDIS_PORT` env variable.

```
./bootstrap.sh
```


### Prerequisties

Docker and docker-compose are optional, you can run your own Redis cluster/
Just make sure change a `.env` to specify the redis port.

* [docker](https://docs.docker.com/install/)
* [docker-compose](https://docs.docker.com/compose/install/)
* [rustc and cargo](https://www.rust-lang.org/tools/install)

### Compile and run from sources

You can build and run the console ui or worker with `--help` args with these commands
```
cargo run -p console_ui -- --help
cargo run -p worker -- --help
```

## Redis layout
![](https://user-images.githubusercontent.com/36276403/79699035-6e456100-8295-11ea-9b72-2eb024a71cdf.png)

Data structures used in this lab:

- **SET**: set is used for quick lookups and tests for membership and to store data
that must be unique in no specific order.
Based on these features it was used to store the logins of users that are online
because they must be unique, to store user logins by their role in order to get
quick lookups and because they are unordered and unique. Also there is a separate
set for each message status, because this allows for quick message lookups and
to get message statistics for a particular user by intersecting with their outgoing messages
(which are also stored in a set) ids set.
- **HASH**: hash allows for storing key-value pairs that is useful for associating
related datums together ala documents (like in document-based databases). This is
the reason this data structure was used to store messages themselves:
    ```json
    {
        "text": "message text",
        "sender": "sender_login",
        "receiver": "receiver_login"
    }
    ```
    Each such message hash is keyed by its id.
- **LIST**: this is double-ended queue which allows for ordered access. This is
the reason it was used as a message queue, such that the new messages are pushed to
the end of the list and pending messages are taken for processing from the start of the list.
The list was also used for storing incomming messages, because this way we can
also store them in order they were comming, so that the user sees messages
in the historical order.

- **ZLIST**: this is a map that allows for ordering its entries by some numeric key.
This data structure was used to store the statistics about the number of spam
and delivered messages per user such that each user is matched with its number of
smap/delivered messages. By storing this data as a ZLIST me are able to get
the most spammy/chatty users very easily just by using ZRANGE query.

- **PUB_SUB_CHANNEL**: this is a feature of Redis that allows for
organizing a MSMC channel (multiple sender and multiple consumer). Such a channel
was used for keeping the events journal, so that the worker send events when
they detect spam and users send events when they log in / log out.



## Run Redis

```bash
docker-compose up
```

## Run worker

```
~/dev/db-labs/term3_2/lab2 (master) $ cargo run -p worker -- --help

Redis chat 0.1.0
Veetaha (github.com/Veetaha)
A chat console application that interacts with Redis database.

USAGE:
    worker [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --spam-check-latency <spam-check-latency>     [default: 4]
        --total-workers <total-workers>               [default: 2]
```

## Run console ui

```
~/dev/db-labs/term3_2/lab2 (master) $ cargo run -p console_ui -- --help

Redis chat 0.1.0
Veetaha (github.com/Veetaha)
A chat console application that interacts with Redis database.

USAGE:
    console_ui <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    find-users             Find users according to the given conditions
    help                   Prints this message or the help of the given subcommand(s)
    log-in                 Log in as a user or admin
    log-out                Log out
    online-users           View the list of users currently online
    send-message           Send a message to the chat
    sent-messages          View messages sent by the user grouped by status
    view-events-journal    View the events journal as an admin
    view-messages          View the messages received by the user
```

Demo:
![demo.gif](https://user-images.githubusercontent.com/36276403/79698711-5e2c8200-8293-11ea-82cd-93d0e30fdb9b.gif)


## Sreenshots

![](https://user-images.githubusercontent.com/36276403/79699362-d85f0580-8297-11ea-8929-ecd055bfa2e8.png)
![](https://user-images.githubusercontent.com/36276403/79699384-f9275b00-8297-11ea-8740-05f56c681e17.png)
![](https://user-images.githubusercontent.com/36276403/79699401-14926600-8298-11ea-871f-18a0813548fe.png)
