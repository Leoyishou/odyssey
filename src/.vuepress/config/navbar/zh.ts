import { navbar } from "vuepress-theme-hope";

export const zhNavbar = navbar([
  // "/zh/",
  // "/zh/portfolio",
  // "/zh/demo/",
  {
    text: "文字",
    icon: "book",
    prefix: "/",
    children: [
      { text: "1 一切皆项目", icon: "https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fliuyishou%2FDesktop%2F2024%2F11%2F13%2F17-27-56-908ffa2fbd1998ffb9127aad403bb047-undraw_handcrafts_pinned_file-88e1f9.svg", link: "1 一切皆项目/" },
      { text: "2 第二大脑", icon: "fas fa-brain", link: "2 第二大脑/" },
      { text: "3 博客", icon: "fas fa-blog", link: "3 博客/" },
      { text: "4 复盘", icon: "fas fa-history", link: "4 复盘/" },
    ],
  },
  {
    text: "B站",
    icon: "book",
    link: "https://space.bilibili.com/244791824",
  }
]);
