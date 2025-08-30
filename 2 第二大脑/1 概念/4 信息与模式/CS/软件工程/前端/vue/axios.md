---
date created: 2024-06-23
date modified: 2025-07-10
uid: f0037a35-c08b-42eb-8df8-b3f11aa69aa8
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
