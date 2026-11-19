# Android 接入 

## 引入框架包

请在android studio中引入我们的框架包 aibase.jar和wade-data.jar。 					

## 继承插件的基类

假设子类的类名为，ExtendScriptPlugin： 					

```js
/**
 * Created by wuyoujian on 2017/5/4.
 */

public class ExtendScriptPlugin extends AIWebViewBasePlugin {
```

## 定义H5调用的原生接口

在Android平台定义一个H5调用的原生接口只需要做的两点：一、是public的函数；二、需要增加注解：@JavascriptInterface。如下面的一个插件函数： 					

```js
// 拍照
@JavascriptInterface
@NotProguard
public void JN_Photograph(String functionName) {
  final String  mFunctionName = functionName;
  final String checkPermissinos [] = {Manifest.permission.CAMERA,
          Manifest.permission.WRITE_EXTERNAL_STORAGE};
  PermissionUitls.mContext = getActivity();
  if(!PermissionUitls.isGetAllPermissionsByList(checkPermissinos) ) {
      new AlertDialog
              .Builder(getActivity())
              .setTitle("提示信息")
              .setMessage("该功能需要您接受应用对一些关键权限（拍照）的申请，如之前拒绝过，可到手机系统的应用管理授权设置界面再次设置。")
              .setPositiveButton("确认", new DialogInterface.OnClickListener() {
                  @Override
                  public void onClick(DialogInterface dialog, int which) {
                      checkPermission(mFunctionName,PermissionUitls.PERMISSION_CAMERA_CODE,checkPermissinos);
                  }
              }).show();
  } else {

      jumpToRectCameraActivity(functionName);
  }
}
```

## 配置H5插件

为了配置文件通用性，我们iOS和android采用同一种xml格式的配置文件： 					

```js
<?xml version="1.0" encoding="UTF-8"?>
<plugins>
    <plugin name="ExtendScriptPlugin" class="com.ai.crmapp.ExtendScriptPlugin" packageName="com.ai.crmapp"></plugin>
</plugins>
```

配置中我们可以配置多个plugin。这里仅配置一个，即上面定义的ExtendScriptPlugin。对于iOS平台class和packageName两个字段是无效的。假定我们的配置文件为：h5Plugin.xml。 					

## 注册H5插件

向WebView内核中注册H5插件，在我们框架中提供插件引擎类：AIWebViewPluginEngine。它目前主要两个能力：一、注册上述配置的H5插件；二、调用H5的JS 函数； 					

（1）注册H5插件

```js
private void setH5PluginEngine() {
        AIWebViewPluginEngine_dl engine = AIWebViewPluginEngine_dl.getInstance();
        engine.registerPlugins(this, mWebView, mPluginCfgFile);
    }
```

（2）通过引擎调用H5的JS

我们的回调JS函数以及原生需要调用H5的JS函数都可通过引擎调用H5的JS。 					

```js
runOnUiThread(new Runnable() {
      @Override
      public void run() {

          AIWebViewPluginEngine_dl engine = AIWebViewPluginEngine_dl.getInstance();
          String JS = String.format("%s('%s','%s')","JS_LaunchApp_callback","0","");
          engine.excuteJavascript(JS, new ValueCallback<String>() {
              @Override
              public void onReceiveValue(String value) {
                  LogUtil.d("jsMethod",value);
              }
          });
      }
  });
```

同理，在插件函数里如果会牵涉到操作UI元素（包括原生和H5）都需要在UI主线程中执行，如上面的runOnUiThread 					