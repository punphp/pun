<?php

declare(strict_types=1);

namespace GuzzleHttp\UriTemplate;

/**
 * Expands URI templates. Userland implementation of PECL uri_template.
 *
 * @see https://datatracker.ietf.org/doc/html/rfc6570
 */
final class UriTemplate
{
    private const RESERVED_OPERATORS = '=,!@|';
    private const SUPPORTED_OPERATORS = '+#./;?&';
    private const VARNAME_PATTERN = '(?:[A-Za-z0-9_]|%[0-9A-Fa-f]{2})(?:\.?(?:[A-Za-z0-9_]|%[0-9A-Fa-f]{2}))*';
    private const MAX_VARIABLE_DEPTH = 64;

    /**
     * Hash for quick operator lookups.
     *
     * @var array<string, array{prefix:string, joiner:string, query:bool, ifemp:string}>
     */
    private const OPERATOR_HASH = [
        '' => ['prefix' => '', 'joiner' => ',', 'query' => false, 'ifemp' => ''],
        '+' => ['prefix' => '', 'joiner' => ',', 'query' => false, 'ifemp' => ''],
        '#' => ['prefix' => '#', 'joiner' => ',', 'query' => false, 'ifemp' => ''],
        '.' => ['prefix' => '.', 'joiner' => '.', 'query' => false, 'ifemp' => ''],
        '/' => ['prefix' => '/', 'joiner' => '/', 'query' => false, 'ifemp' => ''],
        ';' => ['prefix' => ';', 'joiner' => ';', 'query' => true, 'ifemp' => ''],
        '?' => ['prefix' => '?', 'joiner' => '&', 'query' => true, 'ifemp' => '='],
        '&' => ['prefix' => '&', 'joiner' => '&', 'query' => true, 'ifemp' => '='],
    ];

    private function __construct()
    {}

    /**
     * @return array<array-key, mixed>
     */
    public function __serialize(): array
    {}

    /**
     * @param array<array-key, mixed> $data
     */
    public function __unserialize(array $data): void
    {}

    /**
     * @param array<array-key, mixed> $variables Variables to use in the template expansion
     *
     * @throws \InvalidArgumentException When the template syntax or referenced variable shape is invalid
     * @throws \RuntimeException
     */
    public static function expand(string $template, array $variables): string
    {}

    /**
     * Detach and form every referenced variable value before expansion.
     *
     * Spec section 3: each variable's value is formed prior to template
     * expansion. Every expression is parsed and validated first, then
     * definedness is bound and raw values are read, in template order,
     * before any __toString() method runs, so an object converted while
     * values are formed cannot change another referenced variable through
     * a PHP reference. Stringable objects are converted to strings and
     * scalars to their expansion strings, once per value position, in
     * template order, and string values and map keys are validated as
     * UTF-8 in the same pass, so a repeated variable keeps a static value
     * throughout the expansion and value errors surface in member order.
     * Undefined variables are stored as null.
     *
     * Values containing no references, floats, or stringable objects are
     * shared instead of being rebuilt: copy-on-write semantics guarantee
     * shared values cannot change, and sharing avoids materializing
     * structures whose logical size exceeds their physical size, such as
     * arrays repeating one shared subtree. Values whose shape is certain
     * to be rejected are neither shared nor scanned; their rejection is
     * captured while they are bound and thrown at their formation
     * position. Parsed expressions are not retained; expansion parses
     * each occurrence again, so templates with many distinct expressions
     * do not allocate storage proportional to their count.
     *
     * @param list<string>            $references The distinct expression text, in first-occurrence order
     * @param array<array-key, mixed> $variables
     *
     * @return array<array-key, mixed>
     */
    private static function formValues(array $references, array $variables): array
    {}

