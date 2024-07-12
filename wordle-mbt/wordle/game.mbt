pub fn entry() -> Unit {
  @io.println("Hello, Moon!")
  let interactive_mode = @env.get_env_var("NO_COLOR") == None
  if interactive_mode {
    @io.println("I am in a tty. Please print colors!")
  } else {
    @io.println(
      "I am not in a tty. Please print according to test requirements!",
    )
  }
  if interactive_mode {
    @io.print("Your name:")
    @io.flush(Stdout)
    let mut name = @io.read_line_from_stdin().unwrap()
    // we have no trim now in moonbit
    if name.ends_with("\n") {
      name = name.substring(start=0, end=name.length() - 1)
    }
    @io.println("Welcome to MoonBit wordle, " + name + "!")
  }

  // example: print arguments
  @io.println("Command line arguments: ")
  @env.get_args().iter().each(fn(arg) { @io.println(arg) })
}
