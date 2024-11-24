import { defineUserConfig } from "vuepress";
import theme from "./theme.js";

export default defineUserConfig({
  base: "/",

  head: [
    [
      "link",
      {
        rel: "icon",
        href: "https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fliuyishou%2Ftmp%2F2024%2F11%2F13%2F17-40-24-6b6dc2702b1c6102be57b82fdf86132a-undraw_handcrafts_planet-5a8cbd.svg"
      }
    ],
    [
      "link",
      {
        rel: "apple-touch-icon",
        sizes: "180x180",
        href: "https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fliuyishou%2Ftmp%2F2024%2F11%2F13%2F17-40-24-6b6dc2702b1c6102be57b82fdf86132a-undraw_handcrafts_planet-5a8cbd.svg"
      }
    ],
    [
      "link",
      {
        rel: "icon",
        type: "image/png",
        sizes: "32x32",
        href: "https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fliuyishou%2Ftmp%2F2024%2F11%2F13%2F17-40-24-6b6dc2702b1c6102be57b82fdf86132a-undraw_handcrafts_planet-5a8cbd.svg"
      }
    ],
    [
      "link",
      {
        rel: "icon",
        type: "image/png",
        sizes: "16x16",
        href: "https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fliuyishou%2Ftmp%2F2024%2F11%2F13%2F17-40-24-6b6dc2702b1c6102be57b82fdf86132a-undraw_handcrafts_planet-5a8cbd.svg"
      }
    ]
  ],

  locales: {
    "/": {
      lang: "zh-CN",
      title: "转了码的刘公子",
      description: "批评不自由，则赞美无意义",
    },
  },

  theme,
});
