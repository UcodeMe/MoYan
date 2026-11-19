# web 端接入 

## 引入plugin.js 

此js对原生插件API做一个PC端的API的映射，目前我们给一个基础的插件API样例JS，可根据项目的需要增加插件接口，plugin.js内容如下： 					

```js
$(document).ready(function(){
    
    mboss.noTerminal = {
        error: function(){
            alert('请在手机终端运行此方法');
            return false;
        },
        returnEmptyStr: function(){
            return '';
        }, 
    };
    /* 框架相关方法兼容 */
    if (!window.ExtendScriptPlugin){
        window.ExtendScriptPlugin = {};
        ExtendScriptPlugin.JN_Photograph = mboss.noTerminal.error;//拍照
        
        /* 本地存储 */
        //localStorage操作
        mboss.storage = {
            get: function(name){
                return window.localStorage.getItem(name) || '';
            },
            set: function(name, value){
                window.localStorage.setItem(name, value);
            },
            remove: function(name){
                window.localStorage.removeItem(name); 
            },
            clear:function(){
                window.localStorage.clear();
            }
        };

        ExtendScriptPlugin.JN_GetValueWithKey = mboss.storage.get;
        ExtendScriptPlugin.JN_SetValueWithKey = mboss.storage.set;
        ExtendScriptPlugin.JN_RemoveAllData = mboss.storage.clear;
        ExtendScriptPlugin.JN_RemoveDataByKey = mboss.storage.remove;
    }
});
```

## 调用原生接口 

假定原生定义的JN_Photograph如下 					

```js
- (void)JN_Photograph:(NSString*)functionName;
```

非iframe页面中使用

请使用window对象引用插件对象： 					

```js
window.ExtendScriptPlugin.JN_Photograph('callbackFunctionName');
```

在iframe页面中使用

请使用top对象引用插件对象： 					

```js
top.ExtendScriptPlugin.JN_Photograph('callbackFunctionName');
```

## js回调函数的处理 

目前我们是采用给原生接口传入回调函数名的方式实现JS回调，如上面的JN_Photograph。后面我们会支持JS匿名函数的回调方式。 