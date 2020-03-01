from scrapy.crawler import CrawlerProcess
from scrapy.utils.project import get_project_settings
from lxml import etree
import os
import webbrowser


def cleanup():
    try:
        os.remove("task1.xml")
        os.remove("task2.xml")
        os.remove("task2.xhtml")
    except OSError:
        pass


def scrap_data():
    process = CrawlerProcess(get_project_settings())
    process.crawl('isport')
    process.crawl('portativ')
    process.start()


def task1():
    print("ISPORT")
    root = etree.parse("../output1.xml")
    pages = root.xpath("//page")
    print("URLS")
    for page in pages:
        print(page.xpath("@url")[0])


def task2():
    print("PORTATIV")
    transform = etree.XSLT(etree.parse("../transform.xsl"))
    try:
        result = transform(etree.parse("../output2.xml"))
    except BaseException as err:
        print(err)

    result.write("../output2.xhtml", pretty_print=True, encoding="UTF-8")
    print("OPENING DA BROWSER...")
    webbrowser.open('file://' + os.path.realpath("../output2.xhtml"))


if __name__ == '__main__':
    print("HELLO WORLD")
    cleanup()
    print("CRAWLING DA WEBSAITS", flush=True)
    scrap_data()
    print("DONE")
    while True:
        print("PRESS 1 for ISPORT, 2 FOR PORTATIV:")
        print("> ", end='', flush=True)
        number = input()
        if number == "1":
            task1()
        elif number == "2":
            task2()
        else:
            break
    print("SEEYA!")
