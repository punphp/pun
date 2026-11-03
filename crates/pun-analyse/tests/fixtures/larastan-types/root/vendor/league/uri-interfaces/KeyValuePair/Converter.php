<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri\KeyValuePair;

use BackedEnum;
use League\Uri\Exceptions\SyntaxError;
use League\Uri\StringCoercionMode;
use Stringable;

use function array_combine;
use function explode;
use function implode;
use function is_string;
use function preg_match;
use function str_replace;

use const PHP_QUERY_RFC1738;
use const PHP_QUERY_RFC3986;

final class Converter
{
    private const REGEXP_INVALID_CHARS = '/[\x00-\x1f\x7f]/';

    /**
     * @param non-empty-string $separator the query string separator
     * @param array<string> $fromRfc3986 contains all the RFC3986 encoded characters to be converted
     * @param array<string> $toEncoding contains all the expected encoded characters
     */
    private function __construct(
        private readonly string $separator,
        private readonly array $fromRfc3986 = [],
        private readonly array $toEncoding = [],
    ) {}

    /**
     * @param non-empty-string $separator
     */
    public static function new(string $separator): self
    {}

    /**
     * @param non-empty-string $separator
     */
    public static function fromRFC3986(string $separator = '&'): self
    {}

    /**
     * @param non-empty-string $separator
     */
    public static function fromRFC1738(string $separator = '&'): self
    {}

    /**
     * @param non-empty-string $separator
     *
     * @see https://url.spec.whatwg.org/#application/x-www-form-urlencoded
     */
    public static function fromFormData(string $separator = '&'): self
    {}

    public static function fromEncodingType(int $encType): self
    {}

    /**
     * @return non-empty-string
     */
    public function separator(): string
    {}

    /**
     * @return array<string, string>
     */
    public function encodingMap(): array
    {}

    /**
     * @return list<non-empty-array<string|null>>
     */
    public function toPairs(BackedEnum|Stringable|string|int|float|bool|null $value): array
    {}

    /**
     * @param iterable<array{0:string|null, 1:BackedEnum|Stringable|string|bool|int|float|null}> $pairs
     */
    public function toValue(iterable $pairs): ?string
    {}

    /**
     * @param non-empty-string $separator
     */
    public function withSeparator(string $separator): self
    {}

    /**
     * Sets the conversion map.
     *
     * Each key from the iterable structure represents the RFC3986 encoded characters as string,
     * while each value represents the expected output encoded characters
     */
    public function withEncodingMap(iterable $encodingMap): self
    {}
}
