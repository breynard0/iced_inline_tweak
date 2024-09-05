## Example for how to use `iced` and `inline_tweak`
A simple integration for how to use the `inline_tweak` crate with `iced`. A few things to note:
- You may need to specify `std::column` when using the `tweak!` macro
- Hot-reloading enums is not currently supported
- Since Iced is a retained mode GUI library, it is necessary to interact with the window to refresh the view

  
https://github.com/user-attachments/assets/03124f44-4248-4e3a-8917-ed77357f4046
