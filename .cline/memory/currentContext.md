## Explanation of a current work.

## Recent changes

1. Added `adjust` option to get specific date.
2. Added `range` option to show multiple dates in a range.

## Next step

Implement more features as needed.

## Things determined and needed to consider on the current context.

- The `range` option (`-r` or `--range`) allows displaying multiple consecutive dates.
- When using the range option, all dates are printed to the console and copied to the clipboard.
- The range option works well with other options like format (`-f`) and adjust (`-a`).
- The format_datetime function was extracted to improve code organization and reusability.
