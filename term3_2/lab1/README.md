# Лабораторна робота No 1. Вивчення базових операцій обробки XML-документів

## Завдання

[Постановка завдання](docs/lab1_bd2-db2019_2020.docx.pdf)

## Варіант завдання

5 варіант згідно номера у списку групи

| Базова сторінка (завдання 1) | Зміст завдання 2     | Адреса інтернет-магазину (завдання 3) |
|------------------------------|----------------------|---------------------------------------|
| www.football.ua         | Кількість графічних фрагментів по кожному документу | www.moyo.ua |

## Лістинг коду

### Збирання даних зі сторінки www.football.ua  

`src/scrapers/spiders/football.py`

```python
class FootballSpider(scrapy.Spider):
    name = 'football'
    allowed_domains = ['football.ua']
    start_urls = ['https://football.ua/newsarc/']

    def parse(self, response: Response):
        all_images = response.xpath("//img/@src[starts-with(., 'http')]")
        all_text = response.xpath(
            "//*[not(self::script)][not(self::style)][string-length(normalize-space(text())) > 30]/text()")
        yield {
            'url': response.url,
            'payload': [{'type': 'text', 'data': text.get().strip()} for text in all_text] +
                       [{'type': 'image', 'data': image.get()} for image in all_images]
        }
        if response.url == self.start_urls[0]:
            all_links = response.xpath(
                "//a/@href[starts-with(., 'https://football.ua/')][substring(., string-length() - 4) = '.html']")
            selected_links = [link.get() for link in all_links][:19]
            for link in selected_links:
                yield scrapy.Request(link, self.parse)
```

### Збирання даних зі сторінки www.moyo.ua

`src/scrapers/spiders/moyo.py`

```python
class MoyoSpider(scrapy.Spider):
    name = 'moyo'
    allowed_domains = ['www.moyo.ua']
    start_urls = ['https://www.moyo.ua/telecommunication/smart/']

    def parse(self, response: Response):
        products = response.xpath("//section[contains(@class, 'product-tile_product')]")[:20]
        for product in products:
            yield {
                'description': product.xpath("./@data-name").get(),
                'price': product.xpath("./@data-price").get(),
                'img': product.xpath("./@data-img").get()
            }
```

### Запис зібраних даних до файлів

`src/scrapers/pipelines.py`

```python
class ScrapersPipeline(object):
    def __init__(self):
        self.root = None

    def open_spider(self, spider):
        self.root = etree.Element("data" if spider.name == "football" else "shop")

    def close_spider(self, spider):
        with open('task%d.xml' % (1 if spider.name == "football" else 2), 'wb') as f:
            f.write(etree.tostring(self.root, encoding="UTF-8", pretty_print=True, xml_declaration=True))

    def process_item(self, item, spider):
        if spider.name == "football":
            page = etree.Element("page", url=item["url"])
            for payload in item["payload"]:
                fragment = etree.Element("fragment", type=payload["type"])
                fragment.text = payload["data"]
                page.append(fragment)
            self.root.append(page)
        else:
            product = etree.Element("product")
            desc = etree.Element("description")
            desc.text = item["description"]
            pr = etree.Element("price")
            pr.text = item["price"]
            img = etree.Element("image")
            img.text = item["img"]
            product.append(desc)
            product.append(pr)
            product.append(img)
            self.root.append(product)
        return item
```

### Завдання №1

`src/main.py`

```python
def task1():
    print("Task #1")
    root = etree.parse("task1.xml")
    pages = root.xpath("//page")
    print("Number of graphical documents for scrapped documents:")
    for page in pages:
        url = page.xpath("@url")[0]
        count = page.xpath("count(fragment[@type='image'])")
        print("%s: %d" % (url, count))
```

### Завдання №2

`src/main.py`

```python
def task2():
    print("Task #2")
    transform = etree.XSLT(etree.parse("task2.xsl"))
    result = transform(etree.parse("task2.xml"))
    result.write("task2.xhtml", pretty_print=True, encoding="UTF-8")
    print("XHTML page will be opened in web-browser...")
    webbrowser.open('file://' + os.path.realpath("task2.xhtml"))
```

