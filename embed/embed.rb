
require 'ffi'

module Hello
  extend FFI::Library
  ffi_lib 'target/release/libembed.dylib'
  attach_function :process, [], :void
  attach_function :demo, [:string], :void
end

# Hello.process
Hello.demo("asd")

puts 'done!'