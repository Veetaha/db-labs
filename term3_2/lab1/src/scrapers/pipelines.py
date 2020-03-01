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
