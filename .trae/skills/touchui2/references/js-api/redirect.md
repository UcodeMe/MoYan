# 页面跳转
打开页面、获取页面信息的方法集合。 

## 属性

| 方法                                                | 描述                                                         |
| --------------------------------------------------- | ------------------------------------------------------------ |
| $.redirect.buildUrl(subsys, page, listener, params) | 根据子系统、页面名、监听方法和参数构造页面url，其中subsys可以不传，如：$.redirect.buildUrl(page, listener, params); |
| openNav(title, page, listener, params, subsys)      | 在外框中打开新标签页面，如果有已经打开的标题为title的页面则切换到该页面 |
| openNavByUrl(title, url, subsys)                    | 在外框中按url打开新页面，，如果有已经打开的标题为title的页面则切换到该页面 |
| redirectNav(title, page, listener, params, subsys)  | 跳转外框当前打开的页面，如果title和当前标签页面不一致，则修改当前标签页面的title |
| redirectNavByUrl(title, url, subsys)                | 按url跳转外框当前打开的页面，如果title和当前标签页面不一致，则修改当前标签页面的title |
| switchToNav(title)                                  | 切换到标题为title的外框标签页面                              |
| getNavContent()                                     | 获取外框当前打开页面的窗口window对象                         |
| getNavContentByTitle(title)                         | 获取外框打开的标签页面中标题为title的窗口window对象          |
| closeNav()                                          | 关闭当前外框打开的标签页面                                   |
| closeNavByTitle(title)                              | 根据title关闭外框指定标签页                                   |

注意：
如果需要跨模块打开或跳转页面，则openNav、redirectNav方法必须传subsys参数，具体传其它模块地址端口之后，“？”号之前的相对路径例如："/personserv/personserv", "/ordercentre/ordercentre"