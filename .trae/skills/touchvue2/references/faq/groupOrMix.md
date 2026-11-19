# group 和 mix 有何区别

<a href="#/example/element/group" target="_blank">group</a> 与 <a href="#/example/element/mix" target="_blank">mix</a> 元素其性质是一样的，都是组合各种表单元素的载体。

Touch UI 一开始只提供了 mix ，但由于对 IE8 兼容性需求的增加，而 mix 在 IE8 下又存在很多不尽人意的渲染表现，才产生了 group 这样一个新的界面元素，其差别主要在于

* mix 元素是使用的 flex 布局，在 IE8 下是 table 布局；group 元素使用的 float 布局。
* mix 中的子元素是顺序加载；group 中的子元素，需要按照指定的规则，先书写左右两侧的子元素，最后书写需要自适应宽度的子元素（一般是 input）。
* group 在兼容 IE8 时有良好表现。

目前 Touch UI 封装的全部组合框类型的元素（如 <a href="#/example/element/dateField" target="_blank">日期输入框</a>、<a href="#/example/element/increaseReduce" target="_blank">数值调节</a>等）均是使用的 group 元素。