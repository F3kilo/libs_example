from ctypes import *
get_my_integer_lib = cdll.LoadLibrary("target/debug/libmy_get_int.so")
print(get_my_integer_lib.get_integer())