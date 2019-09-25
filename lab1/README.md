# Lab 1. Getting started with basic PostrgeSQL functionality

## Business field

Website devoted to publishing and discussing computer game news.

## Graphical ER model

![ER digram](er-diagram.png)

## Database relations and tables

![Tables in database](relations.png)

## Data description

You can find ojbects fields description [here at live GraphQL playground](https://times-are-changing.herokuapp.com/gql). Beware that server may be idle (since it
is hosted for free) and it needs some time to get up and running.


| Relation | Attribute | Data type |
|----------|-----------|-----------|
| user | `login` - unqiue identifier<br>`creationDate` - date when this account was created<br>`lastUpdateDate` - Date when this account was updated last time<br>`role` - defines user access level<br>`name` - user name to refer to him/her<br>`avatarImgId` - avatar picture url, or null of was not set | String<br>Date<br>Date<br>Enum<br>String<br> String |
news | `id` - unique identifier<br>`creationDate` - date when this news was created<br>`lastUpdateDate` - date when this news was updated last time<br>`creatorLogin` - login of the user that created this news.<br>`title` - human-readable sentence that laconically describes this news.<br>`body` - news main body markdown text, it may be vulnerable XSS attacks, be sure to sanitize it on the client side after having converted it to HTML.<br>`promoImgId` - id image to display as the.| Int<br>Date<br>Date<br>String<br>String<br>String |
news_comment |`id` - unique identifier<br>`creationDate` - date when this comment was created<br>`lastUpdateDate` - date when this comment was updated<br>`commentatorLogin` - login of the user that cread the comment<br>`newsId` - id of the news that this comment was attached to<br>`body` - text body of the comment| String<br>Date<br>Date<br>String<br>Int<br>String<body> |
news_rating | `raterLogin` - login of the user that rated the news.<br>`newsId` - id of the news that the user rated.<br>`hasLiked` - defines whether the user liked the news or not. | String<br>Int<br>hasLiked |
