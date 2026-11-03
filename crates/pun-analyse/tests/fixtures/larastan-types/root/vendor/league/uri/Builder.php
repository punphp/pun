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
use League\Uri\Contracts\Conditionable;
use League\Uri\Contracts\FragmentDirective;
use League\Uri\Contracts\Transformable;
use League\Uri\Contracts\UriComponentInterface;
use League\Uri\Exceptions\SyntaxError;
use SensitiveParameter;
use Stringable;
use Throwable;
use TypeError;
use Uri\Rfc3986\Uri as Rfc3986Uri;
use Uri\WhatWg\Url as WhatWgUrl;

use function is_bool;
use function str_replace;
use function strpos;

final class Builder implements Conditionable, Transformable
{
    private ?string $scheme = null;
    private ?string $username = null;
    private ?string $password = null;
    private ?string $host = null;
    private ?int $port = null;
    private ?string $path = null;
    private ?string $query = null;
    private ?string $fragment = null;

    public function __construct(
        BackedEnum|Stringable|string|null $scheme = null,
        BackedEnum|Stringable|string|null $username = null,
        #[SensitiveParameter] BackedEnum|Stringable|string|null $password = null,
        BackedEnum|Stringable|string|null $host = null,
        BackedEnum|int|null $port = null,
        BackedEnum|Stringable|string|null $path = null,
        BackedEnum|Stringable|string|null $query = null,
        BackedEnum|Stringable|string|null $fragment = null,
    ) {}

    /**
     * @throws SyntaxError
     */
    public function scheme(BackedEnum|Stringable|string|null $scheme): self
    {}

    /**
     * @throws SyntaxError
     */
    public function userInfo(
        BackedEnum|Stringable|string|null $user,
        #[SensitiveParameter] BackedEnum|Stringable|string|null $password = null
    ): static {}

    /**
     * @throws SyntaxError
     */
    public function host(BackedEnum|Stringable|string|null $host): self
    {}

    /**
     * @throws SyntaxError
     * @throws TypeError
     */
    public function port(BackedEnum|int|null $port): self
    {}

    /**
     * @throws SyntaxError
     */
    public function authority(BackedEnum|Stringable|string|null $authority): self
    {}

    /**
     * @throws SyntaxError
     */
    public function path(BackedEnum|Stringable|string|null $path): self
    {}

    /**
     * @throws SyntaxError
     */
    public function query(BackedEnum|Stringable|string|null $query): self
    {}

    /**
     * @throws SyntaxError
     */
    public function fragment(BackedEnum|Stringable|string|null $fragment): self
    {}

    /**
     * Puts back the Builder in a freshly created state.
     */
    public function reset(): self
    {}

    /**
     * Executes the given callback with the current instance
     * and returns the current instance.
     *
     * @param callable(self): self $callback
     */
    public function transform(callable $callback): static
    {}

    public function when(callable|bool $condition, callable $onSuccess, ?callable $onFail = null): static
    {}

    /**
     * @throws SyntaxError if the URI can not be build with the current Builder state
     */
    public function guard(Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUri = null): self
    {}

    /**
     * Tells whether the URI can be built with the current Builder state.
     */
    public function validate(Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUri = null): bool
    {}

    public function build(Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUri = null): Uri
    {}

    /**
     * @throws SyntaxError
     */
    private function buildAuthority(): ?string
    {}

    /**
     * @throws SyntaxError
     */
    private function buildPath(?string $authority): ?string
    {}

    /**
     * Filter a string.
     *
     * @throws SyntaxError if the submitted data cannot be converted to string
     */
    private function filterString(BackedEnum|Stringable|string|null $str): ?string
    {}
}
