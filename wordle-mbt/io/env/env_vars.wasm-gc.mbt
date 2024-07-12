fn env_get_var(s : @ffi.ExternString) -> @ffi.ExternString = "__moonbit_fs_unstable" "env_get_var"

fn args_get() -> @ffi.JSArray = "__moonbit_fs_unstable" "args_get"

pub fn get_env_var(name : String) -> String? {
  let res = env_get_var(@ffi.string_to_extern(name))
  let mbt_string = @ffi.string_from_extern(res)
  if mbt_string == "" {
    None
  } else {
    Some(mbt_string)
  }
}

pub fn get_args() -> Array[String] {
  let arr = args_get()
  let len = @ffi.array_len(arr)
  let res = []
  for i = 0; i < len; i = i + 1 {
    let val = arr[i]
    if @ffi.jsvalue_is_string(val).not() {
      abort("Expected all strings in array")
    }
    res.push(@ffi.string_from_extern(@ffi.jsvalue_get_string(val)))
  }
  res
}
