<?php declare(strict_types=1);

namespace PhpParser;

/**
 * A PHP version, representing only the major and minor version components.
 */
class PhpVersion {
    /** @var int Version ID in PHP_VERSION_ID format */
    public int $id;

    /** @var int[] Minimum versions for builtin types */
    private const BUILTIN_TYPE_VERSIONS = [
        'array'    => 50100,
        'callable' => 50400,
        'bool'     => 70000,
        'int'      => 70000,
        'float'    => 70000,
        'string'   => 70000,
        'iterable' => 70100,
        'void'     => 70100,
        'object'   => 70200,
        'null'     => 80000,
        'false'    => 80000,
        'mixed'    => 80000,
        'never'    => 80100,
        'true'     => 80200,
    ];

    private function __construct(int $id) {}

    /**
     * Create a PhpVersion object from major and minor version components.
     */
    public static function fromComponents(int $major, int $minor): self {}

    /**
     * Get the newest PHP version supported by this library. Support for this version may be partial,
     * if it is still under development.
     */
    public static function getNewestSupported(): self {}

    /**
     * Get the host PHP version, that is the PHP version we're currently running on.
     */
    public static function getHostVersion(): self {}

    /**
     * Parse the version from a string like "8.1".
     */
    public static function fromString(string $version): self {}

    /**
     * Check whether two versions are the same.
     */
    public function equals(PhpVersion $other): bool {}

    /**
     * Check whether this version is greater than or equal to the argument.
     */
    public function newerOrEqual(PhpVersion $other): bool {}

    /**
     * Check whether this version is older than the argument.
     */
    public function older(PhpVersion $other): bool {}

    /**
     * Check whether this is the host PHP version.
     */
    public function isHostVersion(): bool {}

    /**
     * Check whether this PHP version supports the given builtin type. Type name must be lowercase.
     */
    public function supportsBuiltinType(string $type): bool {}

    /**
     * Whether this version supports [] array literals.
     */
    public function supportsShortArraySyntax(): bool {}

    /**
     * Whether this version supports [] for destructuring.
     */
    public function supportsShortArrayDestructuring(): bool {}

    /**
     * Whether this version supports flexible heredoc/nowdoc.
     */
    public function supportsFlexibleHeredoc(): bool {}

    /**
     * Whether this version supports trailing commas in parameter lists.
     */
    public function supportsTrailingCommaInParamList(): bool {}

    /**
     * Whether this version allows "$var =& new Obj".
     */
    public function allowsAssignNewByReference(): bool {}

    /**
     * Whether this version allows invalid octals like "08".
     */
    public function allowsInvalidOctals(): bool {}

    /**
     * Whether this version allows DEL (\x7f) to occur in identifiers.
     */
    public function allowsDelInIdentifiers(): bool {}

    /**
     * Whether this version supports yield in expression context without parentheses.
     */
    public function supportsYieldWithoutParentheses(): bool {}

    /**
     * Whether this version supports unicode escape sequences in strings.
     */
    public function supportsUnicodeEscapes(): bool {}

    /*
     * Whether this version supports attributes.
     */
    public function supportsAttributes(): bool {}

    public function supportsNewDereferenceWithoutParentheses(): bool {}
}
