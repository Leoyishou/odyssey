---
draw:
tags: []
title: CS144 Computer Network
date created: 2024-09-21
date modified: 2025-01-15
---

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F23%2F11-51-40-d83ae1e55a6a6ef17e6e6b169e730006-202409231151421-2ce186.png)

## 环境部署

Ubuntu 18.04 [BYO Linux installation (stanford.edu)](https://stanford.edu/class/cs144/vm_howto/vm-howto-byo.html)  
[CS144Winter2024--Chapter0--wsl2+docker环境搭建_cs144环境配置-CSDN博客](https://blog.csdn.net/weixin_73074012/article/details/135847082)

[2021 CS144 实验笔记 | 沧海月明 (inlighting.org)](https://www.inlighting.org/archives/2021-cs144-notes)

用 Clion 里的 remote 开发

## 参考资料

官网  
https://www.scs.stanford.edu/10au-cs144/  
小林 coding  
https://xiaolincoding.com/network/  
lab 思路  
https://www.cnblogs.com/kangyupl/p/stanford_cs144_labs.html

[CS144 Lab0翻译 | Doraemonzzz](https://doraemonzzz.com/2022/01/30/2022-1-30-CS144-Lab0%E7%BF%BB%E8%AF%91/)  
[CS 144: Introduction to Computer Networking](https://cs144.github.io/)  
[CS144计算机网络 Lab0 | Kiprey's Blog](https://kiprey.github.io/2021/11/cs144-lab0/)

[【计算机网络】Stanford CS144 Lab Assignments 学习笔记 - 康宇PL - 博客园 (cnblogs.com)](https://www.cnblogs.com/kangyupl/p/stanford_cs144_labs.html)

- Toolchain (工具链):
    - 想象你有一个工具箱，里面有各种工具来建造房子。
    - 在编程中，Toolchain 就是你的"编程工具箱"。
    - 它包括：
        - 编译器：把你写的代码转换成计算机能理解的语言（就像把你的设计图转换成实际的房子）
        - 调试器：帮你找出代码中的错误（就像检查房子是否有问题）
        - 其他工具：帮助你构建和运行程序的各种工具
    - 在 Java 中，你可能用过 JDK，它就是一种 Toolchain。
- CMake:
    - 想象你是一个建筑经理，负责管理整个建房子的过程。
    - CMake 就像是你的"建筑计划"。
    - 它告诉计算机：
        - 你的代码文件在哪里
        - 如何使用 Toolchain 中的工具来编译这些代码
        - 如何把所有编译好的部分组合成一个完整的程序
    - 在 Java 中，你可能用过 Maven 或 Gradle，CMake 在 C++ 中的作用类似。

## 核心对象

stream reassembler 将**带索引的字节流碎片**重组成有序的字节流

## Lab

| Lab                               | Tasks                                                                        | 认知                           |
| --------------------------------- | ---------------------------------------------------------------------------- | ---------------------------- |
| [[Lab0 Networking Warmup]]        | • 实现字节流(ByteStream)<br>• 熟悉环境设置和基础工具<br>• 主要是热身练习，为后续实验打基础                   | ByteStream的本质是一个 char 的deque |
| [[数据流重组 Lab1 Stream Reassembler]] | • 实现TCP的流重组器(StreamReassembler)<br>• 处理乱序到达的数据并重组为有序字节流<br>• 模拟TCP如何处理失序的数据包 |                              |
| [[Lab2 TCP Receiver]]             | • 实现TCP接收端<br>• 处理序列号(Sequence Numbers)<br>• 管理接收窗口(Receive Window)          |                              |
| [[Lab3 TCP Sender]]               | • 实现TCP发送端<br>• 实现重传机制<br>• 实现流量控制                                           |                              |
| [[Lab4 TCP Connection]]           | • 将之前的Sender和Receiver组合<br>• 实现完整的TCP连接<br>• 处理连接的建立和终止                      |                              |
| [[Lab5 Network Interface]]        | • 实现网络接口层<br>• ARP (Address Resolution Protocol)协议<br>• 以太网帧的处理              |                              |
| [[Lab6 Router]]                   | • 实现路由器的核心功能<br>• 路由表的查找和维护<br>• IP数据包的转发                                    |                              |

## 编译

```shell
# 1. 首先，回到项目根目录
cd /Users/liuyishou/usr/projects/CS144-Computer-Network

# 2. 创建（如果不存在）并进入 build 目录
mkdir -p build
cd build

# 3. 运行 cmake
cmake ..

# 4. 编译项目
make

# 5. 运行 webget
./apps/webget stanford.edu /class/cs144
```

- (a) `make format`(使编码风格正常化)
- (b) `make`(确保代码可以编译)
- (c) `make check_lab0`(确保自动测试通过)

## How to Excel in CS144: A Comprehensive Guide

This guide is your roadmap to success in CS144, Introduction to Computer Networking. This challenging course explores how computer networks function, covering everything from the fundamentals of packet switching and layering to the intricacies of internet protocols and applications like the World Wide Web, video streaming services (Netflix, Hulu), video conferencing (Zoom, Skype), and BitTorrent1. We'll equip you with the knowledge and strategies to not only understand the core concepts but also excel in the challenging labs.

### Course Website and Essential Resources

Your journey begins with the CS144 course website. This central hub provides crucial information, including lecture schedules, assignments, and announcements2. Here's what you'll typically find:

- Syllabus: This outlines the course's learning objectives, grading criteria, and important deadlines2.
    
- Lecture Notes and Slides: These supplement in-class lectures, allowing you to review concepts at your own pace and delve deeper into specific topics3.
    
- Lab Assignments: Detailed instructions and guidelines for the hands-on lab component of the course, where you'll gain practical experience4.
    
- Discussion Forums: Connect with your classmates, ask questions, and engage in collaborative learning2.
    

Beyond the course website, explore these valuable external resources:

- Online Course Videos: Platforms like YouTube and Coursera offer educational videos on computer networking, providing visual explanations and real-world examples5. You can find videos covering specific protocols, network architectures, and even walkthroughs of similar lab assignments.
    
- GitHub Repositories: Many students and instructors share CS144 projects, lab solutions, and study materials on GitHub6. These repositories can offer valuable insights and inspiration for your own work, but remember to use them ethically and avoid plagiarism.
    
- Textbooks: While not always required, textbooks can be comprehensive references, offering in-depth explanations of networking concepts2. They can be particularly helpful when you need more detailed explanations or alternative perspectives on the material.
    

To deepen your understanding of computer networking, consider exploring the history of the internet5. Understanding how the internet evolved can provide valuable context and a deeper appreciation for the concepts you'll be learning. Additionally, learning about network attacks and security vulnerabilities can highlight the importance of security in network design and implementation5. Finally, familiarize yourself with the Dynamic Host Configuration Protocol (DHCP), a crucial mechanism for how devices obtain IP addresses in a network5.

### Mastering the Labs: A Step-by-Step Approach

The labs in CS144 are where you'll truly put your knowledge to the test. You'll gain hands-on experience with core networking concepts by implementing network protocols, building network applications, and analyzing network traffic. Here's a strategic approach to excel in these labs:

1. Start Early: Don't procrastinate! Lab assignments in CS144 can be complex and time-consuming. Starting early allows you to understand the requirements, plan your approach, and have ample time for debugging.
    
2. Break Down the Problem: Divide the lab assignment into smaller, manageable tasks. This approach makes the problem less daunting and allows you to focus on one step at a time. For example, if you're implementing a routing protocol, you might break it down into tasks like parsing routing tables, calculating shortest paths, and forwarding packets.
    
3. Understand the Concepts: Before writing any code, ensure you have a solid grasp of the underlying networking concepts. Review lecture notes, consult textbooks, and seek clarification from the instructor or teaching assistants if needed. This foundational knowledge is crucial for successful implementation.
    
4. Write Clean Code: Follow coding best practices, such as using meaningful variable names, writing comments, and organizing your code logically. This makes your code easier to understand, debug, and maintain. It also demonstrates your understanding of the concepts to the graders.
    
5. Test Thoroughly: Develop a comprehensive testing strategy to ensure your code works correctly under various conditions. Use the provided test cases and create your own to cover edge cases and potential errors. Thorough testing can save you time and frustration in the long run.
    
6. Seek Help When Needed: Don't hesitate to ask for help if you encounter difficulties. Utilize the discussion forums, attend office hours, and collaborate with classmates (while adhering to the course's collaboration policy). Remember that everyone struggles at some point, and seeking help is a sign of strength, not weakness.
    

A key insight to keep in mind is that the CS144 labs are designed to guide you through the process of building a complete internet infrastructure step-by-step6. Each lab builds upon the previous ones, culminating in a comprehensive understanding of how different network components interact.

To succeed in the labs, you'll need to be comfortable with C++ and debugging tools1. CS144 is a demanding course that requires strong programming skills. If you're struggling with C++, consider reviewing the basics or seeking help from the TAs.

It's also important to remember that the lab assignments are not just specifications to be followed blindly7. They require critical thinking and problem-solving skills. You'll need to understand the concepts behind the code and apply them to different scenarios.

When working on the labs, pay close attention to the different types of sequence numbers used in TCP8. These sequence numbers are crucial for implementing TCP correctly and ensuring reliable data transmission. You'll also need to familiarize yourself with the tools used in the labs, such as the Minnow library4. These tools provide a framework for your implementations and simplify the development process.

Understanding the concept of "bytes in flight" is essential for grasping TCP flow control and congestion control9. This concept refers to the amount of data that has been sent but not yet acknowledged. Managing bytes in flight is crucial for preventing network congestion and ensuring efficient data transfer.

Finally, pay attention to the endianness of data in network programming10. Endianness refers to the order in which bytes are stored in memory. Different computer architectures may use different endianness, and it's crucial to handle this correctly when sending and receiving data over the network.

### Seeking Support and Collaboration

CS144 can be challenging, and it's crucial to leverage the available support systems. Actively participate in discussion forums or online communities to connect with fellow students, ask questions, and share insights2. These forums can be valuable resources for getting help with specific problems, understanding difficult concepts, and staying motivated.

Engage with your teaching assistants (TAs) during office hours or lab sessions to seek clarification on concepts, debugging assistance, or feedback on your progress. TAs are there to help you succeed, so don't hesitate to reach out to them.

Collaboration with peers can be beneficial, but always ensure you adhere to the course's collaboration policy and maintain academic integrity1. Discuss concepts and approaches with your classmates, but avoid sharing code or copying solutions.

### Time Management and Study Strategies

Effective time management and study strategies are essential for success in CS144. Here are some tips to help you stay on track:

- Stay Organized: Keep track of deadlines, lecture notes, and lab assignments. Use a planner, calendar, or online tools to manage your time effectively.
    
- Plan Ahead: Create a study schedule that allocates sufficient time for lectures, labs, and self-study. Break down large tasks into smaller, more manageable chunks.
    
- Prioritize Tasks: Identify the most important tasks and focus on completing them first. Don't get bogged down in less important details.
    
- Take Breaks: Regular breaks can help you stay focused and avoid burnout. Step away from your computer and engage in activities that help you relax and recharge.
    

### Tips from Past Students

Learning from those who have successfully navigated CS144 can provide valuable insights and strategies. Here are some tips gathered from online forums and student communities: 1

- Don't fall behind: CS144 covers a lot of material, and it's easy to fall behind if you're not careful. Attend all lectures, stay on top of readings, and start lab assignments early.
    
- Practice, practice, practice: The best way to learn networking concepts is to apply them. Work through practice problems, review past exams, and participate in online quizzes1.
    
- Don't be afraid to ask for help: Everyone struggles at some point in CS144. Don't hesitate to ask for help from your TAs, instructors, or classmates.
    
- Form study groups: Collaborating with classmates can be a great way to learn the material and stay motivated.
    
- Find what works for you: Experiment with different study techniques and find what works best for you. Some students prefer to study alone, while others prefer to study in groups. Some students prefer to take notes by hand, while others prefer to type them.
    
- CS144 can be demanding, but persistence is key. If you encounter setbacks, don't get discouraged. Seek help, review the material, and keep practicing.
    

### Conclusion

By following this comprehensive guide, actively engaging with the course material, and utilizing the available resources, you can not only succeed in CS144 but also develop a deep understanding of computer networking principles. Remember to start early, break down the labs, seek support when needed, and learn from the experiences of others. With dedication and perseverance, you can confidently navigate this challenging course and emerge with a solid foundation in computer networking.

Now that you're equipped with this knowledge, take the next step! Connect with online communities, explore the recommended resources, and embark on your journey to mastering computer networking.

#### Works cited

1. Stuff you need to know about CS144 (Winter 2024), accessed December 30, 2024, [https://cs144.github.io/logistics.pdf](https://cs144.github.io/logistics.pdf)
2. CS144: Web Applications–Spring 2021 - UCLA, accessed December 30, 2024, [http://oak.cs.ucla.edu/classes/cs144/](http://oak.cs.ucla.edu/classes/cs144/)
3. CS144 Lecture notes, accessed December 30, 2024, [https://www.scs.stanford.edu/10au-cs144/notes/](https://www.scs.stanford.edu/10au-cs144/notes/)
4. Lab Checkpoint 1: stitching substrings into a byte stream - CS 144, accessed December 30, 2024, [https://cs144.github.io/assignments/check1.pdf](https://cs144.github.io/assignments/check1.pdf)
5. wnz27/computer-networking-learn-cs144: Lecture Slides for Philip Levis and Nick McKeown's "Introduction to Computer Networking" Stanford course - GitHub, accessed December 30, 2024, [https://github.com/wnz27/computer-networking-learn-cs144](https://github.com/wnz27/computer-networking-learn-cs144)
6. Learning materials for Stanford Computer Network course: CS144 - GitHub, accessed December 30, 2024, [https://github.com/PKUFlyingPig/CS144-Computer-Network](https://github.com/PKUFlyingPig/CS144-Computer-Network)
7. Lab 0: networking warmup - GitHub Pages, accessed December 30, 2024, [https://vixbob.github.io/cs144-web-page/assignments/lab0.pdf](https://vixbob.github.io/cs144-web-page/assignments/lab0.pdf)
8. 【计算机网络】Stanford CS144 Lab Assignments 学习笔记- 康宇PL - 博客园, accessed December 30, 2024, [https://www.cnblogs.com/kangyupl/p/stanford_cs144_labs.html](https://www.cnblogs.com/kangyupl/p/stanford_cs144_labs.html)
9. CS144 Intro to Computer Networks Final Exam–Thursday, December 9, 2021, accessed December 30, 2024, [https://cs144.github.io/exams/21fa-final-ans.pdf](https://cs144.github.io/exams/21fa-final-ans.pdf)
10. K1ASER/CS144-Router: Repository for Lab 3 & 5 for CS144 - GitHub, accessed December 30, 2024, [https://github.com/K1ASER/CS144-Router](https://github.com/K1ASER/CS144-Router)

**
