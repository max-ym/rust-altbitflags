# alt_bitflags
If you need to create a lot of many-many-many bitfields in an intuitive way, check out some of these macro.

Note that it IS NOT a clone of "rust bitfields" crate. It is though my own alternative view of how bitfields in rust must be and I believe that in some cases it's more apropriate and easier to use (and understand). Or maybe I just didn't pay much effort to learn the original bitfields whatsoever :)

It is not C-like bitfield structure generator. It's more like a named bitmap. Still, if you need to change often single bits in integer numbers you may access them by functions that are generated with these macros.