    /**
     * Capture the rejection for a value whose shape guarantees one.
     *
     * The exception is created while the value is bound, before any
     * __toString() method runs, so references held by the caller cannot
     * mutate a rejected value into an accepted shape, and it is thrown
     * later, at the variable's formation position, so error precedence
     * does not depend on how a value was classified. Engine failures
     * surfaced while walking the value are deferred the same way.
     *
     * @param array{value:string, modifier:(''|'*'|':'), position?:int} $varspec
     * @param mixed                                                     $variable
     *
     * @return \InvalidArgumentException|\RuntimeException|null
     */
    private static function captureShapeRejection(array $varspec, $variable, string $expression, string $operator): ?\Exception
    {}

    /**
     * Determine whether a value must be detached and rebuilt to be formed.
     *
     * Only PHP references can change an array's contents after it has been
     * read, and only floats and stringable objects need eager conversion,
     * so a value containing none of the three is shared as it is. Levels
     * beyond the nesting limit are not scanned; shapes that deep are
     * always rejected by validation before their contents are read.
     *
     * @param mixed $value
     */
    private static function valueNeedsForming($value, int $depth): bool
    {}

    /**
     * Determine whether shape validation is certain to reject a value.
     *
     * Mirrors the shape rules without messages, encoding checks, or user
     * code, and stops at the first guaranteed rejection, so cyclic or
     * copy-on-write shared structures that can never expand are rejected
     * by walking them instead of being rebuilt first.
     *
     * @param array{value:string, modifier:(''|'*'|':'), position?:int} $varspec
     * @param mixed                                                     $variable
     */
    private static function shapeGuaranteesRejection(array $varspec, $variable, string $operator): bool
    {}

    /**
     * @param array<array-key, mixed> $value
     */
    private static function mapShapeGuaranteesRejection(array $value, bool $allowNestedArrays, int $depth): bool
    {}

    /**
     * @param array<array-key, mixed> $value
     */
    private static function nestedQueryShapeGuaranteesRejection(array $value, int $depth): bool
    {}

    /**
     * Detach a raw variable value from the caller's variables array.
     *
     * Admissible arrays are rebuilt so that PHP references held by the
     * caller cannot change the value after it has been read, and scalars
     * are copied. Object handles are kept as they are; stringable objects
     * are resolved later, while values are formed. Array members outside a
     * nested query array, and levels deeper than the nesting limit, are
     * kept as they are: shape validation rejects them by type or depth
     * without reading their contents, and rebuilding them could
     * materialize a copy-on-write shared array graph of unbounded logical
     * size.
     *
     * @param mixed $value
     *
     * @return mixed
     */
    private static function detachValue($value, bool $membersMayNest, int $depth)
    {}

    /**
     * Validate and encode the template's literal text and collect its
     * distinct expressions in first-occurrence order.
     *
     * Only distinct expression text is collected, so a template that
     * repeats an expression many times does not allocate storage
     * proportional to the number of occurrences.
     *
     * @return array{string, list<string>}
     */
    private static function prepareTemplate(string $template): array
    {}

    private static function invalidTemplate(int $offset, string $message): \InvalidArgumentException
    {}

    /**
     * Process an expansion
     *
     * @param array{0: string, 1: string} $matches Matches met in the preg_replace_callback
     * @param array<array-key, mixed>     $values  Variable values formed before the expansion began
     *
     * @return string Returns the replacement string
     */
    private static function expandMatch(array $matches, array $values): string
    {}

    /**
     * Cast a scalar or stringable variable value to its expansion string.
     *
     * Booleans expand as "1" and "0" so that false remains distinguishable
     * from the empty string and matches the http_build_query semantics used
     * by the nested query-array extension.
     *
     * Floats are formatted with "." as the decimal separator regardless of
     * the process locale, because the plain float-to-string cast honors
     * LC_NUMERIC before PHP 8.0.
     *
     * @param mixed $value
     */
    private static function stringifyValue($value): string
    {}

