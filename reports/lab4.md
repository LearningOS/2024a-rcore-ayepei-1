实现的功能：
linkat，硬链接，将链接文件的目录项的inode_id的值一样，并修改inode链接次数。
unlinkat：取消硬链接，讲对应的目录项置为空，再减小链接次数。
fstat：获取文件的inode_id,链接次数等信息。

问答题：
1.root inode起到可以访问其他所有文件的信息，它存放着其他文件的目录项，如果损坏整个文件系统就失效。
ch7：1.在父子进程可以通过pipe来传递信息。
2.可以创建一个缓冲区来通信。

# **荣誉准则**

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > *无*

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > *[[第六章：文件系统与I/O重定向 - rCore-Camp-Guide-2024A 文档](https://learningos.cn/rCore-Camp-Guide-2024A/chapter6/index.html)](https://learningos.cn/rCore-Camp-Guide-2024A/chapter5/index.html)*
   >
   > [第六章：文件系统 - rCore-Tutorial-Book-v3 3.6.0-alpha.1 文档](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter6/index.html)
   >
   > [rCore 作业讲解与答疑 - 飞书云文档](https://sjodqtoogh.feishu.cn/docx/ZoqBdmcmAoXi9yxZUkucMmxBnzg)

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。