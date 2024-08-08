## API specific weirdness:

- Years are not displayed
  - not easy to display data from multiple years on one request
  - a prev/curr check would have to be done on every single line, if the year
    changed, and update a counter
- The months are 0 indexed!

## Inherent CSV problems:

The issue with CSV is not being able to escape delimiters in the CSV syntax.

Delimiter for fields like comma (,) can exist in normal text. The only way to
escape it is to have it within string delimiters like (") or ('), but even those
exist within normal text, especially in English. And not even string blocks can
escape new lines (\n).

CSV should NEVER be used for data transfer! Especially not for strings, but even
numbers can have different decimal separators (,) or (.) that again interfere
with the encoding syntax. And excel should be very strongly avoided at any step
of the pipeline.

One would have to decide it's own CSV standard based on the use case. For
example setting very unusual characters for both string and field delimiting.
And even version control it, if the CSV is being stored anywhere. To keep
backwards compatibility to previously stored versions in case these delimiters
need to change. It would be a nightmare and CSV should **NEVER** be used for
export. Because it simply causes data corruption.
