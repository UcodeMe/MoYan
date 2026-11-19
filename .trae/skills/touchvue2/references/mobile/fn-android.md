# Android 功能 

## 证件拍照

RectCameraActivity 					

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

    private void jumpToRectCameraActivity(String functionName) {
        Intent intent = new Intent(getActivity(), RectCameraActivity.class);
        intent.putExtra("functionName",functionName);
        intent.putExtra("fileId","fileId");
        getActivity().startActivityForResult(intent, 101);
    }
```

## 手写控件

AISignatureView 
它是一个标准的View类，可以类似于系统的View一样的使用方式；他提供保存手写图片、获取手写图片的Bitmap对象、删除保存的图片、设置笔的粗细、设置画布背景、设置画笔颜色等等能力； 					

## 手势密码

AIGesturePasswordLayout 
它是一个标准的View类，用法同系统的View,下面我们给一份使用纯代码布局的范例 					

```js
mGesturePasswordLayout = new AIGesturePasswordLayout(this);
```

## 扫码控件

CaptureActivity 
这个控件是基于ZXing开源库来封装的。 					

```js
private void ToScanCode2(int type) {
	//跳转扫描页面
	Intent intent = new Intent(getActivity(), CaptureActivity.class);
	intent.putExtra("type",type);
	getActivity().startActivityForResult(intent,SCAN_BAR_CODE_REQUEST_CODE);
}
```

