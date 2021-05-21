[![Layered NLP](https://raw.githubusercontent.com/storyscript/layered-nlp/main/assets/layered-nlp.svg)](https://github.com/storyscript/layered-nlp)

Incrementally build up recognizers over an abstract token that combine to create _multiple_ possible interpretations.

Key features:

- Abstract over token type to support "rich" tokens like we have at Story.ai.
- May generate multiple interpretations of the same token span.
- Produces a set of ranges over the input token list with different attributes, for example:

### Layering

The key idea here is to enable starting from a bunch of vague tags and slowly building meaning up through incrementally adding information that builds on itself.

Simplification: `Money = '$' + Number`

```
    $   123   .     00
                    ╰Natural
              ╰Punct
        ╰Natural
        ╰Amt(Decimal)╯
    ╰Money($/£, Num)─╯
```

Simplification:

- `Location(NYC) = 'New' + 'York' + 'City'`
- `Location(AMS) = 'Amsterdam'`
- `Address(Person, Location) = Person + Verb('live') + Predicate('in') + Location`

```
    I     live      in      New York City
                                     ╰Noun
                                ╰Noun
                            ╰Adj
                    ╰Predicate
          ╰Verb
    ╰Noun
    ╰Person(Self)
                            ╰──Location─╯
    ╰────Address(Person, Location)─────╯
```

[![MIT licensed][mit-badge]][mit-url]
[![APACHE licensed][apache-2-badge]][apache-2-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE-MIT
[apache-2-badge]: https://img.shields.io/badge/license-APACHE%202.0-blue.svg
[apache-2-url]: LICENSE-APACHE
