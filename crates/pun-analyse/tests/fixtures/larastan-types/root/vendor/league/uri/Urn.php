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

namespace League\Uri;

use BackedEnum;
use Closure;
use JsonSerializable;
use League\Uri\Contracts\Conditionable;
use League\Uri\Contracts\Transformable;
use League\Uri\Contracts\UriComponentInterface;
use League\Uri\Contracts\UriInterface;
use League\Uri\Exceptions\SyntaxError;
use League\Uri\UriTemplate\Template;
use Stringable;
use Uri\Rfc3986\Uri as Rfc3986Uri;
use Uri\WhatWg\Url as WhatWgUrl;

use function is_bool;
use function preg_match;
use function str_replace;
use function strtolower;

/**
 * @phpstan-type UrnSerialize array{0: array{urn: non-empty-string}, 1: array{}}
 * @phpstan-import-type InputComponentMap from UriString
 * @phpstan-type UrnMap array{
 *      scheme: 'urn',
 *      nid: string,
 *      nss: string,
 *      r_component: ?string,
 *      q_component: ?string,
 *      f_component: ?string,
 *  }
 */
final class Urn implements Conditionable, Stringable, JsonSerializable, Transformable
{
    /**
     * RFC8141 regular expression URN splitter.
     *
     * The regexp does not perform any look-ahead.
     * Not all invalid URN are caught. Some
     * post-regexp-validation checks
     * are mandatory.
     *
     * @link https://datatracker.ietf.org/doc/html/rfc8141#section-2
     *
     * @var string
     */
    private const REGEXP_URN_PARTS = '/^
        urn:
        (?<nid>[a-z0-9](?:[a-z0-9-]{0,30}[a-z0-9])?): # NID
        (?<nss>.*?)                                   # NSS
        (?<frc>\?\+(?<rcomponent>.*?))?               # r-component
        (?<fqc>\?\=(?<qcomponent>.*?))?               # q-component
        (?:\#(?<fcomponent>.*))?                      # f-component
    $/xi';

    /**
     * RFC8141 namespace identifier regular expression.
     *
     * @link https://datatracker.ietf.org/doc/html/rfc8141#section-2
     *
     * @var string
     */
    private const REGEX_NID_SEQUENCE = '/^[a-z0-9]([a-z0-9-]{0,30})[a-z0-9]$/xi';

    /** @var non-empty-string */
    private readonly string $uriString;
    /** @var non-empty-string */
    private readonly string $nid;
    /** @var non-empty-string */
    private readonly string $nss;
    /** @var non-empty-string|null */
    private readonly ?string $rComponent;
    /** @var non-empty-string|null */
    private readonly ?string $qComponent;
    /** @var non-empty-string|null */
    private readonly ?string $fComponent;

    /**
     * @param Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string $urn the percent-encoded URN
     */
    public static function parse(Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string $urn): ?Urn
    {}

    /**
     * @param Rfc3986Uri|WhatWgUrl|Stringable|string $urn the percent-encoded URN
     * @see self::fromString()
     *
     * @throws SyntaxError if the URN is invalid
     */
    public static function new(Rfc3986Uri|WhatWgUrl|Stringable|string $urn): self
    {}

    /**
     * @param Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string $urn the percent-encoded URN
     *
     * @throws SyntaxError if the URN is invalid
     */
    public static function fromString(Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string $urn): self
    {}

    /**
     * Create a new instance from a hash representation of the URI similar
     * to PHP parse_url function result.
     *
     * @param InputComponentMap $components a hash representation of the URI similar to PHP parse_url function result
     */
    public static function fromComponents(array $components = []): self
    {}

    /**
     * @param Stringable|string $nss the percent-encoded NSS
     *
     * @throws SyntaxError if the URN is invalid
     */
    public static function fromRfc2141(BackedEnum|Stringable|string $nid, BackedEnum|Stringable|string $nss): self
    {}

    /**
     * @param string $nss the percent-encoded NSS
     * @param ?string $rComponent the percent-encoded r-component
     * @param ?string $qComponent the percent-encoded q-component
     * @param ?string $fComponent the percent-encoded f-component
     *
     * @throws SyntaxError if one of the URN part is invalid
     */
    private function __construct(
        string $nid,
        string $nss,
        ?string $rComponent = null,
        ?string $qComponent = null,
        ?string $fComponent = null,
    ) {}

    /**
     * @return non-empty-string
     */
    private function setUriString(): string
    {}

    /**
     * Returns the NID.
     *
     * @return non-empty-string
     */
    public function getNid(): string
    {}

    /**
     * Returns the percent-encoded NSS.
     *
     * @return non-empty-string
     */
    public function getNss(): string
    {}

    /**
     * Returns the percent-encoded r-component string or null if it is not set.
     *
     * @return ?non-empty-string
     */
    public function getRComponent(): ?string
    {}

    /**
     * Returns the percent-encoded q-component string or null if it is not set.
     *
     * @return ?non-empty-string
     */
    public function getQComponent(): ?string
    {}

    /**
     * Returns the percent-encoded f-component string or null if it is not set.
     *
     * @return ?non-empty-string
     */
    public function getFComponent(): ?string
    {}

    /**
     * Returns the RFC8141 URN string representation.
     *
     * @return non-empty-string
     */
    public function toString(): string
    {}

    /**
     * Returns the RFC2141 URN string representation.
     *
     * @return non-empty-string
     */
    public function toRfc2141(): string
    {}

    /**
     * Returns the human-readable string representation of the URN as an IRI.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc3987
     */
    public function toDisplayString(): string
    {}

    /**
     * Returns the RFC8141 URN string representation.
     *
     * @see self::toString()
     *
     * @return non-empty-string
     */
    public function __toString(): string
    {}

    /**
     * Returns the RFC8141 URN string representation.
     * @see self::toString()
     *
     * @return non-empty-string
     */
    public function jsonSerialize(): string
    {}

    /**
     * Returns the RFC3986 representation of the current URN.
     *
     * If a template URI is used the following variables as present
     * {nid} for the namespace identifier
     * {nss} for the namespace specific string
     * {r_component} for the r-component without its delimiter
     * {q_component} for the q-component without its delimiter
     * {f_component} for the f-component without its delimiter
     */
    public function resolve(UriTemplate|Template|BackedEnum|string|null $template = null): UriInterface
    {}

    public function hasRComponent(): bool
    {}

    public function hasQComponent(): bool
    {}

    public function hasFComponent(): bool
    {}

    public function hasOptionalComponent(): bool
    {}

    /**
     * Return an instance with the specified NID.
     *
     * This method MUST retain the state of the current instance, and return
     * an instance that contains the specified NID.
     *
     * @throws SyntaxError for invalid component or transformations
     *                     that would result in an object in invalid state.
     */
    public function withNid(BackedEnum|Stringable|string $nid): self
    {}

    /**
     * Return an instance with the specified NSS.
     *
     * This method MUST retain the state of the current instance, and return
     * an instance that contains the specified NSS.
     *
     * @throws SyntaxError for invalid component or transformations
     *                     that would result in an object in invalid state.
     */
    public function withNss(BackedEnum|Stringable|string $nss): self
    {}

    /**
     * Return an instance with the specified r-component.
     *
     * This method MUST retain the state of the current instance, and return
     * an instance that contains the specified r-component.
     *
     * The component is removed if the value is null.
     *
     * @throws SyntaxError for invalid component or transformations
     *                     that would result in an object in invalid state.
     */
    public function withRComponent(BackedEnum|Stringable|string|null $component): self
    {}

    private static function formatComponent(?string $component): ?string
    {}

    /**
     * Return an instance with the specified q-component.
     *
     * This method MUST retain the state of the current instance, and return
     * an instance that contains the specified q-component.
     *
     * The component is removed if the value is null.
     *
     * @throws SyntaxError for invalid component or transformations
     *                     that would result in an object in invalid state.
     */
    public function withQComponent(BackedEnum|Stringable|string|null $component): self
    {}

    /**
     * Return an instance with the specified f-component.
     *
     * This method MUST retain the state of the current instance, and return
     * an instance that contains the specified f-component.
     *
     * The component is removed if the value is null.
     *
     * @throws SyntaxError for invalid component or transformations
     *                     that would result in an object in invalid state.
     */
    public function withFComponent(BackedEnum|Stringable|string|null $component): self
    {}

    public function normalize(): self
    {}

    public function equals(Urn|Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string $other, UrnComparisonMode $urnComparisonMode = UrnComparisonMode::ExcludeComponents): bool
    {}

    public function when(callable|bool $condition, callable $onSuccess, ?callable $onFail = null): static
    {}

    public function transform(callable $callback): static
    {}

    /**
     * @return UrnSerialize
     */
    public function __serialize(): array
    {}

    /**
     * @param UrnSerialize $data
     *
     * @throws SyntaxError
     */
    public function __unserialize(array $data): void
    {}

    /**
     * @return UrnMap
     */
    public function toComponents(): array
    {}

    /**
     * @return UrnMap
     */
    public function __debugInfo(): array
    {}
}
