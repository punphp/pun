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

namespace League\Uri\Idna;

use ReflectionClass;
use ReflectionClassConstant;

/**
 * @see https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/uidna_8h.html
 */
final class Option
{
    private const DEFAULT                    = 0;
    private const ALLOW_UNASSIGNED           = 1;
    private const USE_STD3_RULES             = 2;
    private const CHECK_BIDI                 = 4;
    private const CHECK_CONTEXTJ             = 8;
    private const NONTRANSITIONAL_TO_ASCII   = 0x10;
    private const NONTRANSITIONAL_TO_UNICODE = 0x20;
    private const CHECK_CONTEXTO             = 0x40;

    private function __construct(private readonly int $value)
    {}

    private static function cases(): array
    {}

    public static function new(int $bytes = self::DEFAULT): self
    {}

    public static function forIDNA2008Ascii(): self
    {}

    public static function forIDNA2008Unicode(): self
    {}

    public function toBytes(): int
    {}

    /** array<string, int> */
    public function list(): array
    {}

    public function allowUnassigned(): self
    {}

    public function disallowUnassigned(): self
    {}

    public function useSTD3Rules(): self
    {}

    public function prohibitSTD3Rules(): self
    {}

    public function checkBidi(): self
    {}

    public function ignoreBidi(): self
    {}

    public function checkContextJ(): self
    {}

    public function ignoreContextJ(): self
    {}

    public function checkContextO(): self
    {}

    public function ignoreContextO(): self
    {}

    public function nonTransitionalToAscii(): self
    {}

    public function transitionalToAscii(): self
    {}

    public function nonTransitionalToUnicode(): self
    {}

    public function transitionalToUnicode(): self
    {}

    public function add(Option|int|null $option = null): self
    {}

    public function remove(Option|int|null $option = null): self
    {}
}
