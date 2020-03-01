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
