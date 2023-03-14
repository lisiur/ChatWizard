import {createRouter, createWebHashHistory} from 'vue-router'

export default createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: '/',
            component: () => import('./App'),
            children: [
                {
                    path: '',
                    component: () => import("./pages/Main")
                }
            ],
        }
    ]
})