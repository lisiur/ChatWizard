import { createRouter, createWebHashHistory } from "vue-router";

export default createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      component: () => import("./App"),
      redirect: "/main/chat",
      children: [
        {
          path: "main",
          component: () => import("./pages/main/Index"),
          children: [
            {
              name: "chat",
              path: "chat",
              component: () => import("./pages/main/Chat"),
            },
            {
              name: "prompt",
              path: "prompt",
              component: () => import("./pages/main/Prompt"),
            },
          ],
        },
        {
          name: "setting",
          path: "setting",
          component: () => import("./pages/setting/Index"),
        },
      ],
    },
  ],
});
