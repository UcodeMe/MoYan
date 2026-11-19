# iOS 功能 

## 证件拍照

假设子类的类名为，ExtendScriptPlugin： 					

```js
//243px x 153px
CGFloat width = 200 * 243 / 153;
CGRect markFrame = CGRectMake((self.view.frame.size.width - width)/2.0, 184, width, 200);
AICaptureView *captureView = [[AICaptureView alloc] initWithFrame:self.view.bounds];
self.captureView = captureView;
[_captureView cropRectForInterest:markFrame];
[self.view addSubview:captureView];
```

注意：上述cropRectForInterest设置证件拍照的区域框。 					

## 手写控件

控件中提供了画笔的基本的操作：笔的大小、笔的颜色、撤销一笔、重做一笔、清除所有笔画、保存签名图片。 					

```js
__weak SignatureViewController *wSelf = self;
self.signView = [[SignatureView alloc] initWithFrame:CGRectMake(0, [DeviceInfo navigationBarHeight], self.view.frame.size.width, [DeviceInfo screenHeight] - [DeviceInfo navigationBarHeight] - 44) status:^(SigntureStatus status) {
  //
  SignatureViewController *sSelf = wSelf;
  if (status == SignatureStatusBegin) {
      [sSelf.undoItem setEnabled:YES];
  }
}];

[_signView setPenSize:3.0];
[self.view addSubview:_signView];
```

## 手势密码

```js
// 创建控件
_passwordView = [[AIGesturePasswordView alloc] initWithFrame:CGRectMake(0, 120, self.view.frame.size.width,  self.view.frame.size.height - 60 - 120)];
    [_passwordView setDelegate:self];
    [_passwordView setDefaultColor:[UIColor blackColor]];
    [_passwordView setPathColor:[UIColor colorWithHex:0x378FC9]];
    [_passwordView setSelectColor:[UIColor colorWithHex:0x378FC9]];
    [_passwordView setWrongColor:[UIColor redColor]];
    [self.view addSubview:_passwordView];

// 实现手势控件回调
#pragma mark - AIGesturePasswordViewDelegate
// 通过手势设置密码
- (void)gesturePasswordView:(AIGesturePasswordView*)sender password:(NSString*)password {
}
// 密码个数小于4位，无效
- (void)gestureInvalidPasswordView:(AIGesturePasswordView*)sender {
    
}
```

## 扫码控件

```
QRViewController *qrController = [[QRViewController alloc] init];
[picker presentViewController:qrController animated:YES completion:^{
  //
  [qrController startScanQRCodeWithFinish:^(NSString *result, QRCodeScanStatus status) {
      //
      [picker dismissViewControllerAnimated:YES completion:^{}];
      
      ImagePickerController *sSelf = wSelf;
      if (sSelf.finishBlock) {
          sSelf.finishBlock(ImagePickerTypeScanQRCode,ImagePickerStatusSuccess,result);
      }
  }];
}];
```

 QRViewController是一个ViewController控制器，所以页面的启动可以根据需要用push还是用modal方式，上图实例中是采用的modal方式；startScanQRCodeWithFinish启动扫描，通过block的方式回调传回扫描的结果。 					

## 二维码识别和生成

是通过给UIImage做一个类别扩展，方法定义如下： 					

```
// 生成二维码
+ (UIImage *)generateQRCode:(NSString *)code width:(CGFloat)width height:(CGFloat)height;
// 二维码识别
- (NSArray *)recognitionQRCodeFromImage;
```

QRViewController是一个ViewController控制器，所以页面的启动可以根据需要用push还是用modal方式，上图实例中是采用的modal方式；startScanQRCodeWithFinish启动扫描，通过block的方式回调传回扫描的结果。 		

## 验证码控件

```js
CaptchaControl *codeCtrl = [[CaptchaControl alloc] initWithFrame:CGRectMake(40, 100, 90, 30) interval:60];
    [codeCtrl setBackgroundColor:[UIColor lightTextColor]];
    [codeCtrl addTarget:self action:@selector(getCode:) forControlEvents:UIControlEventTouchUpInside];
    [codeCtrl setDefaultText:@"验证码"];
    [self.view addSubview:codeCtrl];
```

## 轮播控件

```js
CycleBannerView *banner = [[CycleBannerView alloc] initWithFrame:view.bounds];
    NSArray *images = @[@"http://pic22.nipic.com/20120717/9774499_115645635000_2.jpg",
                        @"http://pic4.nipic.com/20090919/3372381_123043464790_2.jpg",
                        @"http://www.9doo.net/__demo/jd0024/upload/b1.jpg",
                        @"http://pic.58pic.com/58pic/13/18/50/23K58PIC38v_1024.jpg",
                        @"http://pic2.ooopic.com/10/57/50/93b1OOOPIC4d.jpg"];
    [banner reloadData:images];
    [banner setPageControlPos:PageControlPositionRight];
    [banner autoScroll];
    [view addSubview:banner];
```

可设置pageControl的位置；也可设置是否是自动轮播，默认不自动轮播。 	

## 自消失的提示框

