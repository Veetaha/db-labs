# lab2

## Prerequisites

* [Rust toolchain](https://www.rust-lang.org/learn/get-started)
* libpq

## Bootstrap (Linux)

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Add .cargo/bin to .bash_profile
echo 'export PATH="${HOME}/.cargo/bin:${PATH}"' >> ~/.bash_profile
# Update $PATH variable
source ~/.bash_profile
# Install libpq
sudo apt update
sudo apt install libpq-dev
# Install diesel_cli
cargo install diesel_cli --no-default-features --features postgres
```

## Documentation

[Task (pdf)](docs/task.pdf)

## Task number 6

| Search by attributes | Fulltext search |
|---------------------|----------------------|
| number range, value set | the word doesn't occur, the word is included |

### Normalized database model


| Relation | Attribute | Data type |
|----------|-----------|-----------|
| user | ```login` - unqiue identifier<br>`creationDate` - date when this account was created<br>`lastUpdateDate` - Date when this account was updated last time<br>`role` - defines user access level<br>`name` - user name to refer to him/her<br>`avatarImgId` - avatar picture url, or null of was not set | String<br>Date<br>Date<br>Enum<br>String<br> String |
news | `id` - unique identifier<br>`creationDate` - date when this news was created<br>`lastUpdateDate` - date when this news was updated last time<br>`creatorLogin` - login of the user that created this news.<br>`title` - human-readable sentence that laconically describes this news.<br>`body` - news main body markdown text, it may be vulnerable XSS attacks, be sure to sanitize it on the client side after having converted it to HTML.<br>`promoImgId` - id image to display as the.| Int<br>Date<br>Date<br>String<br>String<br>String |
news_comment |`id` - unique identifier<br>`creationDate` - date when this comment was created<br>`lastUpdateDate` - date when this comment was updated<br>`commentatorLogin` - login of the user that cread the comment<br>`newsId` - id of the news that this comment was attached to<br>`body` - text body of the comment| Int<br>Date<br>Date<br>String<br>Int<br>String<body> |
news_rating | `id` - unique identifier<br> `raterLogin` - login of the user that rated the news.<br>`newsId` - id of the news that the user rated.<br>`hasLiked` - defines whether the user liked the news or not. | Int<br>String<br>Int<br>hasLiked |