    /**
     * Convert a float to its expansion string independently of the locale.
     *
     * Before PHP 8.0 the float-to-string cast honors LC_NUMERIC, so a
     * comma-decimal locale such as de_DE renders 3.5 as "3,5". Normalizing
     * the locale decimal separator back to "." keeps expansion output
     * deterministic across runtimes while preserving the precision-dependent
     * formatting of the cast.
     */
    private static function stringifyFloat(float $value): string
    {}

    /**
     * Format a named (name, value) pair.
     *
     * Spec section 3.2.1: a pair whose value is the empty string is rendered
     * as the name followed by the operator's ifemp string ("=" for the
     * form-style "?" and "&" operators, nothing for all other operators).
     */
    private static function formatPair(string $name, string $value, string $ifemp): string
    {}

    /**
     * Parse an expression into parts
     *
     * @param string $expression Expression to parse
     *
     * @return array{operator:string, values:array<array{value:string, modifier:(''|'*'|':'), position?:int}>}
     */
    private static function parseExpression(string $expression): array
    {}

    /**
     * @return array{value:string, modifier:(''|'*'|':'), position?:int}
     */
    private static function parseVarSpec(string $expression, string $varspec): array
    {}

    private static function invalidExpression(string $expression, string $message): \InvalidArgumentException
    {}

    /**
     * Determines if a referenced variable is undefined.
     *
     * Spec section 2.3: a list is undefined only when it contains zero
     * members, while a map is undefined when it contains zero members or
     * when all member names are associated with undefined values. A
     * non-empty list whose members are all null is therefore a defined
     * variable with no defined members. Spec section 3.2.1: undefined
     * variables are ignored by the expansion process, so they are skipped
     * before varspec shape validation.
     *
     * @param array<array-key, mixed> $variables
     */
    private static function isUndefinedVariable(array $variables, string $name): bool
    {}

    private static function invalidVariable(string $expression, string $path, string $message): \InvalidArgumentException
    {}

    /**
     * Escape unsafe diagnostic text before embedding it in exception messages.
     *
     * C0, DEL, and C1 controls are rendered as \xHH. If UTF-8-aware diagnostic
     * escaping fails, every byte outside printable ASCII is rendered in the
     * same form. The result is diagnostic text, not a reversible encoding.
     */
    private static function escapeDiagnosticValue(string $value): string
    {}

    private static function escapeDiagnosticBytes(string $value): string
    {}

    /**
     * Validate a variable value against a varspec without rebuilding it.
     *
     * Values that were shared instead of formed contain no references,
     * floats, or stringable objects, so validating them in place is
     * sufficient, and later occurrences of every variable re-run the
     * varspec-specific checks, such as the prefix-on-composite rule,
     * against the already formed value.
     *
     * @param array{value:string, modifier:(''|'*'|':'), position?:int} $varspec
     * @param mixed                                                     $variable
     */
    private static function assertVariableShape(array $varspec, $variable, string $expression, string $operator): void
    {}

    /**
     * @param array<array-key, mixed> $value
     */
    private static function assertListShape(string $path, array $value, string $expression): void
    {}

    /**
     * @param array<array-key, mixed> $value
     */
    private static function assertMapShape(
        string $path,
        array $value,
        string $expression,
        bool $allowNestedArrays,
        int $depth
    ): void {}

    /**
     * @param array<array-key, mixed> $value
     */
    private static function assertNestedQueryShape(string $path, array $value, string $expression, int $depth): void
    {}

    /**
     * Validate a referenced variable and return its snapshot.
     *
     * Stringable objects, including stringable members of lists and maps,
     * are converted to strings exactly once, at validation time, so a
     * repeated variable keeps a static value throughout the expansion and a
     * mutating __toString() cannot bypass validation. Later occurrences of
     * the same variable revalidate the returned snapshot against their own
     * varspec and discard the result, so varspec-specific checks such as
     * the prefix-on-composite rule run on every occurrence.
     *
     * @param array{value:string, modifier:(''|'*'|':'), position?:int} $varspec
     * @param mixed                                                     $variable
     *
     * @return mixed
     */
    private static function normalizeVariableShape(array $varspec, $variable, string $expression, string $operator)
    {}

