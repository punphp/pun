<?php declare(strict_types=1);

namespace PhpParser\Node;

use PhpParser\NodeAbstract;

class Name extends NodeAbstract {
    /**
     * @psalm-var non-empty-string
     * @var string Name as string
     */
    public string $name;

    /** @var array<string, bool> */
    private static array $specialClassNames = [
        'self'   => true,
        'parent' => true,
        'static' => true,
    ];

    /**
     * Constructs a name node.
     *
     * @param string|string[]|self $name Name as string, part array or Name instance (copy ctor)
     * @param array<string, mixed> $attributes Additional attributes
     */
    final public function __construct($name, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    /**
     * Get parts of name (split by the namespace separator).
     *
     * @psalm-return non-empty-list<string>
     * @return string[] Parts of name
     */
    public function getParts(): array {}

    /**
     * Gets the first part of the name, i.e. everything before the first namespace separator.
     *
     * @return string First part of the name
     */
    public function getFirst(): string {}

    /**
     * Gets the last part of the name, i.e. everything after the last namespace separator.
     *
     * @return string Last part of the name
     */
    public function getLast(): string {}

    /**
     * Checks whether the name is unqualified. (E.g. Name)
     *
     * @return bool Whether the name is unqualified
     */
    public function isUnqualified(): bool {}

    /**
     * Checks whether the name is qualified. (E.g. Name\Name)
     *
     * @return bool Whether the name is qualified
     */
    public function isQualified(): bool {}

    /**
     * Checks whether the name is fully qualified. (E.g. \Name)
     *
     * @return bool Whether the name is fully qualified
     */
    public function isFullyQualified(): bool {}

    /**
     * Checks whether the name is explicitly relative to the current namespace. (E.g. namespace\Name)
     *
     * @return bool Whether the name is relative
     */
    public function isRelative(): bool {}

    /**
     * Returns a string representation of the name itself, without taking the name type into
     * account (e.g., not including a leading backslash for fully qualified names).
     *
     * @psalm-return non-empty-string
     * @return string String representation
     */
    public function toString(): string {}

    /**
     * Returns a string representation of the name as it would occur in code (e.g., including
     * leading backslash for fully qualified names.
     *
     * @psalm-return non-empty-string
     * @return string String representation
     */
    public function toCodeString(): string {}

    /**
     * Returns lowercased string representation of the name, without taking the name type into
     * account (e.g., no leading backslash for fully qualified names).
     *
     * @psalm-return non-empty-string&lowercase-string
     * @return string Lowercased string representation
     */
    public function toLowerString(): string {}

    /**
     * Checks whether the identifier is a special class name (self, parent or static).
     *
     * @return bool Whether identifier is a special class name
     */
    public function isSpecialClassName(): bool {}

    /**
     * Returns a string representation of the name by imploding the namespace parts with the
     * namespace separator.
     *
     * @psalm-return non-empty-string
     * @return string String representation
     */
    public function __toString(): string {}

    /**
     * Gets a slice of a name (similar to array_slice).
     *
     * This method returns a new instance of the same type as the original and with the same
     * attributes.
     *
     * If the slice is empty, null is returned. The null value will be correctly handled in
     * concatenations using concat().
     *
     * Offset and length have the same meaning as in array_slice().
     *
     * @param int $offset Offset to start the slice at (may be negative)
     * @param int|null $length Length of the slice (may be negative)
     *
     * @return static|null Sliced name
     */
    public function slice(int $offset, ?int $length = null) {}

    /**
     * Concatenate two names, yielding a new Name instance.
     *
     * The type of the generated instance depends on which class this method is called on, for
     * example Name\FullyQualified::concat() will yield a Name\FullyQualified instance.
     *
     * If one of the arguments is null, a new instance of the other name will be returned. If both
     * arguments are null, null will be returned. As such, writing
     *     Name::concat($namespace, $shortName)
     * where $namespace is a Name node or null will work as expected.
     *
     * @param string|string[]|self|null $name1 The first name
     * @param string|string[]|self|null $name2 The second name
     * @param array<string, mixed> $attributes Attributes to assign to concatenated name
     *
     * @return static|null Concatenated name
     */
    public static function concat($name1, $name2, array $attributes = []) {}

    /**
     * Prepares a (string, array or Name node) name for use in name changing methods by converting
     * it to a string.
     *
     * @param string|string[]|self $name Name to prepare
     *
     * @psalm-return non-empty-string
     * @return string Prepared name
     */
    private static function prepareName($name): string {}

    public function getType(): string {}
}
