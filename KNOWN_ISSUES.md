
# Extensions can no longer call sqlite3_mprintf through the extension API
We doesn't populate function pointer anymore, in `loadext.rs`
```bash
  mprintf: Some(                                                  
      crate::src::src::printf::sqlite3_mprintf
          as unsafe extern "C" fn(                                                                                                                                      
              *const ::core::ffi::c_char,
              ...
          ) -> *mut ::core::ffi::c_char,                                                                                                                                
  )
```
