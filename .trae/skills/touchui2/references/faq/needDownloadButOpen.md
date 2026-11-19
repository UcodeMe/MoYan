# 需要下载的文件被浏览器直接打开

## 问题现象

我们开发中时常遇到一个这样的情况：当我们给 ``<a>`` 标签设置了 ``href="template.txt"``，希望点击后能下载 “template.txt” 文件，但是却被浏览器直接打开了。

此时右键 “另存为” 又可以正常下载。

## 解决方法

我们只需要给 WEB 容器配置里把对应文件类型扩展名文件的 mime-type 设置成可以下载的类型即可，比如：

XML:

```xml
<mime-mapping> 
	<extension>txt</extension> 
	<mime-type>application/octet-stream</mime-type> 
</mime-mapping>
```

SpringBoot 启动类：
```java
@Bean
public WebServerFactoryCustomizer webServerFactoryCustomizer(){
	return new WebServerFactoryCustomizer<ConfigurableServletWebServerFactory>() {
		@Override
		public void customize(ConfigurableServletWebServerFactory factory) {
			MimeMappings mappings = new MimeMappings(MimeMappings.DEFAULT);
			mappings.add("txt", "application/octet-stream; charset=utf-8");
			factory.setMimeMappings(mappings);
		}
	}
}
```
