# Bionic Reading HTML-ifier
This program will convert a **plain text** file into HTML using [bionic reading](https://bionic-reading.com) formatting.

# Usage
A `template.html` file must be present. You can either drag a file onto the executable, or provide an argument in cmd like `<executable name here.exe> <text file name.txt>`. This will create a new file called `<text file name>.bionic.html` in the same folder as the text file.

# Customization
You can edit the `template.html` file included with the executable. This does require a couple extra formatting rules, which can be seen in the packaged `template.html` file:
- Any curly braces (`{}`) present in usual HTML (style tag, strings, etc.) *must* be doubled (`{{}}`).
- ONE pair of empty, single braces must be present. No more, no less. The braces will be replaced with the bionic text output. More or less will crash the program.
- The above rules adhere to Rust's formatting rules, more specifically for the `rt-format` crate.
- Every bold segment will be put into a `<p class="bolded"></p>`. The `bolded` class can be customized, but it is highly recommended to keep `display: inline;` as every segment will cause a line break without.
- All newline characters will be replaced with `<br>`.
