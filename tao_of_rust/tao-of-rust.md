静态类型、动态类型
强类型、弱类型

胖指针（fat pointer）：包含了动态大小类型地址信息和携带了长度信息的指针，如字符串切片、数组切片。

零大小类型（Zero Sized Type, ZST）：比如单元类型、单元结构体，由单元类型组成的数组。
技巧：可以用单元类型来查看数据类型。
底类型（Bottom Type）：用叹号表示。也叫Bang type。

turbofish操作符

trait是Rust的灵魂。Rust中所有的抽象,比如接口抽象、OOP范式抽象、函数式范式抽象等，均基于trait来完成。同时，trait也保证了这些抽象几乎都是运行时零开销的。trait有4种用法：接口抽象、泛型约束、抽象类型和标签trait。

孤儿规则（Orphan Rule）

trait继承：用来扩展功能
