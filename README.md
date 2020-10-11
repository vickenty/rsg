# RSG

Query rust source code using [Xpath][xpath] expressions

## Installation

    cargo install --git https://github.com/vickenty/rsg --branch main

## Usage

    rsg <xpath> [path]...

When no paths are given, reads from stdin. Path may be file or a directory.

Will print each matched item per line, serialized as a token stream.

## Examples

Find all function signatures that accept a trait object:

```
$ rsg '//Signature[.//FnArg//TypeTraitObject]' src
src/main.rs: fn match_reader (r : & mut dyn Read , xpath : & str , name : & str) -> Result < () , Error >
src/lib.rs: fn add < F > (& mut self , name : & 'static str , item : Option < & 'ast dyn ToTokens > , f : F) where F : FnOnce (& mut Self) ,
```

Find all unwrap calls:

```
$ rsg '//ExprMethodCall[Ident="unwrap"]' src
src/lib.rs: el . attribute_value (ID_ATTR) . unwrap () . parse () . unwrap ()
src/lib.rs: v . root . unwrap ()
src/lib.rs: el . attribute_value (ID_ATTR) . unwrap ()
src/main.rs: types_builder . build () . unwrap ()
src/main.rs: paths . next () . unwrap ()
src/main.rs: matches . value_of ("expr") . unwrap ()
src/main.rs: match_reader (& mut std :: io :: stdin () . lock () , xpath , "<stdin>") . unwrap ()
```

## How it works

`rsg` builds an XML document, where each of `syn` types is represented with an
element of the same name. Identifiers have text content equal to the name of
identifier.

[xpath]: https://developer.mozilla.org/en-US/docs/Web/XPath

## Limitations

- Does not show location of found items.
- Only supports core Xpath 1.0.
- Field names of `syn` structs are not represented in the XML document.

## Contributing

Bug reports and merge requests are welcome.
