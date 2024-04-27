Generate a checksum of files from a given directory at compile-time. This crate was made for the purpose of validating inter-process communication.
Various hashing algorithms are available as backends, accessible via these feature flags. If multiple features are enabled, this is the order of priority.
```
blake2      
ascon-hash  
belt-hash
fsb         
gost94      
groestl     
jh          
md2         
md4         
md5        
ripemd      
skein       
sm3         
streebog    
tiger       
whirlpool
sha1        
sha1-checked
sha2        
sha3 [default]
```   
256 variants are used when relevant, ie, `sha3::Sha3_256`.