`src/task2.xsl`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns="http://www.w3.org/1999/xhtml">
    <xsl:output
        method="xml"
        doctype-system="http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd"
        doctype-public="-//W3C//DTD XHTML 1.1//EN"
        indent="yes"
    />
    <xsl:template match="/">
        <html xml:lang="en">
            <head>
                <title>Task 2</title>
            </head>
            <body>
                <h1>Table of products:</h1>
                <xsl:apply-templates select="/shop"/>
                <xsl:if test="count(/shop/product) = 0">
                    <p>There are no products available</p>
                </xsl:if>
            </body>
        </html>
    </xsl:template>
    <xsl:template match="/shop">
        <table border="1">
            <thead>
                <tr>
                    <td>Image</td>
                    <td>Description</td>
                    <td>Price, UAH</td>
                </tr>
            </thead>
            <tbody>
                <xsl:apply-templates select="/shop/product"/>
            </tbody>
        </table>
    </xsl:template>
    <xsl:template match="/shop/product">
        <tr>
            <td>
                 <xsl:apply-templates select="image"/>
            </td>
            <td>
                <xsl:apply-templates select="description"/>
            </td>
            <td>
                <xsl:apply-templates select="price"/>
            </td>
        </tr>
    </xsl:template>
    <xsl:template match="image">
        <img alt="image of product">
            <xsl:attribute name="src">
                <xsl:value-of select="text()"/>
            </xsl:attribute>
        </img>
    </xsl:template>
    <xsl:template match="price">
        <xsl:value-of select="text()"/>
    </xsl:template>
    <xsl:template match="description">
        <xsl:value-of select="text()"/>
    </xsl:template>
</xsl:stylesheet>
```

## Лістинг згенерованих файлів

### task1.xml

```xml
<?xml version='1.0' encoding='UTF-8'?>
<data>
  <page url="https://football.ua/newsarc/">
    <fragment type="text">Новости футбола Украины и мира — football.ua</fragment>
    <fragment type="text">Преимущества регистрации на football.ua:</fragment>
    <fragment type="text">Общение с журналистами football.ua и isport.ua в конференциях</fragment>
    <fragment type="text">Возможность получить e-mail на домене @football.ua</fragment>
    <fragment type="text">Соглашение о конфиденциальности</fragment>
    <fragment type="text">Я уже зарегистрирован, впустите!</fragment>
    <fragment type="text">Введите адрес вашей электронной почты</fragment>
    <fragment type="text">Шахтер спасся от потери очков в матче против Десны</fragment>
    <fragment type="text">"Бело-фиолетовые" воспользовались численным преимуществом и добыли победу.</fragment>
    <fragment type="text">Симоне Индзаги: Лацио мог бы быть на вершине в других лигах</fragment>
    <fragment type="text">Наставник римлян прокомментировал победу над "грифонами".</fragment>
    <fragment type="text">Бруну Фернандеш — первый игрок МЮ, забивший дебютный гол с пенальти в АПЛ</fragment>
    <fragment type="text">Португальский хавбек отличился взятием ворот в матче против Уотфорда.</fragment>
    <fragment type="text">Политика в сфере конфиденциальности и персональных данных</fragment>
    <fragment type="image">https://football.ua/tpl/football/img/header/logo.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/630x373/415/415711.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/110x72/415/415714.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/110x72/415/415715.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/110x72/415/415713.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/110x72/415/415712.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/110x72/415/415709.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/110x72/415/415710.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/110x72/415/415707.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/110x72/415/415695.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/110x72/415/415696.jpg</fragment>
  </page>
  <page url="https://football.ua/ukraine/415711-shakhter-spassja-ot-poteri-ochkov-v-matche-protiv-desny.html">
    <fragment type="text">Шахтер спасся от потери очков в матче против Десны — football.ua</fragment>
    <fragment type="text">Преимущества регистрации на football.ua:</fragment>
    <fragment type="text">Общение с тысячами футбольных болельщиков</fragment>
    <fragment type="text">Возможность получить e-mail на домене @football.ua</fragment>
    <fragment type="text">Соглашение о конфиденциальности</fragment>
    <fragment type="text">Я уже зарегистрирован, впустите!</fragment>
    <fragment type="text">Введите адрес вашей электронной почты</fragment>
    <fragment type="text">Жизнь футболиста: как игроки выбирают себе недвижимость</fragment>
    <fragment type="text">Футбольное пиршество Шахтера обошлось без новых блюд на столе</fragment>
    <fragment type="text">Серия А. Главные итоги зимнего трансферного окна</fragment>
    <fragment type="text">10 главных моментов в карьере Криштиану Роналду</fragment>
    <fragment type="text">Луиш Нани — о начале в Лиссабоне, воспоминаниях об МЮ и мечтах в MLS</fragment>
    <fragment type="text">Топ-10 трансферов, которые еще могут состояться в январе</fragment>
    <fragment type="text">Ромелу Лукаку: Однажды Конте сказал, что я играл как мусор</fragment>
    <fragment type="text">Пауло Дибала: Нужно надеть маску, как гладиатор, и драться</fragment>
    <fragment type="text">Главные выводы после первого матча Шахтера на сборах</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/210x151/414/414797.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/210x151/415/415272.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/210x151/415/415187.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/210x151/413/413096.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/210x151/414/414655.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/news/210x151/413/413361.jpg</fragment>
    <fragment type="image">https://s.ill.in.ua/i/football/team/logo_sm/0x20/24.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/football/team/logo_sm/0x20/23.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/football/team/logo_sm/0x20/219.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/football/team/logo_sm/0x20/290.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/football/team/logo_sm/0x20/406.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/football/team/logo_sm/0x20/1303.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/football/team/logo_sm/0x20/407.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/football/team/logo_sm/0x20/293.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/football/team/logo_sm/0x20/78.png</fragment>
    <fragment type="image">https://s.ill.in.ua/i/football/team/logo_sm/0x20/220.png</fragment>
  </page>
