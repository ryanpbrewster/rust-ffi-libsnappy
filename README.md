# Rust FFI bindings for libsnappy

Following along with https://doc.rust-lang.org/nomicon/ffi.html, with minor
modifications to deal with macOS

## Building `libsnappy.a`

Built `libsnappy.a` by following the instructions at
https://github.com/google/snappy.

## Troubleshooting

If you get a linker error like
```
  = note: Undefined symbols for architecture x86_64:                                                                                             
            "vtable for __cxxabiv1::__class_type_info", referenced from:                                                                         
                typeinfo for snappy::Sink in libsnappy.a(snappy-sinksource.cc.o)                                                                 
                typeinfo for snappy::Source in libsnappy.a(snappy-sinksource.cc.o)                                                               
            NOTE: a missing vtable usually means the first non-inline virtual member function has no definition.                                 
            "vtable for __cxxabiv1::__si_class_type_info", referenced from:                                                                      
                typeinfo for snappy::ByteArraySource in libsnappy.a(snappy-sinksource.cc.o)                                                      
                typeinfo for snappy::UncheckedByteArraySink in libsnappy.a(snappy-sinksource.cc.o)                                               
            NOTE: a missing vtable usually means the first non-inline virtual member function has no definition.                                 
            "___cxa_pure_virtual", referenced from:                                                                                              
                vtable for snappy::Sink in libsnappy.a(snappy-sinksource.cc.o)                                                                   
                vtable for snappy::Source in libsnappy.a(snappy-sinksource.cc.o)                                                                 
            "___gxx_personality_v0", referenced from:                                                                                            
                snappy::Compress(snappy::Source*, snappy::Sink*) in libsnappy.a(snappy.cc.o)                                                     
                snappy::RawCompress(char const*, unsigned long, char*, unsigned long*) in libsnappy.a(snappy.cc.o)                               
                snappy::ByteArraySource::~ByteArraySource() in libsnappy.a(snappy-sinksource.cc.o)                                               
                snappy::UncheckedByteArraySink::~UncheckedByteArraySink() in libsnappy.a(snappy-sinksource.cc.o)                                 
            "operator delete(void*)", referenced from:                                                                                           
                snappy::ByteArraySource::~ByteArraySource() in libsnappy.a(snappy-sinksource.cc.o)                                               
                snappy::UncheckedByteArraySink::~UncheckedByteArraySink() in libsnappy.a(snappy-sinksource.cc.o)                                 
            "operator delete[](void*)", referenced from:                                                                                         
                snappy::Compress(snappy::Source*, snappy::Sink*) in libsnappy.a(snappy.cc.o)                                                     
                snappy::internal::WorkingMemory::~WorkingMemory() in libsnappy.a(snappy.cc.o)                                                    
            "std::terminate()", referenced from:                                                                                                 
                ___clang_call_terminate in libsnappy.a(snappy.cc.o)                                                                              
            "___cxa_begin_catch", referenced from:                                                                                               
                ___clang_call_terminate in libsnappy.a(snappy.cc.o)                                                                              
            "operator new[](unsigned long)", referenced from:                                                                                    
                snappy::internal::WorkingMemory::GetHashTable(unsigned long, int*) in libsnappy.a(snappy.cc.o)                                   
                snappy::Comp                snappy::Compress(snappy::Source*, snappy::Sink*) in libsnappy.a(snappy.cc.o)                                                     
          ld: symbol(s) not found for architecture x86_64                                                                                        
```
it's because apparently on macOS you need to explicitly link in libc++ ([ref](https://github.com/auth0/react-native-lock/issues/88)).
This is why `build.rs` has
```
    println!("cargo:rustc-link-lib=c++");
```