默认位置弹出

```js
[FadePromptView showPromptStatus:@"当前设置暂时没有办法发送邮件" duration:1.0 finishBlock:nil];
```

指定位置弹出

```js
[FadePromptView showPromptStatus:@"请输入您的笔记~" duration:0.6 positionY:[DeviceInfo screenHeight]- 300 finishBlock:^{
    //
}];
```

上述两个范例都是可以支持提示框消失后的回调。 					

## 九宫格

```js
self.gridView = [[GridMenuView alloc] initWithFrame:CGRectMake(0, [DeviceInfo navigationBarHeight], [DeviceInfo screenWidth], [DeviceInfo screenHeight] - [DeviceInfo navigationBarHeight])];
    [_gridView setDelegate:self];
    [_gridView setColumnCount:3];
    [_gridView setRowHeight:120];
    
    [self.view addSubview:_gridView];
    
    [_gridView appendingMenusData:[self gridData]];

- (NSArray *)gridData {
    
    GridMenuItem *item1 = [[GridMenuItem alloc] init];
    item1.icon = @"http://ico.58pic.com/iconset01/Doraemon-icons/gif/79802.gif";
    item1.title = @"菜单1";
    item1.iconSize = CGSizeMake(60, 60);
    item1.titleFont = [UIFont systemFontOfSize:14];
    item1.titleColor = [UIColor grayColor];
    
    GridMenuItem *item2 = [[GridMenuItem alloc] init];
    item2.icon = @"http://ico.58pic.com/iconset02/fast_icon_users/gif/13406.gif";
    item2.title = @"菜单2";
    item2.iconSize = CGSizeMake(60, 60);
    item2.titleFont = [UIFont systemFontOfSize:14];
    item2.titleColor = [UIColor grayColor];
    
    GridMenuItem *item3 = [[GridMenuItem alloc] init];
    item3.icon = @"http://ico.58pic.com/iconset01/Doraemon-icons/gif/79792.gif";
    item3.title = @"菜单3";
    item3.iconSize = CGSizeMake(60, 60);
    item3.titleFont = [UIFont systemFontOfSize:14];
    item3.titleColor = [UIColor grayColor];
    
    
    NSArray *menus = @[item1,item2,item3];
    return menus;
}
```

## icon菜单

```js
AIActionSheet *sheet = [[AIActionSheet alloc] initInParentView:self.tabBarController.view delegate:self];
for (int i = 0; i < 7; i ++) {
	AISheetItem * item = [[AISheetItem alloc] init];
	item.icon = @"capture.png";
	item.title = [NSString stringWithFormat:@"测试测-%d",i];
	[sheet addActionItem:item];
}
[sheet show];
```

## 图片视频选择器

ImagePickerController 这个类集成了图片的选择，系统的拍照、系统的录像、二维码识别等能力。使用简单请参考.h里的接口定义即可 					

## 邮件和短信发送器

MailSMSController 这个类集成了邮件和短信的发送，使用简单请参考.h里的接口定义即可。 	

## HTTP通信及数据model一体化封装

通信引擎类NetworkTask，它封装了常用的http通信API：文件上传、GET、POST、PUT、DELETE等等，它需要结合数据model类配套使用。 数据model基类NetResultBase。 					

```js
// 1、接口调用
NSDictionary* param =[[NSDictionary alloc] initWithObjectsAndKeys:
nameString,@"phone",
[pwdString md5EncodeUpper:NO],@"passwd",nil];
[SVProgressHUD showWithStatus:@"正在登录..." maskType:SVProgressHUDMaskTypeBlack];
[[NetworkTask sharedNetworkTask] startPOSTTaskApi:API_Login
forParam:param
delegate:self
resultObj:[[LoginResult alloc] init]
customInfo:@"login"];

// 2、回调实现
#pragma mark - NetworkTaskDelegate
-(void)netResultSuccessBack:(NetResultBase *)result forInfo:(id)customInfo {
    [SVProgressHUD dismiss];
    
    if ([customInfo isEqualToString:@"login"]) {
        AppDelegate *app = [AppDelegate shareMyApplication];
        [app.mainVC switchToHomeVC];
        
        LoginResult *userInfo = (LoginResult *)result;
        [[SysDataSaver SharedSaver] saveUserInfo:userInfo];
    }
}


-(void)netResultFailBack:(NSString *)errorDesc errorCode:(NSInteger)errorCode forInfo:(id)customInfo {
    [SVProgressHUD dismiss];
    [FadePromptView showPromptStatus:errorDesc duration:1.0 finishBlock:^{
        //
    }];
}

// 实现配套的数据model类
//
//  LoginResult.h
//
//  Created by wuyj on 16/12/14.
//  Copyright © 2016年 ai.touch. All rights reserved.
//

#import "NetResultBase.h"

@interface LoginResult : NetResultBase<NSCoding>
@property (nonatomic, copy)NSString *nick;
@property (nonatomic, copy)NSString *avatar;
@property (nonatomic, copy)NSString *favorCount;
@property (nonatomic, copy)NSString *noteCount;
@property (nonatomic, copy)NSString *user_id;
@property (nonatomic, copy)NSString *phone;
@property (nonatomic, copy)NSString *mood;
@end
```

