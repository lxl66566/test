from DrissionPage import ChromiumOptions, ChromiumPage

page = ChromiumPage(ChromiumOptions().set_headless(True))
page.get(
    "http://g1879.gitee.io/DrissionPageDocs",
    show_errmsg=True,
    retry=3,
    interval=5,
    timeout=20,
)
print("page open success")
print(page.ele("css=.anchor.anchorWithStickyNavbar_LWe7").text)
