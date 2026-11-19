# iOS 接入 

## 引入框架包

引入 AIBase.framework 框架包。 					

## 定义穿梭协议 

此协议继承自系统的穿梭协议JSExport。 在协议中定义插件提供给H5端调用的原生API，如图所示我们定义的插件基类的穿梭协议： 					

```js
/**
 *  扩展插件类
 *  1、JSExport是JS与OC之间的穿梭机；
 *  2、把需要扩展的API定义到protocol:JSExport里，在插件类里实现接口；
 *  3、若是多参数的API，可以采用JSExportAs来声明接口,可以参考ScriptPluginBase；默认的情况，在JS端是会把参数tag首字母大写拼接成JS的function调用方式，例如：test:(NString*)str key:(NString*)k --> testKey(str,k);
 *  4、注意，扩展的API,如果要操作UI的，一定要在主线程里调用。很多情况，jscontext是非主线程调用API；
 */

@protocol JN_BasePluginExport <JSExport>

// 基础的能力接口
// 1、数据存取
JSExportAs(JN_SetValueWithKey,- (void)JN_SetValueWithKey:(NSString*)key value:(NSString*)value);
- (NSString*)JN_GetValueWithKey:(NSString*)key;
- (void)JN_RemoveAllData;
- (void)JN_RemoveDataByKey:(NSString*)key;

// 2、数据安全
- (NSString*)JN_Encrypt:(NSString*)context;
- (NSString*)JN_Decrypt:(NSString*)encrypt;
- (NSString*)JN_IDMD5:(NSString*)userName;
- (NSString*)JN_Token:(NSString*)params;
JSExportAs(JN_ShowFadeMsg, -(void)JN_ShowFadeMsg:(NSString *)msg status:(int)status);

// 隐藏键盘
- (void)JN_HideKeyboard;
// 获取手机型号等信息
- (NSString *)JN_GetPhoneModule;
@end
```

由于OC与JS定义函数的语法不同，针对有多个入参的函数，应该采用JSExportAs做函数语法的映射，如上图的 JN_SetValueWithKey，它在JS中的定义为： 					

```js
function JN_SetValueWithKey(key,value); 
```

在OC中定义为： 					

```js
- (void)JN_SetValueWithKey:(NSString*)key value:(NSString*)value;
```

为了后面的讲述，我们假设插件定义的穿梭协议： 					

```js
@protocol JN_ExtendPluginExport <JSExport>
@end
```

## 继承插件的基类 

继承插件的基类:AIWebViewBasePlugin   

假设子类的类名为：ExtendScriptPlugin 

## 实现穿梭协议 

插件类需要实现穿梭协议定义的API： 					

```js
@interface ExtendScriptPlugin : AIWebViewBasePlugin<JN_ExtendPluginExport>
@end
```

 请在ExtendScriptPlugin中实现JN_ExtendPluginExport中定义的函数。
注意：在实现穿梭协议的函数的时候，如果该函数有操作UI，包括H5和原生的UI元素的操作，请在UI主线程中执行这些代码 如下面的代码所示： 					

```js
- (void)JN_Photograph:(NSString*)functionName {
    __weak CRMMainVC *vc = (CRMMainVC*)self.vc;
    dispatch_async(dispatch_get_main_queue(), ^{
        [vc JN_Photograph:functionName];
    });
}
```

上面dispatch_async就是iOS平台推荐的主线程中执行代码的标准写法，这样`[vc JN_Photograph:functionName]`就会在UI主线程中执行。 	

## 配置H5插件 

为了配置文件通用性，我们iOS和android采用同一种xml格式的配置文件 					

```
<?xml version="1.0" encoding="UTF-8"?>
<plugins>
    <plugin name="ExtendScriptPlugin" class="com.ai.crmapp.ExtendScriptPlugin" packageName="com.ai.crmapp"></plugin>
</plugins>js
```

配置中我们可以配置多个plugin。这里仅配置一个，即上面定义的ExtendScriptPlugin。对于iOS平台class和packageName两个字段是无效的。假定我们的配置文件为：h5Plugin.xml。 	

## 注册H5插件 

向WebView内核中注册H5插件，在我们框架中提供插件引擎类：AIWebViewPluginEngine。它目前主要两个能力：一、注册上述配置的H5插件；二、调用H5的JS 函数； 					

（1）注册H5插件

```js
- (void)registerH5Engine:(UIWebView*)webView {
    NSData *configData = [[AIPluginTools SharedObj] roadResWithName:@"h5Plugin.xml"];
    AIWebViewPluginEngine *engine = [AIWebViewPluginEngine SharedObj];
    [engine registerPluginsInVC:self webView:webView configData:configData];
}
```

上述代码读取插件配置文件h5Plugin.xml，然后通过通过插件引擎在webview上注册插件。 					

（2）通过引擎调用H5的JS

我们的回调JS函数以及原生需要调用H5的JS函数都可通过引擎调用H5的JS 					

```js
dispatch_async(dispatch_get_main_queue(), ^{
    NSString *js = [NSString stringWithFormat:@"%@('%@')",@"JS_SetScanResult",result];
    AIWebViewPluginEngine *engine = [AIWebViewPluginEngine SharedObj];
    [engine excuteJavascript:js];
});
```

上述对象js即我们需要调用H5中的JS。这里我们也采用在主线程中执行调用（因为这个执行的JS会操作H5的图片展示控件） 					