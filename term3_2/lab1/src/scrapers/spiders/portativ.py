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
