---
draw:
title: axios
tags: []
date created: 2024-06-23
date modified: 2024-12-27
---

```js
>>>> request.js
import axios from 'axios'  
  
const baseURL = import.meta.env.VITE_BASE_URL  
  
const service = axios.create({  
  baseURL: baseURL,  
  timeout: 500000  
})  
  
  
service.interceptors.response.use(  
  (response) => {  
    return response.data  
  },  
  (error)=> {  
    console.log(error) // for debug  
    return Promise.reject(error)  
  }  
)  
  
  
export default service
```
