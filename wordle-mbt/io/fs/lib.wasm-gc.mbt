fn read_file_to_string(path : @ffi.ExternString) -> @ffi.ExternString = "__moonbit_fs_unstable" "read_file_to_string"

fn write_string_to_file(path : @ffi.ExternString, content : @ffi.ExternString) = "__moonbit_fs_unstable" "write_string_to_file"

fn path_exists(path : @ffi.ExternString) -> Bool = "__moonbit_fs_unstable" "path_exists"

pub fn read_to_string(path : String) -> String {
  let content = read_file_to_string(@ffi.string_to_extern(path))
  @ffi.string_from_extern(content)
}

pub fn write_to_string(path : String, content : String) -> Unit {
  write_string_to_file(
    @ffi.string_to_extern(path),
    @ffi.string_to_extern(content),
  )
}

pub fn exists(path : String) -> Bool {
  path_exists(@ffi.string_to_extern(path))
}
