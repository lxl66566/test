from DrissionPage import ChromiumPage

page = ChromiumPage()
page.get("http://g1879.gitee.io/DrissionPageDocs")
print("page open success")
print(page.ele("css:️.-概述 > p").text)