注意对于数据model对象，如果数据字段是Array类型，请使用宏定义定义，如下： 						

```js
@property(nonatomic,strong,getter=arrayLeibie)NSArray
*BaiduParserArray(leibie,NSString);
```

上述宏定义：BaiduParserArray，里面有两个参数第一参数：leibie是Array字段名称；NSString是Array的item的类型；当然也可以使用自定义的类，如下： 						

```js
@property(nonatomic,strong,getter=arrayBook)NSArray
*BaiduParserArray(books,BookItem);
```

上述Array字段名books，Array的item的类型：BookItem，定义如下： 						

```js
@interface BookItem : NSObject				
```

## 独立的分享框架

分享能力我们目前是提供一个独立的framework，目前集成了3个常用的分享平台: 微信、QQ、微博；
如果需要使用分享能力，请引入我们框架的AIShare.framework框架包。这个框架中主要实现了3大平台的分享能力，同时集成了支付能力。

（1）SharedDataModel，分享的数据类

（2）SharedManager，分享引擎类；

```js
SharedDataModel *sharedData = [[SharedDataModel alloc] init];
        sharedData.content = @"测试URL分享";
        sharedData.title = @"测试视频分享";
        sharedData.url = @"http://www.tudou.com/programs/view/_cVM3aAp270/";
        sharedData.dataType = SharedDataTypeText;
        [[SharedManager sharedManager] sharedData:sharedData finish:^(NSInteger statusCode,id resp) {
            //
        }];
```

## 数据存储、处理

（1）keychain存储类：AISave2Keychain

（2）剪贴板存储类：AIPasteboard

（3）基于系统的UserDefault存储类：SysDataSaver

（4）文件存储类：FileCache

（5）文件流操作类：FileStreamOperation

（6）图片处理类别：UIImage+Utility和UIImage+ResizeMagick

（7）颜色处理类别：UIColor+Utility

（8）字符串处理类别：NSString+Utility

（9）数据流处理类别：NSData+Crypto

（10）zip解压缩：ZipArchiveEx

## framework动态加载框架

这个框架主要是封装framework的动态加载和资源从framework的自己的bundle中加载的能力。这封装主要解决就是iOS平台实现插件机制。
主要掌握下面两个类的使用，请参考.h文件即可： 					

（1）AIPluginLoader类：framework插件加载器。

```js
//
//  AIPluginLoader.h
//  CommonProject
//
//  Created by wuyoujian on 2017/6/19.
//
//

#import <Foundation/Foundation.h>
#import "../Owns/AINavigationController.h"
#import "../Owns/AICommonDef.h"
#import "AIPluginProtocol.h"


typedef NS_ENUM(NSInteger,PluginLoadStatus) {
    
    PluginLoadStatusNone = 0,
    PluginLoadStatusUnDownload = 1,     // 没有下载
    PluginLoadStatusDownloading = 2,    // 下载中
    PluginLoadStatusDownloaded = 3,     // 下载完毕
    PluginLoadStatusException = 4,      // 加载异常
    
};

@interface AIPluginLoader : NSObject

AISINGLETON_DEF(AIPluginLoader,SharedObj);

// 获取插件的状态
- (PluginLoadStatus)pluginloadStatus:(NSString*)pluginURL;

// 下载插件
- (void)downloadPlugin:(NSString*)pluginURL;

// 加载插件
- (void)loadPluginName:(NSString*)frameworkName
                   nav:(AINavigationController*)navVC
      enterVCClassName:(NSString*)className;

// 加载插件
- (UIViewController<AIPluginProtocol> *)loadPluginName:(NSString*)frameworkName enterVCClassName:(NSString*)className;

@end
```

AIPluginTools：framework资源加载工具类

```js
//
//  AIPluginTools.h
//  AIBase
//
//  Created by wuyoujian on 2017/6/21.
//  Copyright © 2017年 ai.touch. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>
#import "../Owns/AICommonDef.h"

@interface AIPluginTools : NSObject

AISINGLETON_DEF(AIPluginTools,SharedObj);

// 从framework根目录下加载图片
- (UIImage *)roadImageWithName:(NSString *)name;

// 从framework根目录下的包名：@param bunleName 中加载图片，例如，res.bundle
- (UIImage *)roadImageWithName:(NSString *)name inResBundle:(NSString*)bundleName;

// 从矢量图加载图片
- (UIImage *)roadVectorImageWithName:(NSString *)name;

// 从framework根目录下加载其他资源
- (NSData *)roadResWithName:(NSString*)name;

// 从framework根目录下的包名：@param bunleName 中加载其他资源，例如，res.bundle
- (NSData *)roadResWithName:(NSString*)name inResBundle:(NSString *)bundleName;

// 返回资源路径
- (NSString *)resPathWithName:(NSString*)name;

// 返回资源路径
- (NSString *)resPathWithName:(NSString*)name inResBundle:(NSString *)bundleName;


@end
```