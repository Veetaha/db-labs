# LAB 1

## Task

[Link to the task](docs/lab1_bd2-db2019_2020.docx.pdf)

## Variant 6

| Базова сторінка (завдання 1) | Зміст завдання 2     | Адреса інтернет-магазину (завдання 3) |
|------------------------------|----------------------|---------------------------------------|
| www.isport.ua         | Вивести список гiперпосилань | www.portativ.ua |

## Code excerpts

### XPATH www.isport.ua

`src/scrapers/spiders/isport.py`

```python
# -*- coding: utf-8 -*-
from scrapy.http.response import Response
import scrapy


class ISportSpider(scrapy.Spider):
    name = 'isport'
    allowed_domains = ['isport.ua']
    start_urls = ['https://isport.ua']

    def parse(self, response: Response):
        img_elems = response.xpath("//img/@data-src[starts-with(., 'http')]")
        text_elems = response.xpath(
            "//*[not(self::script)][not(self::style)][string-length(normalize-space(text())) > 20]/text()"
        )
        yield {
            'url': response.url,
            'payload':
                [
                    {
                        'type': 'text',
                        'data': text.get().strip()
                    } for text in text_elems
                ] +
                [
                    {
                        'type': 'image',
                        'data': image.get()
                    } for image in img_elems
                ]
        }
        if response.url == self.start_urls[0]:
            link_elems = response.xpath(
                "//a/@href[starts-with(., 'https://isport.ua/') or starts-with(., '/')]"
            )
            links = [
                link.get() for link in link_elems if link.get() != "/"
            ]
            for link in links[:19]:
                if link.startswith("/"):
                    link = "https://isport.ua" + link
                yield scrapy.Request(link, self.parse)

```

### XSLT www.portativ.ua

`src/scrapers/spiders/portativ.py`

```python
# -*- coding: utf-8 -*-
from scrapy.http.response import Response
import scrapy


class PortativSpider(scrapy.Spider):
    name = 'portativ'
    allowed_domains = ['www.portativ.ua']
    start_urls = ['https://portativ.ua/category_2271966.html?tip_bc6d=178778']

    def parse(self, response: Response):
        for product in response.xpath("//*[contains(@class, 'port-i')]")[:20]:
            yield {
                'availability': product.xpath(
                    ".//*[contains(text(), 'наличии')]/text()"
                ).get(),

                'description': product.xpath(
                    "./div[@class = 'cataloggrid-item-name-block']/a/@title"
                ).get(),

                'price': product.xpath(
                    ".//span[contains(@class, 'price-value')]/@content"
                ).get(),

                'preview': product.xpath(
                    ".//img[contains(@class, 'UI-CATALOG-PRODUCT-IMAGE')]/@src"
                ).get()
            }

```

### Files pipelines

`src/scrapers/pipelines.py`

```python
# -*- coding: utf-8 -*-

# Define your item pipelines here
#
# Don't forget to add your pipeline to the ITEM_PIPELINES setting
# See: https://docs.scrapy.org/en/latest/topics/item-pipeline.html
from lxml import etree


class ScrapersPipeline(object):
    def __init__(self):
        self.root: etree.Element = None

    def open_spider(self, spider):
        self.root = etree.Element("data" if spider.name == "isport" else "shop")

    def close_spider(self, spider):
        with open('../output%d.xml' % (1 if spider.name == "isport" else 2), 'wb') as f:
            f.write(etree.tostring(
                self.root, encoding="UTF-8",
                pretty_print=True,
                xml_declaration=True
            ))

    def process_item(self, item, spider):
        if spider.name == "isport":
            page = etree.Element("page", url=item["url"])
            for payload in item["payload"]:
                fragment = etree.Element("fragment", type=payload["type"])
                fragment.text = payload["data"]
                page.append(fragment)
            self.root.append(page)
        else:
            product: etree.Element = etree.Element("product")
            product.set("availability", item["availability"])

            desc = etree.Element("description")
            desc.text = item["description"]

            pr = etree.Element("price")
            pr.text = item["price"]

            preview = etree.Element("preview")
            preview.text = item["preview"]

            product.append(desc)
            product.append(pr)
            product.append(preview)
            self.root.append(product)
        return item

```

### Task 1

`src/main.py`

```python
print("ISPORT")
root = etree.parse("../output1.xml")
pages = root.xpath("//page")
print("URLS")
for page in pages:
    print(page.xpath("@url")[0])
```

### Task 2

`src/main.py`

```python
print("PORTATIV")
    transform = etree.XSLT(etree.parse("../transform.xsl"))
    try:
        result = transform(etree.parse("../output2.xml"))
    except BaseException as err:
        print(err)

    result.write("../output2.xhtml", pretty_print=True, encoding="UTF-8")
    print("OPENING DA BROWSER...")
    webbrowser.open('file://' + os.path.realpath("../output2.xhtml"))
```

`src/transform.xsl`

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
                <title>LABA</title>
            </head>
            <body>
                <h1>PRODUCTS:</h1>
                <xsl:apply-templates select="/shop"/>
            </body>
        </html>
    </xsl:template>
    <xsl:template match="/shop">
        <table border="1">
            <thead>
                <tr>
                    <td>UAH</td>
                    <td>TEXT</td>
                    <td>AVAIL</td>
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
                <xsl:apply-templates select="description"/>
            </td>
            <td><xsl:apply-templates select="price"/></td>
            <td><xsl:apply-templates select="@availability"/></td>
        </tr>
    </xsl:template>
    <xsl:template match="image">
        <img>
            <xsl:attribute name="src">
                <xsl:value-of select="text()"/>
            </xsl:attribute>
        </img>
    </xsl:template>
    <xsl:template match="price">
        <xsl:attribute name="style">
            font-weight: bold
        </xsl:attribute>
        <xsl:value-of select="text()"/>
    </xsl:template>
    <xsl:template match="description">
        <xsl:attribute name="style">
            font-size: 20px
        </xsl:attribute>
        <xsl:value-of select="text()"/>
    </xsl:template>
    <xsl:template match="@availability">
        <xsl:value-of select="."></xsl:value-of>
    </xsl:template>
</xsl:stylesheet>
```

## Generated files

* [output1.xml](output1.xml)
* [output2.xml](output2.xml)
* [output2.xhtml](output2.xhtml)

## Screenshots

![](https://user-images.githubusercontent.com/36276403/75633466-a2918f00-5c0d-11ea-91f2-0ebce78e9379.png)

![](https://user-images.githubusercontent.com/36276403/75633415-3747bd00-5c0d-11ea-8251-26284024e998.png)
