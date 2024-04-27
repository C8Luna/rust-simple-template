# Simple Rust Templating
Normally we need to pull in a templating library rather than simple composition of the output from the components. Usually the template will be in a separate file, literal strings or mixed like JSX. Ignoring the likes of JSX, the advantage of a file based solution is you can use normal editors that dont have embedded language lsp options

include_str! is not normally meant to be used in this way but works amazingly well without having to create a proc_macro. You can then use macro_rules to simplify the rendering of the output. You could even use include!.

This is very fast and even for a few cases you can do this when ultimate performance is needed and using a proper templating engine. Maybe you have a simple use case like email generation so this could be perfect.

Hope it helps show just another way to minimize dependencies when you don't need anything too complex
