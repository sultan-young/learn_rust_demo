# struct
## 定义并实例化struct
什么是struct：
1. 是一种自定义的数据类型
2. 为相关联的值命名，打包成为有意义的组合。【类似TS种的interface】

定义struct：
1. 使用struct关键字，并未整个struct命名。
2. 在花括号内，为所有字段定义名称和类型。例如：
![](https://cdn.jsdelivr.net/gh/sultan-young/picture-bed/assets/20220922175743.png)

注意：
1. 一旦struct的实例是可变的，那么实例中所有的字段都是可变的。rust不允许单独声明实例中的某一个部分是可变的。
2. struct可以作为函数的返回值。