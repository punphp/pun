<?php declare(strict_types=1);
/*
 * This file is part of sebastian/exporter.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Exporter;

use const COUNT_RECURSIVE;
use function assert;
use function bin2hex;
use function count;
use function get_resource_type;
use function gettype;
use function implode;
use function ini_get;
use function ini_set;
use function is_array;
use function is_bool;
use function is_float;
use function is_infinite;
use function is_nan;
use function is_object;
use function is_resource;
use function is_string;
use function mb_strlen;
use function mb_substr;
use function ord;
use function preg_match;
use function preg_match_all;
use function preg_replace_callback;
use function spl_object_id;
use function sprintf;
use function str_contains;
use function str_repeat;
use function str_replace;
use function strlen;
use function strpbrk;
use function strtr;
use function var_export;
use BackedEnum;
use Google\Protobuf\Internal\Message;
use ReflectionClass;
use ReflectionObject;
use SebastianBergmann\RecursionContext\Context as RecursionContext;
use SplObjectStorage;
use stdClass;
use UnitEnum;

final readonly class Exporter
{
    /**
     * @var non-negative-int
     */
    private int $shortenArraysLongerThan;

    /**
     * @var positive-int
     */
    private int $maxLengthForStrings;
    private ?ObjectExporter $objectExporter;

    /**
     * @param non-negative-int $shortenArraysLongerThan
     * @param positive-int     $maxLengthForStrings
     */
    public function __construct(int $shortenArraysLongerThan = 0, int $maxLengthForStrings = 40, ?ObjectExporter $objectExporter = null)
    {}

    /**
     * Exports a value as a string.
     *
     * The output of this method is similar to the output of print_r(), but
     * improved in various aspects:
     *
     *  - NULL is rendered as "null" (instead of "")
     *  - TRUE is rendered as "true" (instead of "1")
     *  - FALSE is rendered as "false" (instead of "")
     *  - Strings are always quoted with single quotes
     *  - Carriage returns and newlines are normalized to \n
     *  - Recursion and repeated rendering is treated properly
     *
     * An implementation of ObjectExporter must pass the ExportContext it is
     * given to this method when it exports values that are nested in the
     * object it handles.
     *
     * @throws ObjectNotSupportedException
     */
    public function export(mixed $value, int $indentation = 0, ?ExportContext $context = null): string
    {}

    /**
     * @param array<mixed> $data
     * @param positive-int $maxLengthForStrings
     *
     * @throws ObjectNotSupportedException
     */
    public function shortenedRecursiveExport(array &$data, int $maxLengthForStrings = 40, ?RecursionContext $processed = null): string
    {}

    /**
     * Exports a value into a single-line string.
     *
     * The output of this method is similar to the output of
     * SebastianBergmann\Exporter\Exporter::export().
     *
     * Newlines are replaced by the visible string '\n'.
     * Contents of arrays and objects (if any) are replaced by '...'.
     *
     * The representation a custom object exporter provides for an object is
     * used, but it is collapsed to a single line and shortened when it is
     * longer than $maxLengthForStrings.
     *
     * @param positive-int $maxLengthForStrings
     *
     * @throws ObjectNotSupportedException
     */
    public function shortenedExport(mixed $value, int $maxLengthForStrings = 40): string
    {}

    /**
     * Returns whether a custom object exporter provides the representation
     * for an object.
     *
     * This is intended for code that renders objects itself and wants to use
     * the representation a custom object exporter provides when there is one.
     *
     * Bear in mind that an object that is (indirectly) nested in itself is
     * replaced with a reference to the object instead of being exported by a
     * custom object exporter again.
     */
    public function hasCustomRepresentationFor(object $object): bool
    {}

    /**
     * Converts an object to an array containing all of its private, protected
     * and public properties.
     *
     * @return array<mixed>
     */
    public function toArray(mixed $value): array
    {}

    public function countProperties(object $value): int
    {}

    /**
     * Returns, as keys of the returned array, the names of properties that
     * are declared more than once in the inheritance chain of an object.
     *
     * This can only happen when a derived class redeclares a private property
     * that one of its parent classes also declares.
     *
     * @param array<array-key, mixed> $properties
     *
     * @return array<string, true>
     */
    private function shadowedPropertyNames(array $properties): array
    {}

    /**
     * @param array<mixed> $data
     * @param positive-int $maxLengthForStrings
     *
     * @throws ObjectNotSupportedException
     */
    private function shortenedCountedRecursiveExport(array &$data, RecursionContext $processed, int &$counter, int $maxLengthForStrings): string
    {}

    /**
     * @throws ObjectNotSupportedException
     */
    private function recursiveExport(mixed &$value, int $indentation = 0, ?ExportContext $context = null): string
    {}

    /**
     * Collapses a representation to a single line and shortens it when it is
     * longer than $maxLengthForStrings.
     *
     * @param positive-int $maxLengthForStrings
     */
    private function shorten(string $string, int $maxLengthForStrings): string
    {}

    private function exportFloat(float $value): string
    {}

    private function exportString(string $value): string
    {}

    /**
     * @param array<mixed> $value
     *
     * @throws ObjectNotSupportedException
     */
    private function exportArray(array &$value, ExportContext $context, int $indentation): string
    {}

    /**
     * @throws ObjectNotSupportedException
     */
    private function exportObject(object $value, ExportContext $context, int $indentation): string
    {}

    private function canBeReflected(object $object): bool
    {}
}