</data>
```

### task2.xml

```xml
<?xml version='1.0' encoding='UTF-8'?>
<shop>
  <product>
    <description>Смартфон Apple iPhone XS 64GB Space Gray</description>
    <price>19999</price>
    <image>https://img.moyo.ua/img/products/4278/27_96.jpg</image>
  </product>
  <product>
    <description>Смартфон Apple iPhone 7 Plus 32 GB (Black)</description>
    <price>13999</price>
    <image>https://img.moyo.ua/img/products/1954/78_96.jpg</image>
  </product>
  <product>
    <description>Смартфон Huawei Y6 Pro DS Gold</description>
    <price>4139</price>
    <image>https://img.moyo.ua/img/products/2346/93_96.jpg</image>
  </product>
  <product>
    <description>Смартфон Huawei Y6 2019 DS Amber Brown</description>
    <price>2999</price>
    <image>https://img.moyo.ua/img/products/4404/71_96.png</image>
  </product>
  <product>
    <description>Смартфон Huawei P20 (EML-L29) DS Black</description>
    <price>9999</price>
    <image>https://img.moyo.ua/img/products/4311/59_96.jpg</image>
  </product>
</shop>
```

### task2.xhtml

```xhtml
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en">
  <head>
    <title>Task 2</title>
  </head>
  <body>
    <h1>Table of products:</h1>
    <table border="1">
      <thead>
        <tr>
          <td>Image</td>
          <td>Description</td>
          <td>Price, UAH</td>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>
            <img alt="image of product" src="https://img.moyo.ua/img/products/4278/27_96.jpg"/>
          </td>
          <td>Смартфон Apple iPhone XS 64GB Space Gray</td>
          <td>19999</td>
        </tr>
        <tr>
          <td>
            <img alt="image of product" src="https://img.moyo.ua/img/products/4336/84_96.png"/>
          </td>
          <td>Смартфон Huawei P Smart 2019 Aurora Blue</td>
          <td>5499</td>
        </tr>
        <tr>
          <td>
            <img alt="image of product" src="https://img.moyo.ua/img/products/2798/61_96.jpg"/>
          </td>
          <td>Смартфон Apple iPhone 7 32 GB (Gold)</td>
          <td>9999</td>
        </tr>
        <tr>
          <td>
            <img alt="image of product" src="https://img.moyo.ua/img/products/4547/25_96.jpg"/>
          </td>
          <td>Смартфон Xiaomi Redmi 8A 2/32GB Ocean Blue</td>
          <td>2999</td>
        </tr>
      </tbody>
    </table>
  </body>
</html>
```

## Приклади роботи програми

![lab](img/img1.png)

![lab](img/img2.png)

![lab](img/img3.png)

![lab](img/img4.png)