    /**
     * @param mixed $value
     */
    private static function isScalarLike($value): bool
    {}

    /**
     * Form a null, scalar, or stringable value while values are formed.
     *
     * Stringable objects are resolved to strings once per value position
     * here, so their __toString() methods are never called again during
     * expansion, and scalars are converted to their expansion strings, so
     * repeated occurrences render identically even when a __toString()
     * method changes the float precision or the locale mid-expansion.
     * String values are validated as UTF-8 in the same pass, per spec
     * section 1.6, so all value errors surface in member order while
     * values are formed. Null is returned unchanged because it marks an
     * undefined member.
     *
     * @param mixed $value
     */
    private static function formScalarLike($value, string $expression, string $path): ?string
    {}

    /**
     * Validate a list variable and snapshot its members.
     *
     * Rebuilding the list resolves stringable members once per value
     * position and breaks PHP references held by the caller's array, so
     * members mutated after the snapshot is taken cannot change the
     * expansion.
     *
     * @param array<array-key, mixed> $value
     *
     * @return array<array-key, mixed>
     */
    private static function normalizeListShape(string $path, array $value, string $expression): array
    {}

    /**
     * Validate a map variable and snapshot its members.
     *
     * Rebuilding the map resolves stringable members once per value
     * position and breaks PHP references held by the caller's array, so
     * members mutated after the snapshot is taken cannot change the
     * expansion.
     *
     * @param array<array-key, mixed> $value
     *
     * @return array<array-key, mixed>
     */
    private static function normalizeMapShape(
        string $path,
        array $value,
        string $expression,
        bool $allowNestedArrays,
        int $depth
    ): array {}

    /**
     * Validate a nested query array and snapshot its members.
     *
     * Nested query arrays accept only scalar leaves, so there are no
     * stringable objects to resolve, but rebuilding the array breaks PHP
     * references held by the caller's array, so members mutated after the
     * snapshot is taken cannot change the expansion.
     *
     * @param array<array-key, mixed> $value
     *
     * @return array<array-key, mixed>
     */
    private static function normalizeNestedQueryShape(string $path, array $value, string $expression, int $depth): array
    {}

    /**
     * Determines if an array should be expanded as a map.
     *
     * An array is a list only when its keys are exactly 0 through n-1 in
     * ascending insertion order. The keys are compared one by one so the
     * check stops at the first mismatch and never materializes key arrays
     * proportional to the member count.
     *
     * @param array<array-key, mixed> $array
     */
    private static function isAssoc(array $array): bool
    {}

    /**
     * Select a prefix by Unicode code points and pct-encoded characters.
     */
    private static function prefixValue(string $value, int $length): string
    {}

    private static function prefixCharacterByteLength(string $value, int $offset, int $valueLength): int
    {}

    private static function utf8SequenceByteLength(int $lead): int
    {}

    private static function isSingleUtf8CodePoint(string $candidate): bool
    {}

    private static function encodeLiteralSegment(string $literal, int $baseOffset): string
    {}

    /**
     * Count the bytes of the longest well-formed UTF-8 prefix.
     *
     * Mirrors the UTF8-char grammar of RFC 3629 section 4 byte by byte so
     * the reported template offset identifies the first invalid byte rather
     * than the start of the enclosing literal segment.
     */
    private static function validUtf8PrefixLength(string $value): int
    {}

    private static function isAllowedAsciiLiteral(string $char): bool
    {}

    private static function isAllowedUnicodeLiteral(string $char): bool
    {}

    private static function encodeValue(string $value, bool $allowReserved, string $expression, string $name): string
    {}

    private static function assertValidVariableUtf8(string $value, string $expression, string $name): void
    {}
}
